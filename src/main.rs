use std::io;
//use slotmachine::spin;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use tokio;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Telegram,
    receive_selection {
        rec_selection: String,
    },
}

#[tokio::main]
async fn main() {

    println!("Welcome to our Crypto Slot Machine!!!");
    let bot = Bot::from_env(); // import telegram bot token from server environment variable
    //println!("the bot token is {}", bot.token());
    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Telegram].endpoint(telegram))
            .branch(dptree::case![State::receive_selection { rec_selection }].endpoint(receive_selection)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

async fn test() { println!("First Task"); }

async fn telegram(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    //pretty_env_logger::init();
    //log::info!("Starting throw dice bot...");
    bot.send_message(msg.chat.id, "Type what you would like to do next? [spin] for Spin, [deposit] for instruction how to deposit, [withdraw] for instructions to cash out?").await?;
    match msg.text() {
        Some("spin") => {
            let username = msg.chat.username().unwrap();
            //println!("testing telegram bot {}", username);
            
            dialogue.update(State::receive_selection { rec_selection: "spin".into() });
            println!("Let's spin one more time"); 
            let newspin = slotmachine::spin();
            let mut output = String::new();
            match newspin.1.as_str() {
                "JJJ" => {
                    output = format!("{} - CONGRATULATIONS {}, you won {}X your initial bet!!!", newspin.1, username, newspin.0);
                },
                "777" => { 
                    output = format!("{} - CONGRATULATIONS {}, you won {}X your initial bet!!!", newspin.1, username, newspin.0);
                },
                "333" => {
                    output = format!("{} - CONGRATULATIONS {}, you won {}X your initial bet!!!", newspin.1, username, newspin.0);
                },
                "222" => {
                    output = format!("{} - CONGRATULATIONS {}, you won {}X your initial bet!!!", newspin.1, username, newspin.0);
                },
                "111" => {
                    output = format!("{} - CONGRATULATIONS {}, you won {}X your initial bet!!!", newspin.1, username, newspin.0);
                },
                "CCC" => {
                    output = format!("{} - CONGRATULATIONS {}, you won {}X your initial bet!!!", newspin.1, username, newspin.0);
                },
                _ => {
                    output = format!("{} - {}, unfortunately you did not win!", newspin.1, username);
                }
            }
            
            println!("{}", output);
            bot.send_message(msg.chat.id, output).await?;
            //let t1 = test(); 
            //let t2 = test(); 
            //tokio::join!(t1,t2);
        },
        Some("deposit") => { println!("In order to spin, you have to transfer more ETH into your wallet"); },
        Some("withdraw") => { println!("In order to withdraw, follow these instructions XXX") },
        Some(text) => { println!("did not match anyting of the available choices"); },
        None => {
            println!("I do not think this is working");
            bot.send_message(msg.chat.id, "Send me plain text").await?;
        }
    }
    Ok(())
}

async fn receive_selection(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    println!("starting the receive_selection function");
    match msg.text() {
        Some(text) => {
            println!("will the receive function work?");
            bot.send_message(msg.chat.id, "tell me your name").await?;
            dialogue.update(State::receive_selection { rec_selection: text.into() }).await?;
        }
        None => {
            println!("nope, the receive function is not working");
            bot.send_message(msg.chat.id, "Send me plain text").await?;
        }
    }
    Ok(())
}