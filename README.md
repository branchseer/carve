# editres

[![crates.io](https://img.shields.io/crates/v/editres.svg)](https://crates.io/crates/editres)

Embed data into executables after build.

[Documentation](https://docs.rs/editres)

## Supported Executable Formats

- Windows (PE)
- Linux (ELF)
- macOS (Mach-O)

## Quick Start

### 1. Declare resources using `editres::resource!`

```rust
use editres::resource;
use std::str::from_utf8;

# fn main() {
let res = resource!("my_res"); // Option<&'static [u8]>
if let Some(res) = res {
    println!("{}", from_utf8(res).unwrap());
} else {
    println!("my_res is not injected yet");
}
# }
```

### 2. Inject data in the executable

You can inject data in executables using library `editres` or command line from `editres_cli`.

- Library usage:
    1. Add `editres` as a dependency with feature `injector` enabled,
    2. Refer to [`injector::inject`](https://docs.rs/editres/latest/editres/injector/fn.inject.html).
- Command line usage:
    1. `cargo install editres_cli`, or download from the releases.
    2. Get command line help using `editres help inject`

# Notes

editres is based on Node.js' single executable application implementation: [postject](https://github.com/nodejs/postject/).

