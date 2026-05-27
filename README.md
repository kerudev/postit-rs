# 📝 postit-rs

[![Coverage Status](https://coveralls.io/repos/github/keruDev/postit-rs/badge.svg?branch=master)](https://coveralls.io/github/keruDev/postit-rs?branch=master)
[![Build Status](https://github.com/keruDev/postit-rs/workflows/CI/badge.svg)](https://github.com/keruDev/postit-rs/actions)
[![Current Crates.io Version](https://img.shields.io/crates/v/postit.svg)](https://crates.io/crates/postit)
[![Docs.rs](https://img.shields.io/badge/postit-blue.svg?label=docs.rs)](https://docs.rs/postit/latest/postit/)

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).

postit is a simple CLI utility aimed to help you manage and keep track of your tasks.

> [!WARNING]
> Some commands may not work as expected in version `0.2.1` when using `csv`, `json` or `xml` formats.
> Please update to the latest version by running: `cargo install postit`.

## Index

- [From 0.1.x to 0.2.x](#from-01x-to-02x): brief migration guide.
- [Getting started](#getting-started): describes installation and first steps.
- [Features](#features): postit's functionalities and new additions roadmap. 
- [Configuration](#configuration): describes configuration options.

## From 0.1.x to 0.2.x

The 0.1.x minor marked the beginning of postit's development, but the best is
yet to come. As of 0.1.x, postit featured csv and json file support, as well
as some basic commands to manage tasks and the configuration file.

By bumping the version to 0.2.x, it is intended to mark the first great step
of postit to becoming a more serious product.

To migrate from 0.1.x to 0.2.x, you'll need to change the `--path` flag to 
`--persister` (pretty simple, right?).

This minor will be focused on providing support for more database systems
(MongoDB or MySQL) along with some more file extensions (XML) and more commands
to make task management simpler.

Hope to cross paths in future versions :)

## Getting started

To install `postit`, just run:

```sh
cargo install postit
```

By default, `postit` will generate files inside `$HOME/.postit`. You can set a
path in the `POSTIT_ROOT` environment variable to override the default value.

On Linux:

```sh
# ~/.bashrc

# Feel free to change this line
export POSTIT_ROOT="$HOME/.postit"
```  

On Windows:

```ps1
# Feel free to change this line
[Environment]::SetEnvironmentVariable("POSTIT_ROOT", "$env:USERPROFILE\.postit", "User")
```

Here is a list of other useful commands to get started:
- `postit help`: a list of all possible commands.
- `postit docs`: documentation and use examples for every command and flag.

## Features

Although `postit` is still in early development, it is alive and keeps growing!
Here are some of its current features:

- `default`: support for `CSV` files (no dependencies).
- `json`: support for `JSON` files.
- `xml`: support for `XML` files.
- `sqlite`: support for `SQLite` databases.
- `mongo`: support for `MongoDB` and `MongoDB Atlas` databases.
- `all-fs`: installs all file features (`json`, `xml` and `sqlite`).
- `all-db`: installs all database features (`sqlite` and `mongo`).
- `all`: installs everything (`all-fs` and `all-db`).

Roadmap:
- [x] XML support
- [x] MongoDB support
- [ ] MySQL support
- [ ] Tasks filtering and sorting

## Configuration

postit's behavior can be changed using the `.postit.toml` file. Check out its
possible fields in the [docs](https://docs.rs/postit/latest/postit/struct.Config.html)
or run:

```sh
postit docs config
```

The command below will generate postit's configuration file inside `POSTIT_ROOT`:

```sh
postit config init
```

## Contributing

Read [`docs/DEVELOPMENT.md`](./docs/DEVELOPMENT.md).
