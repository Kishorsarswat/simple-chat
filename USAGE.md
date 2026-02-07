# Usage
## To run the server:

`cargo run --bin chat-server`
## To run a client:

```bash
cargo run --bin chat-client -- <username> [host] [port]`
#Example
cargo run --bin chat-client -- Alice
```

## Commands:
- send <message>: Send a message.
- leave: Disconnect and exit.

## Rules
- Can't use used username.
- Messages are broadcasted to all users.
- Server will automatically close the connection if the client crashes.