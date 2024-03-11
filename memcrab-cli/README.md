# memcrab-cli

Command line interface for memcrab.

## Install

From the main branch:
```sh
cargo install --git https://github.com/cospectrum/memcrab memcrab-cli
```

## Usage

### Server 
Start a server on TCP address `127.0.0.1:4949`
```bash
memcrab-cli server -a 127.0.0.1:4949 &
```

### Client 
Execute one and exit
```bash
memcrab-cli client -a 127.0.0.1:4949 set key value
```

Start interactive REPL. Press Ctrl-d to exit.
```bash
memcrab-cli client -a 127.0.0.1:4949
```
