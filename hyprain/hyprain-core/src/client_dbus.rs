use zbus::proxy;

use crate::types::{
    desktop::{WindowData, WorkspaceData},
    theme::SysTheme,
};

#[proxy(
    default_service = "org.hyprain.Daemon",
    default_path = "/org/hyprain/EventListener",
    interface = "org.hyprain.EventListener"
)]
trait EventListener {
    #[zbus(signal)]
    fn theme_changed(&self, theme: SysTheme) -> zbus::Result<()>;

    #[zbus(signal)]
    fn workspace_changed(&self, current_workspace: WorkspaceData) -> zbus::Result<()>;

    #[zbus(signal)]
    fn window_fucused_changed(&self, current_window: WindowData) -> zbus::Result<()>;

    fn get_current_theme(&self, theme: SysTheme) -> zbus::Result<()>;
    fn get_active_workspace(&self, workspace: WorkspaceData) -> zbus::Result<()>;
    fn get_focused_window(&self, window: WindowData) -> zbus::Result<()>;
}
