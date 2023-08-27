use clap::Parser;
use dotenv::dotenv;
use human_panic::setup_panic;

#[derive(Parser, Debug)]
#[command(name="NoobKey", author="Ishan Joshi", version, about="A Simple Key Value Store", long_about = None)]

//? The Args struct is used to parse the command line arguments
struct Args {
    #[arg(required=false)]
    cmd: Option<String>,

    #[arg(short, long)]
    custom: Option<String>,

    #[arg(short, long)]
    docs: bool,
}

mod db;
mod utils;
mod handler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_panic!();
    let logo = r#"
 _   _             _     _  __          
| \ | | ___   ___ | |__ | |/ /___ _   _ 
|  \| |/ _ \ / _ \| '_ \| ' // _ \ | | |
| |\  | (_) | (_) | |_) | . \  __/ |_| |
|_| \_|\___/ \___/|_.__/|_|\_\___|\__, |
                                  |___/ 
"#;
    bunt::println!("{$green}{}{/$}", logo);
    let args = Args::parse();

    let cmd:String;
    if args.cmd.is_some(){
        cmd = args.cmd.unwrap();
    }
    else{
        cmd = inquire::Text::new("Enter Command: ").with_help_message("Enter a valid command").with_autocomplete(
            &utils::suggester,
        ).prompt().unwrap();
    }
    match cmd.as_str() {
        "set" => handler::add().await,
        "list" => handler::list().await,
        "delete" => handler::delete().await,
        "get" => handler::get().await,
        "search" => handler::search().await,
        "exit" => {
            bunt::println!("{$red}Exiting...{/$}");
            std::process::exit(0);
        }
        "help" => todo!("Help command not implemented"),
        _ => todo!("Command not found")
    }
}
