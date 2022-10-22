use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct SearchIndexHandle {}

impl TypeMapKey for SearchIndexHandle {
    type Value = Arc<RwLock<SearchIndex>>;
}

pub struct SearchIndex {
    pub paragraphs: Vec<IndexParagraph>,
    pub populated: bool,
}

#[derive(Clone, Debug)]
pub struct IndexParagraph {
    pub scale: i32,
    pub id: i32,
    pub number: String,
    pub tags: String,
    pub count: i32,
    pub name: String,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            paragraphs: vec![],
            populated: false,
        }
    }

    pub fn populate(&mut self) {
        self.populated = true;
    }

    pub fn flush(&mut self) {
        self.paragraphs.clear()
    }

    pub fn size(&self) -> usize {
        self.paragraphs.len()
    }

    pub fn search(
        &self,
        term: &str,
    ) -> Vec<IndexParagraph> {
        let term = term.to_uppercase();
        let mut v: Vec<IndexParagraph> = vec![];
        for (_, paragraph) in self.paragraphs.iter().enumerate() {
            if paragraph.name.starts_with(&term) {
                let mut p = paragraph.clone();
                p.scale = 100;
                v.push(p);
                continue;
            }
            let dist =
                lev_distance::lev_distance(&paragraph.name, &term) as i32;
            if dist < 15 {
                let mut p = paragraph.clone();
                p.scale = 100 / (dist + 1);
                v.push(p);
                continue;
            }
        }
        v.sort_unstable_by(|a, b| b.scale.cmp(&a.scale));
        v
    }

    pub fn add(
        &mut self,
        id: i32,
        number: String,
        tags: String,
        count: i32,
        name: String,
    ) {
        self.paragraphs.push(IndexParagraph {
            scale: 0,
            id,
            number,
            tags,
            count,
            name,
        })
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
