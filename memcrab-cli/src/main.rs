use clap::Parser;
use core::num::NonZeroUsize;
use memcrab::pb::{cache_rpc_client::CacheRpcClient, GetRequest, SetRequest};
use memcrab_cache::Cache;
use memcrab_server::start_grpc_server;
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

async fn eval_lines(addr: String) -> anyhow::Result<()> {
    let mut client = CacheRpcClient::connect(addr).await.unwrap();
    let mut editor = DefaultEditor::new()?;
    loop {
        let line = editor.readline("memcrab> ");
        match line {
            Ok(line) => {
                eval_line(&mut client, line)
                    .await
                    .unwrap_or_else(|e| println!("error: {:?}", e));
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-c");
            }
            Err(ReadlineError::Eof) => {
                println!("quit");
                break;
            }
            Err(err) => {
                println!("error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

async fn eval_line(
    client: &mut CacheRpcClient<tonic::transport::Channel>,
    line: String,
) -> anyhow::Result<()> {
    let tokens = line.split_whitespace().collect::<Vec<_>>();
    if tokens.is_empty() {
        return Ok(());
    }
    if tokens[0] == "get" {
        if tokens.len() != 2 {
            anyhow::bail!("syntax error: expected one key after `get`");
        }
        let msg = GetRequest {
            key: tokens[1].to_owned(),
        };
        let req = tonic::Request::new(msg);
        let resp = client.get(req).await?.into_inner();
        match resp.value {
            Some(val) => println!("{}: {:?}", tokens[1], val),
            None => println!("no value set"),
        }
    } else if tokens[0] == "set" {
        if tokens.len() < 3 {
            anyhow::bail!("syntax error: expected one key and bytes after `set`");
        }
        let msg = SetRequest {
            key: tokens[1].to_owned(),
            value: tokens[2..]
                .iter()
                .map(|&s| s.parse().unwrap())
                .collect::<Vec<u8>>(),
        };
        let req = tonic::Request::new(msg);
        client.set(req).await?;
    } else {
        anyhow::bail!("syntax error: unexpected token {}", tokens[0]);
    }
    Ok(())
}

async fn serve(addr: String) -> anyhow::Result<()> {
    let maxbytes = 100_000;
    let maxlen = NonZeroUsize::new(110).unwrap();
    let cache = Cache::new(maxlen, maxbytes);

    start_grpc_server(addr.parse()?, cache).await.unwrap();
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let addr = format!("{}:{}", cli.host, cli.port);

    if cli.server {
        serve(addr).await?;
    } else {
        eval_lines(format!("http://{}", addr)).await?;
    }
    Ok(())
}
