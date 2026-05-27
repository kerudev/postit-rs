# Guide for developers

This file contains information to help developers understand the project's
workflow and some oddities.

## Testing

To run postit's tests, use this command:
```sh
cargo test -- --test-threads=1
```

You can also use `tarpaulin`, configured in the `.tarpaulin.toml` file.
It is slower, but shows line coverage (not branch coverage):
```sh
cargo tarpaulin --all-features -- --test-threads=1
```

The reason why tests are run synchronously is to not overwrite existing files,
control the execution flow (creation and cleanup of temp files) and keep them
as lightweight as possible, as they don't use external dependencies.

If you run the `mongo` test suite, you'll need to have a MongoDB instance
deployed. This may change in the future, but for now I'm okay with it.
