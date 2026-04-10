use std::env::VarError;
use tokio::net::UnixSocket;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader as TokioBufReader};
use tokio::net::UnixStream;
use tokio::sync::broadcast;

/// A env of the current Hyprland instance.
const HYPRLAND_INSTANCE_SIGNATURE: &str = "HYPRLAND_INSTANCE_SIGNATURE";

/// A env of the runtime directory.
const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";

/// Get the current Hyprland instance signature to use in socket path.
fn get_hyprland_instance_signature() -> Result<String, VarError> {
    std::env::var(HYPRLAND_INSTANCE_SIGNATURE)
        .inspect(|path| log::debug!("The Hyprland Instance Signature is: {path}"))
        .inspect_err(|err| {
            log::error!("Error to get the Hyprland instance signature! Are you using Hyprland? Error: {err}")
        })
}

/// Get the current Hyprland command socket path.
fn get_command_socket_path() -> Result<String, VarError> {
    let signature = get_hyprland_instance_signature()?;

    let xdg_path = std::env::var(XDG_RUNTIME_DIR)
        .inspect(|path| log::debug!("The XDG Runtime Path: {path}"))
        .inspect_err(|err| log::error!("Error to get the XDG Runtime Path! Error: {err}"))?;

    let full_path = format!("{}/hypr/{}/.socket.sock", xdg_path, signature);

    Ok(full_path)
}

/// Get the current Hyprland event socket path.
fn get_event_socket_path() -> Result<String, VarError> {
    let signature = get_hyprland_instance_signature()?;

    let xdg_path = std::env::var(XDG_RUNTIME_DIR)
        .inspect(|path| log::debug!("The XDG Runtime Path: {path}"))
        .inspect_err(|err| log::error!("Error to get the XDG Runtime Path! Error: {err}"))?;

    let full_path = format!("{}/hypr/{}/.socket2.sock", xdg_path, signature);

    Ok(full_path)
}

/// Dispatches synchronous commands to the Hyprland IPC.
pub struct CommandDispatcher {
    /// Hyprland command socket path.
    socket_path: String,
}

impl CommandDispatcher {
    /// Constructor for the CommandDispatcher.
    pub fn new() -> Result<Self, VarError> {
        match get_command_socket_path() {
            Ok(path) => Ok(Self { socket_path: path }),
            Err(err) => Err(err),
        }
    }

    /// Send a command to Hyprland.
    ///
    /// Send a request to deliver the `msg` to the Hyprland and receive
    /// a response or a None if a error occurs to stablish a connection.
    pub async fn request(&self, msg: &str) -> tokio::io::Result<String> {
        let socket = UnixSocket::new_stream()
            .inspect_err(|err| log::error!("Error to create a socket. Error: {}", err))?;

        let mut stream = socket
            .connect(&self.socket_path)
            .await
            .inspect_err(|err| log::error!("Error to create a stream. Error: {}", err))?;

        stream.write_all(msg.as_bytes()).await.inspect_err(|err| {
            log::error!("Error to dispatch the command: '{}'. Error: {}", msg, err)
        })?;

        let mut response = String::new();
        stream
            .read_to_string(&mut response)
            .await
            .inspect_err(|err| log::error!("Error to read the System response. Error: {}", err))?;

        Ok(response)
    }
}

/// Listens to the Hyprland event socket and broadcasts messages.
pub struct EventListener {
    /// Hyprland socket path to connect.
    socket_path: String,
    /// A sender to dispatch the event to all receivers (subscribers).
    sender: broadcast::Sender<String>,
}

impl EventListener {
    /// Constructor for to EventListener.
    pub fn new() -> Result<Self, VarError> {
        match get_event_socket_path() {
            Ok(path) => {
                let (sender, _) = broadcast::channel(100);
                Ok(Self {
                    socket_path: path,
                    sender: sender,
                })
            }
            Err(err) => Err(err),
        }
    }

    /// Get a subscriber to receive events.
    pub fn listen(&self) -> broadcast::Receiver<String> {
        self.sender.subscribe()
    }

    /// Observe the Hyprland events and send them when occurs.
    pub async fn observe(&mut self) -> tokio::io::Result<()> {
        let stream = UnixStream::connect(&self.socket_path).await?;

        let mut reader = TokioBufReader::new(stream);
        let mut buffer = String::new();

        log::debug!("Listening to Hyprland events.");

        loop {
            buffer.clear();

            let bytes = reader.read_line(&mut buffer).await?;

            if bytes == 0 {
                log::warn!("Hyprland connection finished!");
                break;
            }

            let msg = buffer.trim().to_string();
            log::debug!("{msg}");

            match self.sender.send(msg) {
                Ok(_) => log::debug!("Catch a Hyprland event."),
                Err(_) => log::warn!("No subscribers to listen the Hyprland event."),
            }
        }

        Ok(())
    }
}
