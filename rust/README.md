# P4Runtime Rust Crates

This directory contains Rust crates for the P4Runtime API specification.

## Usage in Your Project

Add the following to your `Cargo.toml`:

**For message types only:**
```toml
[dependencies]
p4runtime-prost = { git = "https://github.com/p4lang/p4runtime.git" }
```

**For gRPC services:**
```toml
[dependencies]
p4runtime-prost = { git = "https://github.com/p4lang/p4runtime.git" }
p4runtime-tonic = { git = "https://github.com/p4lang/p4runtime.git" }
```

## Crates

### `p4runtime-prost`

Provides Protocol Buffer message types using [prost](https://docs.rs/prost/).
This crate contains only the message definitions and no gRPC service
implementations.

**Use this crate when:**
- You need to construct/serialize/deserialize P4Runtime messages
- You don't need gRPC client/server functionality
- You want to minimize dependencies (only depends on `prost`)

**Example:**
```rust
use p4runtime_prost::p4::v1::WriteRequest;

let request = WriteRequest {
    device_id: 1,
    // ... other fields
};
```

### `p4runtime-tonic`

Provides gRPC service definitions using [tonic](https://docs.rs/tonic/).
This crate depends on `p4runtime-prost` for message types and adds gRPC client
and server implementations.

**Use this crate when:**
- You need to implement a P4Runtime server
- You need to create a P4Runtime client

**Example:**
```rust
use p4runtime_tonic::p4::v1::p4_runtime_client::P4RuntimeClient;

let mut client = P4RuntimeClient::connect("http://[::1]:50051").await?;
```

## Design

1. **Automatic Code Generation**: Code is generated during `cargo build` via
`build.rs` scripts, eliminating the need to run `codegen/` scripts manually.

2. **Separate Crates**: Message types and gRPC services are in separate crates,
so users who only need the proto messages don't need to depend on `tonic`.

3. **gRPC Reflection**: File descriptors are exported as part of 
`p4runtime-prost`, enabling gRPC reflection support.

## How can I see the generated code?

### Via the documentation

Simply run
```
cargo doc --open
```
to build documentation for the crates and bring it up in your default web
browser. Then, navgiate to the desired object and click "Source".

### Using cargo outdir

First install `cargo-outdir`:
```
cargo install cargo-outdir
```
Then navigate to the crate you're interested in, e.g. `cd p4runtime-prost`,
and run
```
cd $(cargo outdir --no-names)
```
to navigate to the build directory containing all generated files.
