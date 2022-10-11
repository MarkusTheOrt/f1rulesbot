use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::command::CommandOptionType,
};

pub struct Regulation {
    pub number: u32,
    pub name: String,
    pub search_tags: String,
    pub db_index: u32,
}

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

pub fn simple_index() -> Vec<Regulation> {
    vec![
        Regulation {
            number: 1,
            name: "Regulation".to_string(),
            search_tags: "".to_string(),
            db_index: 13,
        },
        Regulation {
            number: 2,
            name: "General undertaking".to_string(),
            search_tags: "".to_string(),
            db_index: 14,
        },
        Regulation {
            number: 3,
            name: "General conditions".to_string(),
            search_tags: "".to_string(),
            db_index: 15,
        },
        Regulation {
            number: 4,
            name: "Licenses".to_string(),
            search_tags: "".to_string(),
            db_index: 16,
        },
        Regulation {
            number: 5,
            name: "Championship competitions".to_string(),
            search_tags: "".to_string(),
            db_index: 17,
        },
        Regulation {
            number: 6,
            name: "World Championship".to_string(),
            search_tags: "Title, WDC, WCC, Driver/s, Constructor/s, Point/s, Distance, Reduced".to_string(),
            db_index: 18,
        },
        Regulation {
            number: 7,
            name: "Dead Heat".to_string(),
            search_tags: "Tie, Breaker, Tie Breaker".to_string(),
            db_index: 19,
        },
        Regulation {
            number: 8,
            name: "Competitors Applications".to_string(),
            search_tags: "".to_string(),
            db_index: 20,
        },
    ]
}
