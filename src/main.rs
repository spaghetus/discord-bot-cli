use serenity::{prelude::GatewayIntents, Client};
use structopt::StructOpt;
use tokio::io::{stdin, stdout, AsyncWriteExt};
use tokio::io::{AsyncBufReadExt, BufReader};

#[derive(StructOpt)]
struct Args {
	pub token: String,
	pub channel: u64,
}

#[tokio::main]
async fn main() {
	let args = Args::from_args();
	let mut client = Client::builder(
		args.token,
		GatewayIntents::default() | GatewayIntents::GUILD_MESSAGES,
	)
	.await
	.expect("Failed to build client");
	let channel = client
		.cache_and_http
		.http
		.get_channel(args.channel)
		.await
		.expect("Failed to get channel.");
	async fn prompt() {
		let mut out = stdout();
		out.write_all(b"> ").await.expect("Broken pipe");
		out.flush().await.expect("Broken pipe");
	}
	prompt().await;
	let mut stdin = BufReader::new(stdin()).lines();
	while let Ok(Some(line)) = stdin.next_line().await {
		channel
			.id()
			.send_message(&*client.cache_and_http.http, |msg| msg.content(line))
			.await
			.expect("Failed to send");
		prompt().await;
	}
}
