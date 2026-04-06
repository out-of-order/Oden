use gpui::{App, AppContext, Application, Entity, WindowOptions};
use gpui_component::Root;

use crate::{root::AppRoot, state::AppState, store::ItemStore};

mod models;
mod root;
mod state;
mod store;
mod views;

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        ItemStore::init(cx);
        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let app_state: Entity<AppState> = cx.new(|_| AppState::init());
                let view = cx.new(|_| AppRoot::new(app_state));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .unwrap();
        })
        .detach();
    });
}
