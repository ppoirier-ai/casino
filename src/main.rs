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
            1 => { println!("testing selection 1"); slotmachine::spin(); let t1 = test(); let t2 = test(); tokio::join!(t1,t2); },
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
    println!("starting the telegram function");
    bot.send_message(msg.chat.id, "What is your selection?").await?;
    match msg.text() {
        Some("1") => {
            println!("testing telegram bot {}", msg.chat.id.to_string());
            
            dialogue.update(State::receive_selection { rec_selection: "1".into() });
            println!("finishing telegram function with");
        },
        Some("2") => { println!("you selected option 2"); },
        Some(text) => { println!("did not match anyting specific"); },
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