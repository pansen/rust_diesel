# diesel

Diesel's `Getting Started` guide using SQLite for Actix web

## Prerequisites

```bash
# if ubuntu : sudo apt-get install libsqlite3-dev libdbus-1-dev
# if fedora : sudo dnf install libsqlite3x-devel dbus-devel
```

Ensure this to avoid warnings we cannot control:

```bash
$ cat ~/.cargo/config
# https://stackoverflow.com/a/38040431
# https://github.com/rust-lang/rust/issues/50504#issuecomment-412341631
[build]
rustflags = ["-Aproc-macro-derive-resolution-fallback"]
```

## Usage

Default Rust to use nightly, cause we are using Rust edition 2018

```bash
rustup default nightly
```

### init database sqlite

```bash
cargo install diesel_cli --no-default-features --features sqlite
cp .env.example .env
diesel migration run
```

### server

```bash
cargo install cargo-watch

cargo watch -d 0.1 -x run

# Started http server: 127.0.0.1:8080
```

### tests

```bash
cargo install cargo-testify

cargo testify
```

### web client

[http://127.0.0.1:8080/NAME](http://127.0.0.1:8080/NAME)

### sqlite client

```bash
# if ubuntu : sudo apt-get install sqlite3
# if fedora : sudo dnf install sqlite3x
sqlite3 test.db
sqlite> .tables
sqlite> select * from users;
```


## Postgresql

You will also find another complete example of diesel+postgresql on      [https://github.com/TechEmpower/FrameworkBenchmarks/tree/master/frameworks/Rust/actix](https://github.com/TechEmpower/FrameworkBenchmarks/tree/master/frameworks/Rust/actix)
