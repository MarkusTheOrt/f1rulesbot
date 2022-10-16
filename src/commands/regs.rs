use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{
            command::CommandOptionType,
            interaction::autocomplete::AutocompleteInteraction,
        },
    },
};

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
    // let data = ctx.data.read().await;
    // let db = data.get::<DatabaseHandle>().expect("Database is
    // none").as_ref();

    // if let Ok(rows) = query!("select * from session").fetch_all(db).await {
    //     for (_, row) in rows.iter().enumerate() {
    //         println!("{:?}", row);
    //     }
    // }
    command
        .create_autocomplete_response(&ctx.http, |f| {
            f.add_string_choice("choice", "1245")
        })
        .await
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
