# Cargo Rust project workspace base

This is template structure for cargo projects with workspace

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```sh
cargo generate --git https://github.com/Grapple228/rust-workspace-base.git --name my-project
cd my-project
```

## Dev setup

Firstly install `cargo-watch`

```sh
cargo install cargo-watch
```

### For execution app on save, use command

```sh
cargo watch -q -c -w crates/services/test-service/src/ -w crates/libs/ -w .cargo/ -x "run -p test-service"
```

### For execution test app on save, use command

```sh
cargo watch -q -c -w crates/services/test-service/examples/ -x "run -p test-service --example quick-dev"
```

### For execution tests on save, use command

```sh
cargo watch -q -c -x "test -q -p lib-fs test_fs_execute_not_error -- --nocapture"
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
