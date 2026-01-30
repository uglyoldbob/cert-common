//! Struct definitions used in the api

/// Defines the list of applets that can be worked with
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AppletList {
    /// The list of valid applet ids
    pub applet_ids: Vec<i64>,
}

/// Defines the list of calls that an applet can handle
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct AppletCalls {
    /// The list of valid applet calls
    pub calls: Vec<String>,
}