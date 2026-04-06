use uuid::Uuid;

#[allow(dead_code)]
pub struct AppState {
    pub mode: AppMode,
    pub selected_id: Option<Uuid>,
}

impl AppState {
    #[allow(dead_code)]
    pub fn new(mode: AppMode, selected_id: Option<Uuid>) -> Self {
        Self { mode, selected_id }
    }

    pub fn init() -> Self {
        Self {
            mode: AppMode::List,
            selected_id: None,
        }
    }
}

#[allow(dead_code)]
pub enum AppMode {
    Search,
    List,
    Graph,
    Settings,
}
