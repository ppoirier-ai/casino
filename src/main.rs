use std::io;
use slotmachine::spin;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};
use tokio;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Telegram,
    Receive_selection {
        rec_selection: String,
    },
}

#[tokio::main]
async fn main() {

    println!("Welcome to our Crypto Slot Machine!!!");
    let bot = Bot::from_env(); // import telegram bot token from server environment variable

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Telegram].endpoint(telegram))
            .branch(dptree::case![State::Receive_selection { rec_selection }].endpoint(receive_selection)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;

    loop {
        println!("Type what you would like to do next? [1] for Spin, [2] to stop, [3] to cash out, [4] to deposit");

        let mut selection = String::new();

        io::stdin()
        .read_line(&mut selection)
        .expect("Please input a single number");

        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You chose: {selection}");

        match selection {
            1 => { slotmachine::spin(); let t1 = test(); let t2 = test(); tokio::join!(t1,t2); },
            3 => { println!("You selected to cash out"); },
            4 => { println!("You selected to deposit"); },
            2 => { println!("Good bye and see you soon!"); break; },
            _ => { println!("Pick a number between 1 and 4"); }
        }
    }
}

async fn test() { println!("First Task"); }

async fn telegram(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    //pretty_env_logger::init();
    //log::info!("Starting throw dice bot...");

    match msg.text() {
        Some(text) => {
            println!("testing telegram bot");
            bot.send_message(msg.chat.id, "What is your selection?").await?;
            dialogue.update(State::Receive_selection { rec_selection: text.into() }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text").await?;
        }
    }
    Ok(())
}

async fn receive_selection(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "tell me your name").await?;
            //dialogue.update(State::Receive_selection { rec_selection: text.into() }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text").await?;
        }
    }
    Ok(())
}