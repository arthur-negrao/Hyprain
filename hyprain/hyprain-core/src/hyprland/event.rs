use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseEventError;

/// A event emitted by the Hyprland like: workspace change, focus change, notification, ...
/// Each event has a `event_name` and a `event_data` provided by Hyprland.
pub enum HyprEvent {
    Workspace(String),
    ActiveWindow(String),
    OpenWindow(String),
    CloseWindow(String),
    Fullscreen(String),
    Minimized(String),
    Ignored,
    Unknown { name: String, data: Option<String> },
}

impl FromStr for HyprEvent {
    type Err = ParseEventError;

    fn from_str(raw_event: &str) -> Result<Self, Self::Err> {
        let clean_event = raw_event.trim();

        if clean_event.is_empty() {
            return Err(ParseEventError);
        }

        let (name, data) = match clean_event.split_once(">>") {
            Some((n, d)) => (n, Some(d)),
            None => (clean_event, None),
        };

        // parse the hyprland events. See: https://wiki.hypr.land/IPC/#events-list
        let event = match (name, data) {
            ("workspacev2", Some(d)) => Self::Workspace(d.to_string()),
            ("activewindowv2", Some(d)) => Self::ActiveWindow(d.to_string()),
            ("openwindow", Some(d)) => Self::OpenWindow(d.to_string()),
            ("closewindow", Some(address)) => Self::CloseWindow(address.to_string()),
            ("minimized", Some(d)) => Self::Fullscreen(d.to_string()),
            ("fullscreen", Some(d)) => Self::Fullscreen(d.to_string()),
            ("workspace" | "activewindow", _) => Self::Ignored, // duplicated events
            (n, d) => Self::Unknown {
                name: n.to_string(),
                data: d.map(String::from),
            },
        };

        Ok(event)
    }
}
