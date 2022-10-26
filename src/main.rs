mod commands;
mod search_index;

use dotenvy::dotenv;

use search_index::{
    SearchIndex,
    SearchIndexHandle,
};
use serenity::{
    async_trait,
    framework::standard::{
        macros::group,
        StandardFramework,
    },
    http::Http,
    model::{
        application::{
            command::Command,
            interaction::Interaction,
        },
        prelude::*,
        user::OnlineStatus,
    },
    prelude::*,
};
use sqlx::query;

use std::{
    collections::HashSet,
    env,
    sync::Arc,
};

use crate::{
    commands::get_database,
    search_index::get_index,
};

/// Store the Database connection in the client. This is Send + Sync since we
/// store the database in an Arc (Atomically Reference Counted)
struct DatabaseHandle {}

impl TypeMapKey for DatabaseHandle {
    type Value = Arc<sqlx::postgres::PgPool>;
}

#[group]
struct General;

struct Bot {}

#[async_trait]
impl EventHandler for Bot {
    async fn ready(
        &self,
        ctx: Context,
        _ready: Ready,
    ) {
        println!("Connected!");
        ctx.set_presence(
            Some(Activity::watching("Something")),
            OnlineStatus::Online,
        )
        .await;

        let _ =
            Command::create_global_application_command(&ctx.http, |command| {
                commands::cache::register(command)
            })
            .await;
        let _global =
            Command::create_global_application_command(&ctx.http, |command| {
                commands::ping::register(command)
            })
            .await;
        if let Err(why) =
            Command::create_global_application_command(&ctx.http, |command| {
                commands::regs::register(command)
            })
            .await
        {
            println!("Error registering Regs command. {}", why)
        }
    }

    async fn cache_ready(
        &self,
        ctx: Context,
        _guilds: Vec<GuildId>,
    ) {
        println!("Cache built and populated.");

        let db = get_database(&ctx).await;
        let index = get_index(&ctx).await;
        let mut index = index.write().await;

        let data = query!("SELECT * FROM headings")
            .fetch_all(db.as_ref())
            .await
            .expect("Database returned null");
        for (_, paragraph) in data.iter().enumerate() {
            index.add(
                paragraph.id,
                paragraph.number.clone(),
                paragraph.tags.clone(),
                paragraph.count,
                paragraph.name.clone(),
            )
        }
        ctx.set_activity(Activity::listening(format!(
            "{} regulations",
            index.size()
        )))
        .await;
    }
    
    async fn interaction_create(
        &self,
        ctx: Context,
        interaction: Interaction,
    ) {
        if let Interaction::Autocomplete(command) = interaction {
            if let Err(why) = match command.data.name.as_str() {
                "regs" => commands::regs::autocomplete(&ctx, &command).await,
                _ => Err(SerenityError::Other(
                    "Tried to autocomplete not-implemented command.",
                )),
            } {
                println!("Couldn't send a autocomplete response. {}", why);
            }
            return;
        }
        if let Interaction::ApplicationCommand(command) = interaction {
            if let Err(why) = match command.data.name.as_str() {
                "ping" => commands::ping::execute(&ctx, &command).await,
                "regs" => commands::regs::execute(&ctx, &command).await,
                "cache" => commands::cache::execute(&ctx, &command).await,
                _ => commands::not_implemented::execute(&ctx, &command).await,
            } {
                println!("Couldn't send a command response. {}", why);
            }
            return;
        }
    }

    async fn resume(
        &self,
        _: Context,
        _: ResumedEvent,
    ) {
    }
}

#[tokio::main]
async fn main() {
    if let Err(why) = dotenv() {
        println!("Couldn't read Dotenv... Skipping. Err: {}", why);
    }
    let token = env::var("DISCORD_TOKEN").expect("Missing Discord token.");
    let database_url = env::var("DATABASE_URL").expect("Missing Database url.");
    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Couldn't access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILDS;
    let mut client = Client::builder(token, intents)
        .event_handler(Bot {})
        .framework(framework)
        .await
        .expect("Error creating Client");

    let database = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .expect("Couldn't Connect to Database");

    {
        let mut data = client.data.write().await;
        data.insert::<DatabaseHandle>(Arc::new(database));
        data.insert::<SearchIndexHandle>(Arc::new(RwLock::new(
            SearchIndex::new(),
        )))
    }

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Couldn't register <ctrl><C> handler");
    });

    if let Err(why) = client.start().await {
        println!("Client Error: {:?}", why);
    }
}
