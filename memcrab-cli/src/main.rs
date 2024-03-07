use std::{net::SocketAddr, io};
use std::net::Ipv4Addr;
use anyhow::anyhow;
use clap::{Args, Parser, Subcommand};
use memcrab::{RawClient, connections::Tcp, Error};

#[derive(Parser)]
#[command(author, version, about = "command line interface for memcrab (server and REPL)", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Server { 
        // #[arg(short = 'H', long, default_value = "127.0.0.1")]
        // host: Ipv4Addr,
        //
        // #[arg(short, long, default_value = "9090")]
        // port: u16,

        // TODO: PathBuf for UNIX Socket
        #[arg(short, long, default_value = "localhost:9090")]
        address: SocketAddr,
    },
    Client {
        // #[arg(short = 'H', long, default_value = "127.0.0.1")]
        // host: Ipv4Addr,
        //
        // #[arg(short, long, default_value = "9090")]
        // port: u16,

        #[arg(short, long, default_value = "localhost:9090")]
        address: SocketAddr,

        #[clap(num_args = 1.., value_delimiter = ' ', help = "execute command and exit")]
        cmd: Vec<String>,
    }
}

// async fn eval_line<C>(&mut client: RawClient<C>, line: String) -> io::Result<String> {
// }
//

async fn serve_memcrab(addr: SocketAddr) -> anyhow::Result<()> {
    todo!();
}


async fn repl<C: memcrab::Rpc>(mut client: RawClient<C>) -> anyhow::Result<()> {
    let mut editor = rustyline::DefaultEditor::new()?;

    for readline in editor.iter("📝🦀  ") {
        match readline {
            Ok(line) => { 
                let err = eval_line(&mut client, line.split_whitespace()/*.map(|s| s.to_string())*/.collect()).await;
                println!("error: {:?}", err);
            },
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("quit");
                break;
            }
            Err(e) => println!("error: {}", e),
        }
    }
    Ok(())
}

async fn eval_line<C: memcrab::Rpc>(client: &mut RawClient<C>, line: Vec<&str>) -> anyhow::Result<String> {
    match line[..] {
        ["get", string_key] => {
            // TODO:
            // "quoted key with spaces"
            let value = client.get(string_key).await?;
            match value {
                Some(value) => {
                    let string_value = String::from_utf8(value).unwrap();
                    Ok(string_value)
                }
                None => {
                    Ok(String::from("key not found"))
                }
            }
        }
        _ => Err(anyhow!("Syntax error"))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Client { address, cmd } => {
            let mut client = RawClient::<Tcp>::connect(address).await?;
            if cmd.is_empty() {
                repl(client).await?;
            } else {
                eval_line(&mut client, cmd.iter().map(|s| s.as_str()).collect()).await?;
            }
        }
        Commands::Server { address } => {
            serve_memcrab(address).await?;
        }
    }

    Ok(())
}
