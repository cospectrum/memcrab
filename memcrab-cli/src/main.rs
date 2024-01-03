use clap::Parser;
use memcrab::pb::{cache_rpc_client::CacheRpcClient, GetRequest, SetRequest};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: String,

    #[arg(short, long, default_value = "9090")]
    port: String,

    #[arg(short, long, action)]
    server: bool,
}

// async fn eval_line(client: &mut CacheRpcClient, line: String) -> anyhow::Result<()> {
//     Ok(())
// }

// struct REPLSyntaxError;

async fn eval_lines(addr: String) -> anyhow::Result<()> {
    let mut client = CacheRpcClient::connect(addr).await.unwrap();
    let mut readline = DefaultEditor::new()?;
    loop {
        let readline = readline.readline("memcrab> ");
        match readline {
            Ok(line) => {
                let tokens = line.split_whitespace().collect::<Vec<_>>();
                if tokens[0] == "get" {
                    let msg = GetRequest {
                        key: tokens[1].to_owned(),
                    };
                    let req = tonic::Request::new(msg);
                    let resp = client.get(req).await?.into_inner();
                    match resp.value {
                        Some(val) => println!("value: {:?}", val),
                        None => println!("no value set"),
                    }
                } else if tokens[0] == "set" {
                    let msg = SetRequest {
                        key: tokens[1].to_owned(),
                        value: tokens[2..]
                            .iter()
                            .map(|&s| s.parse().unwrap())
                            .collect::<Vec<u8>>(),
                    };
                    let req = tonic::Request::new(msg);
                    client.set(req).await?;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Quit");
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let dst_addr = format!("http://{}{}", cli.port, cli.host);

    eval_lines(dst_addr).await?;
    Ok(())
}
