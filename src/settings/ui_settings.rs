#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UiSettings {
    pub scale_factor: f32,
    pub show_toolbar: bool,
    pub show_status_bar: bool,
    pub show_inspector: bool,
    pub show_hierarchy: bool,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            scale_factor: 1.0,
            show_toolbar: true,
            show_status_bar: true,
            show_inspector: true,
            show_hierarchy: true,
        }
    }
}
