use azalea::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let account = Account::offline("MahmutBot");

    println!("Bot başlıyor...");

    ClientBuilder::new()
        .set_handler(|bot, event| async move {
            match event {

                azalea::Event::Chat(msg) => {
                    let text = msg.to_string();

                    // HELLO
                    if text == "!hello" {
                        bot.chat("Selam ben botum".to_string());
                    }

                    // FOLLOW
                    if text.starts_with("!follow ") {
                        let target = text.replace("!follow ", "");
                        bot.chat(format!("{} takip ediliyor", target));

                        let bot_clone = bot.clone();

                        tokio::spawn(async move {
                            loop {
                                if let Some(player) = bot_clone.world().players().get(&target) {
                                    let pos = player.position();
                                    let _ = bot_clone.goto(pos).await;
                                }

                                sleep(Duration::from_secs(1)).await;
                            }
                        });
                    }

                    // STOP
                    if text == "!stop" {
                        bot.chat("Duruyorum".to_string());
                        bot.stop_pathfinding();
                    }
                }

                _ => {}
            }
        })
        .start(
            Account::offline("MahmutBot"),
            "7Hsnf.aternos.me:25565"
        )
        .await;
}
