mod utils;

use std::time::Duration;

use memcrab_protocol::io::{AsyncReader, AsyncWriter};
use memcrab_protocol::{ClientSocket, Request, Response, ServerSocket};
use tokio::sync::mpsc::{self, Receiver, Sender};

use utils::*;

#[tokio::test]
async fn test_commands() {
    let stream = MockStream::new();
    let (sender, receiver) = mpsc::channel::<Request>(1);
    tokio::spawn(start_server(stream.clone(), sender));

    let socket = ClientSocket::new(stream.clone());
    let mut client = TestClient::new(socket, receiver);

    let resp = client.make_request(Request::Ping).await;
    assert_eq!(resp, Response::Pong);
}

async fn start_server(stream: MockStream, sender: Sender<Request>) {
    let mut server = ServerSocket::new(stream.clone());
    loop {
        while stream.is_empty() {
            tokio::time::sleep(Duration::from_secs_f32(0.1)).await;
        }
        let req = server.recv_request().await.unwrap();
        sender.send(req.clone()).await.unwrap();

        let resp = match req {
            Request::Ping => Response::Pong,
            Request::Clear => Response::Ok,
            Request::Version(_) => Response::Ok,
            Request::Get(_) => Response::Value(vec![1, 2]),
            Request::Delete(_) => Response::KeyNotFound,
            Request::Set { .. } => Response::Ok,
        };
        server.send_response(&resp).await.unwrap();
    }
}

struct TestClient<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    socket: ClientSocket<S>,
    channel: Receiver<Request>,
}

impl<S> TestClient<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    fn new(socket: ClientSocket<S>, channel: Receiver<Request>) -> Self {
        Self { socket, channel }
    }
    async fn make_request(&mut self, request: Request) -> Response {
        let resp = self.socket.make_request(request.clone()).await.unwrap();
        let echo = self.channel.recv().await.unwrap();
        assert_eq!(echo, request);
        resp
    }
}
