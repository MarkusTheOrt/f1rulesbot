use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{
            command::CommandOptionType,
            Activity,
        },
        Permissions,
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
    // Defer interaction response as ephemeral

    if let Err(why) = command.defer_ephemeral(&ctx.http).await {
        println!("Err deferring a response: \"{}\"", why)
    }
    let db = get_database(ctx).await;
    let index = get_index(ctx).await;
    let mut t = index.as_ref().write().await;
    t.flush();
    t.populate();
    let e = query!("SELECT * FROM headings")
        .fetch_all(db.as_ref())
        .await
        .expect("DB Query was null you fool.");

    for (_, itm) in e.into_iter().enumerate() {
        t.add(itm.id, itm.number, itm.tags, itm.count, itm.name)
    }

    ctx.set_activity(Activity::watching(format!("Cache Size: {}", t.size())))
        .await;

    let _ = command
        .create_followup_message(&ctx.http, |response| {
            response.ephemeral(true).content("Cache flushed and repopulated!")
        })
        .await;
    Ok(())
}
