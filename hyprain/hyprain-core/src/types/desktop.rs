use serde::{Deserialize, Serialize};
use zvariant::Type;

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct WindowData {
    pub name: String,
    pub class: String,
    pub address: String,
    pub workspace: u32,
}

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct WorkspaceData {
    pub id: u32,
    pub name: String,
}
