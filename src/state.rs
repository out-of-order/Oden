use uuid::Uuid;

pub struct AppState {
    pub mode: AppMode,
    pub selected_id: Option<Uuid>,
}

impl AppState {
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

pub enum AppMode {
    Search,
    List,
    Graph,
    Settings,
}
