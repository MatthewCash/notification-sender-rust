use clap::Parser;
use serde_derive::Serialize;
use std::env;
use std::result::Result;

#[derive(Parser, Debug, Serialize)]
#[clap(author, version, about, long_about = None)]
#[allow(non_snake_case)]
struct Notification {
    #[clap(short, long)]
    title: String,

    #[clap(short, long)]
    desc: Option<String>,

    #[clap(short, long)]
    imageUrl: Option<String>,

    #[clap(short, long, parse(from_flag))]
    ping: bool,
}

fn send_notification(
    notification: &Notification,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let http_url =
        env::var("HTTP_PATH").unwrap_or_else(|_| "http://127.0.0.1:8723/notification".into());

    let client = reqwest::blocking::Client::new();

    client.post(http_url).json(notification).send()
}

fn main() {
    let notification = Notification::parse();

    send_notification(&notification).expect("An error occurred sending notification!");
}
