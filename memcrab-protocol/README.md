# memcrab-protocol

This crate contains the implementation of the protocol that is used by server and the official Rust client.

**Please note that the protocol is not intended for direct use,
it serves as the basis for building client-server interaction.
Instead, you can use the official memcrab client.**

## Usage 
```rust
// TODO

use memcrab_protocol::{Socket, Msg, Request, Response};

#[tokio::main]
async fn main() {
}
```

## Protocol description

### Encoding

This is a binary protocol. The keys should be encoded as valid UTF-8 strings though.
All primitive types are encoded as `big-endian`.

### Messages 
TCP messages are not framed to distinct messages by themselves. Instead, we need to implement message borders ourselves.

Memcrab messages contain a header of a fixed length, and then payload of variable length.

The header length is `9` bytes.
The first byte of the header encodes the kind of message,
the remaining 8 bytes encode the payload length as u64 (also known as `PayloadLen`).

Message kinds are shared by all messages for client and server.
Clients should only send request messages and understand responses messages however, vice versa.

### Some type definitions

```rs
type PayloadLen = u64;  // number of bytes in payload

type Version = u16;     // protocol-version
type KeyLen = u64;      // number of bytes in the encoded utf8 string key
type Expirtaion = u32;  // expiration in seconds

type Value = Vec<u8>;   // just bytes
type Key = String;      // utf-8
```

### Mapping

#### Requests (first byte < 128)
| Message kind     | first byte | remaining 8 bytes in header | payload
| ---              | ---        | ---                         | --- 
|    Version       | 0          | PayloadLen                  | Version
|    Ping          | 1          | zeros                       | none
|    Get           | 2          | PayloadLen                  | Key
|    Set           | 3          | PayloadLen                  | KeyLen, Expirtaion, Key, Value
|    Delete        | 4          | PayloadLen                  | Key
|    Clear         | 5          | zeros                       | none

#### Responses (first byte >= 128)
| Message kind    | first byte | remaining 8 bytes in header | payload
| ---             | ---        | ---                         | --- 
|    Pong         | 128        | zeros                       | none
|    Ok           | 129        | zeros                       | none
|    Value        | 130        | PayloadLen                  | Value
|    KeyNotFound  | 131        | zeros                       | none
|    Error        | 255        | PayloadLen                  | String (utf-8 encoded)

### Versioning
Protocol is versioned by a number and are not backwards compatible.

The current version is `0`.

The clients must send `Version` message as their first message. The server must close the connection if the version is not compatible.
