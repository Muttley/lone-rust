use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use rand::prelude::*;

#[command]
pub async fn luck(ctx: &Context, msg: &Message) -> CommandResult {
	let flip: u8 = rand::thread_rng().gen_range(0..=1);

	let result;

	if flip == 0 {
		result = String::from("You were Unlucky");
	}
	else {
		result = String::from("You were Lucky");
	}

	msg.channel_id.say(&ctx.http, result).await?;

	Ok(())
}
