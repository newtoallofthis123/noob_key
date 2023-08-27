use clap::Parser;
use dotenv::dotenv;

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
    match cmd {
        _ => todo!("Command not found")
    }
}
