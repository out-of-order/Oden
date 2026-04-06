use gpui::{Entity, ParentElement, Render, Styled, div};

use crate::state::AppState;

#[allow(dead_code)]
pub struct AppRoot {
    pub app_state: Entity<AppState>,
}

impl AppRoot {
    pub fn new(app_state: Entity<AppState>) -> Self {
        Self { app_state }
    }
}

impl Render for AppRoot {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex()
            .flex_row()
            .h_full()
            .w_full()
            .items_center()
            .justify_center()
            .child("Hello World")
    }
}
