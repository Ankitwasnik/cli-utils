use clap::Parser;

/// Echo the message
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Echo {
    /// Echo the text message
    text: String
}

fn main() {
    
    let args = Echo::parse();
    println!("{}", args.text);
}
