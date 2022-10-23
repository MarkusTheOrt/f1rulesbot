use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{
            command::CommandOptionType,
            interaction::{
                application_command::CommandDataOptionValue,
                autocomplete::AutocompleteInteraction,
            },
        },
    },
};

use crate::search_index::get_index;

use super::get_database;

pub fn register(
    command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
    command.name("regs").description("Returns the regulations").create_option(
        |option| {
            option
                .name("regulation")
                .description("The part of the regulations")
                .set_autocomplete(true)
                .kind(CommandOptionType::String)
                .required(true)
            //.default_option(true)
        },
    )
}

pub async fn autocomplete(
    ctx: &serenity::client::Context,
    command: &AutocompleteInteraction,
) -> Result<(), serenity::Error> {
    let index = get_index(ctx).await;
    let index = index.as_ref().read().await;
    if let Some(CommandDataOptionValue::String(term)) =
        &command.data.options.first().expect("Autocomplete invalid").resolved
    {
        command
            .create_autocomplete_response(&ctx.http, |response| {
                for (i, paragraph) in index.search(term).iter().enumerate() {
                    if i > 23 {
                        break;
                    };
                    response.add_string_choice(
                        paragraph.name.clone(),
                        paragraph.number.clone(),
                    );
                }
                response
            })
            .await
    } else {
        command
            .create_autocomplete_response(&ctx.http, |response| response)
            .await
    }
}

pub async fn execute(
    ctx: &serenity::client::Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    if let Some(CommandDataOptionValue::String(str)) = &command
        .data
        .options
        .first()
        .expect("required elem not found.")
        .resolved
    {
        let number =
            *str.split('.').collect::<Vec<&str>>().first().expect("NUMBAH");

        let db = get_database(ctx).await;
        let first =
            sqlx::query!("SELECT text FROM headings WHERE number = $1", number)
                .fetch_one(db.as_ref())
                .await;

        let data =
            sqlx::query!("SELECT * FROM headings WHERE number = $1", str)
                .fetch_one(db.as_ref())
                .await;

        let mut tlr = None;
        if let Ok(fr) = first {
            tlr = Some(fr.text);
        }

        if let Ok(row) = data {
            return command
                .create_interaction_response(&ctx.http, |response| {
                    response.interaction_response_data(|response| {
                        let mut content = "".to_string();
                        if let Some(tlr) = tlr {
                            content = tlr
                        }
                        response
                            .ephemeral(true)
                            .content(format!("{}{}", content, row.text))
                    })
                })
                .await;
        }

        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|response| {
                    response
                        .ephemeral(true)
                        .content("Did not find specified regulation.")
                })
            })
            .await
    } else {
        command
            .create_interaction_response(&ctx.http, |response| {
                response.interaction_response_data(|response| {
                    response.ephemeral(true).content("Invalid option supplied.")
                })
            })
            .await
    }
}
