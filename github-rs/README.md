# github-rs

| Service      | Status |
| -------      | :----: |
| TravisCI     | [![Build Status](https://travis-ci.org/mgattozzi/github-rs.svg?branch=master)](https://travis-ci.org/mgattozzi/github-rs)   |
| DependencyCI | [![DependencyStatus](https://dependencyci.com/github/mgattozzi/github-rs/badge)](https://dependencyci.com/github/mgattozzi/github-rs)   |
| AppveyorCI   | [![Build status](https://ci.appveyor.com/api/projects/status/st04a7hltt8h42lq?svg=true)](https://ci.appveyor.com/project/mgattozzi/github-rs)       |
| CodeCov   | [![codecov](https://codecov.io/gh/mgattozzi/github-rs/branch/master/graph/badge.svg)](https://codecov.io/gh/mgattozzi/github-rs)      |
| crates.io | ![crates.io](https://img.shields.io/crates/v/github-rs.svg)

Pure Rust bindings to the Github API

## Incomplete Bindings
Please look at the [endpoints](./docs/endpoints.md) docs to see which endpoints
are currently covered in the API.

## Dependencies and Support
github-rs is intended to work on all tier 1 supported Rust systems:

- Windows
- Linux
- MacOSX

## Minimum Compiler Version
Due to the use of certain features github-rs requires rustc version 1.18 or
higher.

## Project Aims
- Have a robust API where everything is error handled properly to avoid
  panics of any kind. A library is the base of an application and should
  be a solid foundation to be built upon
- Cover all Github stable endpoints. Anything that's deprecated and beta
  should be obtained only through configuration for those features. As
  deprecated endpoints are removed from Github so too should they be
  removed from this library.
- Having stability as part of the API. As such effort will be
  taken to make sure this code compiles on stable Rust, rather than
  nightly.
- Ease of use. The complexity should be hidden from those not hacking on
  the code itself.
- Documentation of everything so not only is it easy to hack on but
  finding out how to use the library should be easy to find.

## Getting Started
Add the following to your `Cargo.toml`

```toml
[dependencies]
github-rs = "0.5"
```

Then in your `lib.rs` or `main.rs` file add:

```rust
extern crate github_rs
use github_rs::client::Github;
```

Now you can start making queries. Here's a small example to get your user
information:

```rust
extern crate github_rs;
use github_rs::client::Github;

fn main() {
    let client = Github::new("API TOKEN");
    let me = client.get()
                   .user()
                   .execute();
    match me {
        Ok((headers, status, json)) => {
            println!("{}", headers);
            println!("{}", status);
            if let Some(json) = json{
                println!("{}", json);
            }
        },
        Err(e) => println!("{}", e)
    }
}
```

## Hacking on the Library
- [GitHub API Reference Docs](https://developer.github.com/v3/)
- See the [design docs](./docs/design.md) for more information.

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
