use hyprain_core::ipc::HyprChannel;

fn main() {
    let channel = HyprChannel::new().expect("Error to connect with Hyprland socket.");
    channel.request("j/clients");
}
