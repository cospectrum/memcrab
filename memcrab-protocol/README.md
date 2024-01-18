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


The first byte of the header encodes the kind of message. The rest of the header encodes the information about the lengths of the payload or other metainfo.


Message kinds are shared by all messages for client and server. Clients should only send request messages and understand responses messages however, vice versa.


| Message kind            | first byte | rest of the header |  payload
| ---                     | ---        | ---                |  --- 
|    VersionRequest       | 0          | version            |  none
|    PingRequest          | 1          | none               |  none
|    GetRequest           | 2          | klen               |  key
|    SetRequest           | 3          | klen, vlen, exp    |  key, value
|    DeleteRequest        | 4          | klen               |  key
|    ClearRequest         | 5          | none               |  none

|    PongResponse         | 128        | none               |  none
|    OkResponse           | 129        | none               |  none
|    ValueResponse        | 130        | vlen               |  value
|    KeyNotFoundResponse  | 131        | none               |  none
|    ErrorResponse        | 132        | vlen               |  value


The lengths of fields for klen, vlen, version, etc are as follows:


| header field | size (bytes) |
| ---          | ---          |
| klen         | 8            |
| vlen         | 8            |
| version      | 2            |
| exp          | 4            |

The header length is 21 bytes.


### Versioning
Protocol is versioned by a number and are not backwards compatible.

The current version is `0`.

The clients must send `Version` message as their first message. The server must close the connection if the version is not compatible.


