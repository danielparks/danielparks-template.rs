# {{project-name}}

{% if crate_type == "lib" -%}
[![docs.rs](https://img.shields.io/docsrs/{{project-name}})][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/{{project-name}})][crates.io]
![Rust version 1.60+](https://img.shields.io/badge/Rust%20version-1.60%2B-success)

{% endif -%}

Generated from template https://github.com/danielparks/danielparks-template.rs
with [cargo-generate](https://github.com/ashleygwilliams/cargo-generate).

{% if crate_type == "bin" -%}
## Installation

```sh
cargo install {{project-name}}
```

## Rust Crate

[![docs.rs](https://img.shields.io/docsrs/{{project-name}})][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/{{project-name}})][crates.io]
![Rust version 1.60+](https://img.shields.io/badge/Rust%20version-1.60%2B-success)

{% endif -%}
## License

This project dual-licensed under the Apache 2 and MIT licenses. You may choose
to use either.

  * [Apache License, Version 2.0](LICENSE-APACHE)
  * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.

[docs.rs]: https://docs.rs/{{project-name}}/latest/{{crate_name}}/
[crates.io]: https://crates.io/crates/{{project-name}}
