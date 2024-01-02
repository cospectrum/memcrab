use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'H', long)]
    host: Option<String>,

    #[arg(short, long)]
    port: Option<String>,

    #[arg(short, long, action)]
    interactive: bool,
}

fn eval_line(line: String) {
    println!("you typed: {}", line);
}

fn read_lines() -> Result<()> {
    let mut readline = DefaultEditor::new()?;
    loop {
        let readline = readline.readline("memcrab> ");
        match readline {
            Ok(line) => {
                eval_line(line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Some(host) = cli.host.as_deref() {
        println!("Value for host: {}", host);
    }
    if let Some(port) = cli.port.as_deref() {
        println!("Value for port: {}", port);
    }
    if cli.interactive {
        read_lines();
    }
}
