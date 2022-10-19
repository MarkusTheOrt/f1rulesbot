use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct SearchIndexHandle {}

impl TypeMapKey for SearchIndexHandle {
    type Value = Arc<RwLock<SearchIndex>>;
}

pub struct SearchIndex {
    paragraphs: Vec<IndexParagraph>,
}

pub struct IndexParagraph {
    id: u32,
    number: String,
    tags: String,
    count: u32,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            paragraphs: vec![],
        }
    }
}

#[allow(dead_code)]
pub async fn get_index(
    ctx: &serenity::client::Context
) -> Arc<RwLock<SearchIndex>> {
    let data = ctx.data.read().await;
    data.get::<SearchIndexHandle>()
        .expect("Search index is empty you fuck.")
        .clone()
}
