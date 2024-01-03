# memcrab-cli

Command line interface for memcrab

# Usage

Start a server
```bash
cargo run -- -H 127.0.0.1 -p 6969 -s &
```

Connect client to a running server
```bash
cargo run -- -H 127.0.0.1 -p 6969
```

This will start an interactive session. You can use `get` and `set` commands.

```
memcrab> set x 1 2 3 
memcrab> get x
x: [1, 2, 3]
memcrab> 
```
