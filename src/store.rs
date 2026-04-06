use std::collections::HashMap;

use gpui::{App, Global};
use gpui_component::link::Link;

use crate::models::Item;

pub struct ItemStore {
    items: HashMap<uuid::Uuid, Item>,
    links: Vec<Link>,
}

impl Global for ItemStore {}

impl ItemStore {
    pub fn init(cx: &mut App) {
        cx.set_global(ItemStore {
            items: HashMap::new(),
            links: Vec::new(),
        });
    }

    pub fn get(cx: &mut App) -> &Self {
        cx.global::<ItemStore>()
    }

    pub fn get_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<ItemStore>()
    }
}
