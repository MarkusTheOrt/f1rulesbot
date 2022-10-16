use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub async fn execute(
    ctx: &serenity::client::Context,
    command: &ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    command
        .create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|response| {
                response.ephemeral(true).content("**Error: Not Implemented!**")
            })
        })
        .await
}
