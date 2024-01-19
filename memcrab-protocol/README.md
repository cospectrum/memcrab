# memcrab-protocol

This crate contains the implementation of the protocol that is used by server and the official Rust client.

**Note that this is not meant to be the official Rust API for memcrab, use the official wrapper client instead.**

## Usage 
```rust
TODO

use memcrab_protocol::Socket;
use memcrab_protocol::{Message, Request, Response};

#[tokio::main]
async fn main() {
}
```

## Protocol description

### Encoding

This is a binary protocol. The keys should be encoded as valid UTF-8 strings though.

### Messages 
TCP messages are not framed to distinct messages by themselves. Instead, we need to implement message borders ourselves.

Memcrab messages contain a header of a fixed length, and then payload of variable length.

The first byte of the header encodes the kind of message,
the remaining 8 bytes encode the payload length as u64 (also known as `PayloadLen`).

The header length is `9` bytes.

Message kinds are shared by all messages for client and server.
Clients should only send request messages and understand responses messages however, vice versa.

### Some type definitions
```rs
type PayloadLen = u64;  // number of bytes in payload

type Version = u16;     // protocol-version
type KeyLen = u64;      // number of bytes in the encoded utf8 string key
type Expirtaion = u32;  // expiration in seconds
```

### Mapping
| Message kind            | first byte | remaining 8 bytes in header | payload
| ---                     | ---        | ---                         | --- 
|    VersionRequest       | 0          | PayloadLen                  | Version
|    PingRequest          | 1          | zeros                       | none
|    GetRequest           | 2          | PayloadLen                  | key
|    SetRequest           | 3          | PayloadLen                  | KeyLen, Expirtaion, key, value
|    DeleteRequest        | 4          | PayloadLen                  | key
|    ClearRequest         | 5          | zeros                       | none
|    PongResponse         | 128        | zeros                       | none
|    OkResponse           | 129        | zeros                       | none
|    ValueResponse        | 130        | PayloadLen                  | value
|    KeyNotFoundResponse  | 131        | zeros                       | none
|    ErrorResponse        | 255        | PayloadLen                  | value (utf8 encoded)

### Versioning
Protocol is versioned by a number and are not backwards compatible.

The current version is `0`.

The clients must send `Version` message as their first message. The server must close the connection if the version is not compatible.
