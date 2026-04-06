#![allow(dead_code)]

use gpui::{Entity, IntoElement, ParentElement, Render, Styled, div, px};
use gpui_component::{
    ActiveTheme, Icon,
    button::{Button, ButtonVariants},
};

use crate::{
    icons::IconName,
    state::{AppMode, AppState},
};

pub struct AppRoot {
    pub app_state: Entity<AppState>,
}

impl AppRoot {
    pub fn new(app_state: Entity<AppState>) -> Self {
        Self { app_state }
    }

    fn nav_button(
        &self,
        cx: &mut gpui::Context<Self>,
        icon: IconName,
        mode: AppMode,
        tooltip: &'static str,
    ) -> Button {
        Button::new(tooltip)
            .icon(Icon::new(icon))
            .ghost()
            .on_click(cx.listener(move |this, _event, _window, cx| {
                this.app_state.update(cx, |state, cx| {
                    state.mode = mode;
                    cx.notify();
                });
            }))
            .tooltip(tooltip)
    }

    fn render_sidebar(&self, cx: &mut gpui::Context<Self>) -> impl IntoElement + use<> {
        let border_color = cx.theme().border;
        let sidebar_bg = cx.theme().sidebar;

        let icon_rail = vec![
            self.nav_button(cx, IconName::List, AppMode::List, "List Mode"),
            self.nav_button(cx, IconName::Search, AppMode::Search, "Search Mode"),
            self.nav_button(cx, IconName::Graph, AppMode::Graph, "Graph Mode"),
        ];

        div()
            .relative()
            .h_full()
            .w(px(56.0))
            .flex_shrink_0()
            .flex_col()
            .border_color(border_color)
            .bg(sidebar_bg)
            .border_r(px(1.0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_4()
                    .p_2()
                    .children(icon_rail),
            )
            .child(
                div()
                    .absolute()
                    .bottom_0()
                    .w_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p_2()
                    .child(
                        Button::new("settings")
                            .ghost()
                            .tooltip("Settings")
                            .icon(Icon::new(IconName::Settings)),
                    ),
            )
    }
}

impl Render for AppRoot {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let bg = cx.theme().background;
        let sidebar = self.render_sidebar(cx);
        let mode = self.app_state.read(cx).mode.to_string();

        div()
            .flex()
            .flex_row()
            .h_full()
            .w_full()
            .bg(bg)
            .child(sidebar)
            .child(div().p_4().child(mode))
    }
}
