use std::{
    env::VarError,
    io::{Read, Write},
    os::unix::net::UnixStream,
};

const HYPRLAND_INSTANCE_SIGNATURE: &str = "HYPRLAND_INSTANCE_SIGNATURE";
const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";

fn get_hyprland_instance_signature() -> Result<String, VarError> {
    std::env::var(HYPRLAND_INSTANCE_SIGNATURE)
        .inspect(|path| println!("The Hyprland Instance Signature is: {path}"))
        .inspect_err(|err| eprintln!("Error to get the Hyprland instance signature! Error: {err}"))
}

fn get_hyprland_write_socket() -> Result<String, VarError> {
    let signature = get_hyprland_instance_signature()?;

    let xdg_path = std::env::var(XDG_RUNTIME_DIR)
        .inspect(|path| println!("The XDG Runtime Path: {path}"))
        .inspect_err(|err| eprintln!("Error to get the XDG Runtime Path! Error: {err}"))?;

    let full_path = format!("{}/hypr/{}/.socket.sock", xdg_path, signature);

    Ok(full_path)
}

pub struct HyprChannel {
    write_socket_path: String,
}

impl HyprChannel {
    pub fn new() -> Option<Self> {
        let path = get_hyprland_write_socket().ok()?;

        Some(Self {
            write_socket_path: path,
        })
    }

    pub fn request(&self, msg: &str) -> Option<String> {
        let mut stream = UnixStream::connect(&self.write_socket_path).ok()?;

        stream.write_all(msg.as_bytes()).ok()?;

        let mut response = String::new();
        stream.read_to_string(&mut response).ok()?;

        println!("{}", response);
        Some(response)
    }
}
