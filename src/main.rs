use std::collections::HashMap;
use serenity::client::Client;
use serenity::framework::standard::{
    StandardFramework
};
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    http::Http,
    prelude::*,
};
use dotenv::dotenv;
use std::env;
use serenity::builder::CreateActionRow;
use serenity::model::channel::{Message};
use serenity::model::interactions::message_component::MessageComponentInteraction;

mod commands;
use commands::{
    help,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == help::HELP_COMMAND {
            help::help(&ctx, &msg).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command
                        .name("review")
                        .description("Manage reviews of all kinds")
                        .create_option(|option| {
                            option
                                .name("view")
                                .description("View reviews")
                                .kind(ApplicationCommandOptionType::String)
                                .add_string_choice("Anime", "anime")
                                .add_string_choice("All", "all")
                        })
                        .create_option(|option| {
                            option
                                .name("create")
                                .description("Create review")
                                .kind(ApplicationCommandOptionType::String)
                                .add_string_choice("Anime", "anime")
                        })
                })
        })
            .await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".to_string(),
                "id" => {
                    let options = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected user option")
                        .resolved
                        .as_ref()
                        .expect("Expected user object");

                    if let ApplicationCommandInteractionDataOptionValue::User(user, _member) =
                    options
                    {
                        format!("{}'s id is {}", user.tag(), user.id)
                    } else {
                        "Please provide a valid user".to_string()
                    }
                },
                "review" => {
                    match interaction.message_component() {
                        None => {}
                        Some(interaction) => {
                            let action_row = CreateActionRow(/* HashMap */);
                            interaction.create_interaction_response(&ctx.http, |response| {
                                response
                                    .kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|message| {
                                        message
                                            .components(|component| {
                                                component
                                                    .add_action_row(action_row)
                                            })
                                    })
                            })
                        }
                    }
                }
                _ => "Not implemented!".to_string()
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new();

    // Login with a bot token from the environment
    let token = env::var("BOT_TOKEN").expect("parsing token");
    let app_id: u64 = env::var("APP_ID").expect("parsing application id").parse().unwrap();

    let http = Http::new_with_token(&token);

    let mut client = Client::builder(token)
        .application_id(app_id)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
