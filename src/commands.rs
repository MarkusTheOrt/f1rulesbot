use std::sync::Arc;

use sqlx::{
    Pool,
    Postgres,
};

use crate::DatabaseHandle;

pub mod not_implemented;
pub mod ping;
pub mod regs;

#[allow(dead_code)]
pub async fn get_database(
    ctx: &serenity::client::Context
) -> Arc<Pool<Postgres>> {
    let data = ctx.data.read().await;
    data.get::<DatabaseHandle>().expect("Database Handle not found.").clone()
}
