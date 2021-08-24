use std::{env, sync::Arc};

use futures::StreamExt;
use simplelog::*;
use telegram_bot::*;
use std::fs::File;
use log::{info, warn};

use crate::translate::check_language;

mod translate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Warn, Config::default(), File::create("translate.log").unwrap()),
        ]
    ).unwrap();
    dotenv::dotenv().ok();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let translo = Arc::new(env::var("TRANSLO_API_KEY").expect("TRANSLO_API_KEY not set"));
    let api = Api::new(token);

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = match update {
            Ok(update) => update,
            Err(_) => continue,
        };
        let api_key = Arc::clone(&translo);
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                let text = match check_language(data, &api_key).await {
                    Ok(message) => {
                        message
                    }
                    Err(_) => {
                        continue;
                    }
                };
                
                match api.send(message.text_reply(format!(
                    "{}",
                    text
                )))
                .await {
                    Ok(_) => continue,
                    Err(err) => warn!("{}", err),
                }
            }
        }
    }
    Ok(())
}
