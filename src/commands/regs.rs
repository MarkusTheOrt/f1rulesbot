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
    // let db = get_database(&ctx).await.as_ref();

    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|response| {
                response
                    .ephemeral(true)
                    .content("This command is still a work in progress.")
            })
        })
        .await
}
