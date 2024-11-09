# Cargo Rust project workspace base

This is template structure for cargo projects with workspace

## Dev setup

Firsly install cargo-watch

```sh
cargo install cargo-watch
```

For execution app on save, use command:

```sh
cargo watch -q -c -w crates/services/test-service/src/ -w crates/libs/ -w .cargo/ -x "run -p test-service"
```

For execution test app on save, use command:

```sh
cargo watch -q -c -w crates/services/test-service/examples/ -x "run -p test-service --example quick-dev"
```

For execution test on save, use command:

```sh
cargo watch -q -c -x "test -q -p lib-fs test_fs_execute_not_error -- --nocapture"
```
