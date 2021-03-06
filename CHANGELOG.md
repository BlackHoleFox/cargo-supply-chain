## v0.1 (2021-02-10)

- Drop `authors` subcommand
- Add `help` subcommand providing detailed help for each subcommand
- Bring help text more in line with Cargo help text
- Warn about a large amount of data to be downloaded in `update` subcommand

## v0.0.4 (2021-01-01)

- Report failure instead of panicking on network failure in `update` subcommand
- Correctly handle errors returned by the remote server

## v0.0.3 (2020-12-28)

- In case of network failure, retry with exponential backoff up to 3 times
- Use local certificate store instead of bundling the trusted CA certificates
- Refactor argument parsing to use `pico-args` instead of hand-rolled parser

## v0.0.2 (2020-10-14)

- `crates` - Shows the people or groups with publisher rights for each crate.
- `publishers` - Is the reverse of `crates`, grouping by publisher instead.
- `update` - Caches the data dumps from `crates.io` to avoid crawling the web
  service when lookup up publisher and author information.

## v0.0.1 (2020-10-02)

Initial release, supports one command:
- `authors` - Crawl through Cargo.toml of all crates and list their authors.
  Authors might be listed multiple times. For each author, differentiate if
  they are known by being mentioned in a crate from the local workspace or not.
  Support for crawling `crates.io` sourced packages is planned.
- `publishers` - Doesn't do anything right now.
