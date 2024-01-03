use std::{fs, path::Path};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default = "proto/grpc.proto".to_string();
    let relative_path = args.get(1).unwrap_or(&default);

    let curr_dir = std::env::current_dir().unwrap();

    let proto_path = &curr_dir.join(relative_path);
    let proto_dir = &proto_path.parent().unwrap();
    dbg!(proto_path);
    dbg!(proto_dir);

    let base = tonic_build::configure().build_client(false).build_server(false);
    let client = base
        .clone()
        .build_client(true)
        .out_dir(create_dir(curr_dir.join("client")));
    let server = base
        .clone()
        .build_server(true)
        .out_dir(create_dir(curr_dir.join("server")));

    client.compile(&[proto_path], &[proto_dir]).unwrap();
    server.compile(&[proto_path], &[proto_dir]).unwrap();
}

fn create_dir<P: AsRef<Path>>(path: P) -> P {
    fs::create_dir_all(path.as_ref()).unwrap();
    path
}
