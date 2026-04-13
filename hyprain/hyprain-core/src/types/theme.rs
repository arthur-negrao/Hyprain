use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

/// Color data of the current theme.
#[derive(Serialize, Deserialize, Type, Debug)]
pub struct ThemeColor {
    pub foreground: String,
    pub background: String,
    pub cursor: String,
}

/// Geometry and effects data of the current theme.
#[derive(Serialize, Deserialize, Type, Debug)]
pub struct ThemeGeometry {
    pub border: u32,
    pub shadows: u32,
    pub blur: bool,
}

/// A specify theme event to share with all application ecosystem.
/// This data structure has a `ThemeColor` like `color` and a `ThemeGeometry`
/// like `geometry` to share with others applications.
#[derive(Serialize, Deserialize, Type, Debug)]
pub struct SysTheme {
    pub color: ThemeColor,
    pub geometry: ThemeGeometry,
}

impl SysTheme {
    /// Create a new ThemeEvent by a `theme_color` and a `theme_geometry`.
    ///
    /// # Errors
    ///
    /// * This function return a None if a error occurs.
    pub fn new(theme_color: ThemeColor, theme_geometry: ThemeGeometry) -> Option<Self> {
        Some(Self {
            color: theme_color,
            geometry: theme_geometry,
        })
    }
}
