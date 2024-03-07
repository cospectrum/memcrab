use anyhow::anyhow;
use clap::{Parser, Subcommand};
use memcrab::{connections::Tcp, RawClient};
use std::net::SocketAddr;

#[derive(Parser)]
#[command(author, version, about = "command line interface for memcrab (server and client)", long_about = None)]
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

        // TODO: Take either SocketAddr for TCP or PathBuf for UNIX Socket
        #[arg(short, long, default_value = "127.0.0.1:9090")]
        address: SocketAddr,
    },
    Client {
        // #[arg(short = 'H', long, default_value = "127.0.0.1")]
        // host: Ipv4Addr,
        //
        // #[arg(short, long, default_value = "9090")]
        // port: u16,
        #[arg(short, long, default_value = "127.0.0.1:9090")]
        address: SocketAddr,
        // #[clap(trailing_var_arg = true, allow_hyphen_values = true, help = "execute command and exit")]
        // cmd: Vec<String>,
    },
}

fn tokenize_line(line: String) -> anyhow::Result<Vec<String>> {
    let mut tokens = Vec::new();
    let mut word = String::new();
    let mut quote = false;

    for c in line.chars() {
        match c {
            '"' if quote => {
                tokens.push(word.clone());
                word.clear();
                quote = false;
            }
            '"' => {
                quote = true;
            }
            ' ' if quote => {
                word.push(c);
            }
            ' ' => {
                if !word.is_empty() {
                    tokens.push(word.clone());
                    word.clear();
                }
            }
            c => {
                word.push(c);
            }
        }
    }
    if quote {
        return Err(anyhow!("syntax error"));
    }

    if !word.is_empty() {
        tokens.push(word);
    }

    Ok(tokens)
}

async fn eval_line<C: memcrab::Rpc>(
    client: &mut RawClient<C>,
    line: String,
) -> anyhow::Result<String> {
    let tokens = tokenize_line(line)?;
    match tokens.first().map(|s| s.as_ref()) {
        Some("get") if tokens.len() == 2 => {
            let value = client.get(&tokens[1]).await?;
            match value {
                Some(value) => {
                    let string_value = String::from_utf8(value).unwrap();
                    Ok(string_value)
                }
                None => Ok(String::from("key not found")),
            }
        }
        Some("set") if tokens.len() == 3 => {
            client.set(&tokens[1], tokens[2].to_owned().into()).await?;
            Ok(String::from("ok"))
        }
        _ => Err(anyhow!("syntax error")),
    }
}

async fn serve_memcrab(addr: SocketAddr) -> anyhow::Result<()> {
    use memcrab_server::{serve, Cache};
    use tokio::net::TcpListener;

    let gb = 2_usize.pow(30);
    let cache = Cache::builder()
        .segments(10)
        .max_bytesize(gb)
        .build()
        .into();

    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, cache).await?;
    Ok(())
}

async fn repl<C: memcrab::Rpc>(mut client: RawClient<C>) -> anyhow::Result<()> {
    let mut editor = rustyline::DefaultEditor::new()?;

    for readline in editor.iter("📝🦀  ") {
        match readline {
            Ok(line) => match eval_line(&mut client, line).await {
                Ok(message) => println!("{}", message),
                Err(err) => println!("error: {}", err),
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Client { address } => {
            let client = RawClient::<Tcp>::connect(address).await?;
            repl(client).await?;
        }
        Commands::Server { address } => {
            serve_memcrab(address).await?;
        }
    }

    Ok(())
}
