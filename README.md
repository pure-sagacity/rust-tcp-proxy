# rust-tcp-proxy

A small asynchronous TCP proxy built with Rust and Tokio.

The proxy accepts client connections on `0.0.0.0:3000` and forwards each
connection to `127.0.0.1:6767`. Data is copied in both directions until one
side closes the connection or an I/O error occurs.

## Requirements

- Rust and Cargo with support for the 2024 edition
- A TCP service listening on `127.0.0.1:6767`

Install Rust with [rustup](https://rustup.rs/) if it is not already available.

## Run

Start the proxy from the repository root:

```sh
cargo run
```

The proxy prints a startup message and logs each accepted client connection.

Build an optimized binary with:

```sh
cargo build --release
```

The resulting executable is `target/release/rust-tcp-proxy`.

## Verify locally

Start a temporary TCP service in one terminal, for example with Netcat:

```sh
nc -l 127.0.0.1 6767
```

Start the proxy in another terminal:

```sh
cargo run
```

Then connect through the proxy:

```sh
nc 127.0.0.1 3000
```

Text entered through the proxy should appear in the terminal running the
service, and responses from that service should be sent back to the client.

## Configuration

The listen and target addresses are currently compile-time constants in
`src/lib.rs`:

```rust
pub const LISTEN_ADDRESS: &str = "0.0.0.0:3000";
pub const TARGET_ADDRESS: &str = "127.0.0.1:6767";
```

Change those constants and rebuild to use different addresses.

## License

See [LICENSE](LICENSE).