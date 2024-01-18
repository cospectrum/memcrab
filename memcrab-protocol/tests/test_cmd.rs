use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, Receiver, Sender};

use memcrab_protocol::io::{AsyncReader, AsyncWriter};
use memcrab_protocol::{ClientSocket, ErrorResponse, Request, Response, ServerSocket};

#[tokio::test]
async fn test_commands() {
    let addr = "127.0.0.1:9008".parse().unwrap();
    let (echo_sender, echo_receiver) = mpsc::channel::<Request>(1);

    let cache = |req: Request| -> Response {
        match req {
            Request::Ping => Response::Pong,
            Request::Clear => Response::Ok,
            Request::Version(_) => {
                let inner = ErrorResponse::Internal("..".to_owned());
                Response::Error(inner)
            }
            Request::Get(_) => Response::Value(vec![1, 2]),
            Request::Delete(_) => Response::KeyNotFound,
            Request::Set { .. } => Response::Ok,
        }
    };

    tokio::spawn(start_server_socket(addr, echo_sender, cache));

    let stream = connect(addr).await;
    let socket = ClientSocket::new(stream);
    let mut client = TestClient::new(socket, echo_receiver);

    let resp = client.make_request(Request::Ping).await;
    assert_eq!(resp, Response::Pong);

    let resp = client.make_request(Request::Delete("abc".to_owned())).await;
    assert_eq!(resp, Response::KeyNotFound);

    client.make_request(Request::Version(1)).await;
    client.make_request(Request::Clear).await;
    client.make_request(Request::Get("123".to_owned())).await;
    client
        .make_request(Request::Set {
            key: "alex".to_owned(),
            value: vec![11, 2],
            expiration: 2,
        })
        .await;
}

async fn connect(addr: SocketAddr) -> TcpStream {
    for _ in 0..30 {
        match TcpStream::connect(addr).await {
            Ok(stream) => return stream,
            Err(_) => {
                tokio::time::sleep(Duration::from_secs_f32(0.1)).await;
            }
        }
    }
    unreachable!("could not connect to {:?}", addr)
}

async fn start_server_socket(
    addr: SocketAddr,
    echo_channel: Sender<Request>,
    mut cache: impl FnMut(Request) -> Response,
) {
    let listener = TcpListener::bind(addr).await.unwrap();

    let (stream, _) = listener.accept().await.unwrap();
    let mut server = ServerSocket::new(stream);
    loop {
        let req = server.recv_request().await.unwrap();
        let resp = cache(req.clone());
        server.send_response(&resp).await.unwrap();
        echo_channel.send(req.clone()).await.unwrap();
    }
}

struct TestClient<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    socket: ClientSocket<S>,
    echo_channel: Receiver<Request>,
}

impl<S> TestClient<S>
where
    S: AsyncReader + AsyncWriter + Send,
{
    fn new(socket: ClientSocket<S>, echo_channel: Receiver<Request>) -> Self {
        Self {
            socket,
            echo_channel,
        }
    }
    async fn make_request(&mut self, request: Request) -> Response {
        let resp = self.socket.make_request(request.clone()).await.unwrap();
        let echo = self.echo_channel.recv().await.unwrap();
        assert_eq!(echo, request);
        resp
    }
}
