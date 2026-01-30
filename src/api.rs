//! Struct definitions used in the api

/// Defines the list of applets that can be worked with
#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub struct AppletList {
    /// The list of valid applet ids
    applet_ids: Vec<i64>,
}
