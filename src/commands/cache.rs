use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::command::CommandOptionType, Permissions,
    },
};
use sqlx::query;

use crate::search_index::get_index;

use super::get_database;

pub fn register(
    command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
    command
        .create_option(|option| {
            option
                .add_string_choice("flush", "flush")
                .add_string_choice("repopulate", "repopulate")
                .name("method")
                .required(true)
                .kind(CommandOptionType::String)
                .description("Which method to call on the cache.")
        })
        .dm_permission(false)
        .default_member_permissions(Permissions::ADMINISTRATOR)
        .name("cache")
        .description("Various Cache methods.")
}

pub async fn execute(
    ctx: &serenity::client::Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    if let Err(why) = command.defer(&ctx.http).await {
        println!("Err deferring a response in cache {}", why)
    }
    let db = get_database(&ctx).await.as_ref();
    let index = get_index(ctx).await;
    let t = index.as_ref().read().await;
    let e = query!("SELECT * FROM headings");
    
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|response| {
                response.content("Hello!");
            })
        }).await
        
}
