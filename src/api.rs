//! Struct definitions used in the api

/// Defines the list of applets that can be worked with
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AppletList {
    /// The list of valid applet ids
    pub applet_ids: Vec<i64>,
}
