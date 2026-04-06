use chrono::{DateTime, Utc};
use gpui::SharedString;

pub struct Item {
    pub id: uuid::Uuid,
    pub name: SharedString,
    pub content: SharedString,
    pub kind: ItemKind,
    pub tags: Vec<SharedString>,
    pub language: Option<SharedString>,
    pub create_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

pub enum ItemKind {
    Note,
    Snippet,
    Code,
}

pub struct Link {
    from: uuid::Uuid,
    to: uuid::Uuid,
}
