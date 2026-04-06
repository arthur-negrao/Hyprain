use std::env;

use hyprain_core::ipc::{CommandDispatcher, EventListener};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    log::debug!("Init debug");
    let channel = CommandDispatcher::new().expect("Error to connect with Hyprland socket.");
    channel.request("j/clients");

    log::info!("Init observer");
    let mut hypr_listener = EventListener::new().expect("Can not connect with Hyprland socket 2.");
    let mut subscriber = hypr_listener.listen();

    // daemon task
    tokio::spawn(async move {
        if let Err(e) = hypr_listener.observe().await {
            log::error!("Hyprland observer crashed: {}", e);
        }
    });

    // subscriber task
    tokio::spawn(async move {
        loop {
            if let Ok(msg) = subscriber.recv().await {
                log::debug!("[Subscriber] Received message: {}", msg);
            }
        }
    });

    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            println!("\nCtrl+C signal received! Closing the daemon...");
        }
        Err(err) => {
            eprintln!("It was not possible listen the system signal: {}", err);
        }
    };
}
