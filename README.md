# Frunze API Server

[![Build Status](https://travis-ci.org/azasypkin/frunze-api.svg?branch=master)](https://travis-ci.org/azasypkin/frunze-api)
[![License](https://img.shields.io/github/license/mashape/apistatus.svg)](https://raw.githubusercontent.com/azasypkin/frunze-api/master/LICENSE)

Rust API server supporting [__Frunze Web IDE__](https://github.com/azasypkin/frunze).

## Development

__Frunze API Server__ is API server written in Rust that relies on MongoDB database.

### Database

To setup database please follow the instructions from [Frunze repository](https://github.com/azasypkin/frunze/blob/master/README.md#database).

### Bill of Materials API

To estimate the price of your project, we should gather real-time offers for every project hardware component from the 
distributors (DigiKey, Mouser etc.) and we leverage [Octopart](https://octopart.com/) for that. That means you should
supply `--bom-api-key xxxxxx` command line argument with the proper [Octopart API Key](https://octopart.com/api/home). 

### Schematic Export API

To locally deploy Schematic Export API please follow the instructions from [Frunze repository](https://github.com/azasypkin/frunze/blob/master/README.md#frunze).

### Run Server

Run development API server with (localhost:8009 by default):

```bash
$ cargo run
```

or if you'd like to use custom IP address or/and port (don't forget to re-configure client part as well):

```bash
$ cargo run -- --ip 127.0.0.2 --port 8008
```

with Export API key:
```bash
$ RUST_LOG=frunze_api cargo run -- --bom-api-key xxxxxxxx
```

For custom MongoDB instance use the following parameters:

```bash
$ cargo run -- ... --db-ip 127.0.0.3 --db-port 27018 --db-name my-own-db-name
```

There is also option to run the API server in a dedicated Docker container if you just want to check it out:

```bash
$ docker build -t frunze-api:dev .
$ docker run -d --name frunze-api -p 8009:8009 frunze-api:dev
```

### Build Server

Run `cargo build`. The build artifacts will be stored in the `target/` directory. Use `--release` flag
for a production build.

### Run unit tests

Server part unit tests rely on `stainless` crate and hence require Rust Nightly (the server itself works fine with Rust Stable). It's recommended
to use [`rustup`](https://rustup.rs) to deal with several Rust versions simultaneously. Let's say you use `rustup`, then to run unit tests
just run:

```bash
$ cargo +nightly test
```

### Run Rust linter and source code formatter

To run [Clippy](https://github.com/Manishearth/rust-clippy) checks:

```bash
$ cargo +nightly clippy
```

To format project with [RustFmt](https://github.com/rust-lang-nursery/rustfmt):

```bash
$ cargo +nightly fmt
```
