# Example protobuf communication

This repository contains a sample library that handles protobuf messages and a
couple of examples -- one for client and another for server -- illustrating the
application of the library. The library is not flexible enough for general
use. Its only purpose is to stage small-scale experiments and verify that
protobuf communication works correctly.

The definition of message exchanged by the example client and server is
contained in `proto/message.proto`. This definition is automatically compiled
into Rust code found in `src/proto/message.rs` when you run `cargo build` or
`cargo run`. This protobuf build hook is defined in `build.rs`.

To run the examples, execute `cargo run --example server` in one terminal, and
`cargo run --example client` in another. You should see client sending a message
to the server, and getting a modified echo response back.
