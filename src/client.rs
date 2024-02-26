use poise::serenity_prelude as serenity;
use crate::config::*;

pub fn client_builder() -> serenity::ClientBuilder {
  let intents = serenity::GatewayIntents::non_privileged();
  let token = DISCORD_TOKEN.clone();
  
  serenity::ClientBuilder::new(token, intents)
}