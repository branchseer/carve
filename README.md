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
// hello_editres/src/main.ts

use editres::resource
use std::str::from_utf8;

fn main() {
    const res = resource!('my_res'); // Option<&'static [u8]>
    if let Some(res) = res {
        println!("{}", from_utf8(res).unwrap());
    } else {
        println!("my_res is not embedded yet");
    }
}
```

```sh
> cargo run
my_res is not embedded yet
```

### 2. Embed data into the executable with editres cli or injector API.

```
> cargo install editres_cli
```

```
> echo "embedded data" | editres edit target/debug/hello_editres --name my_res --output hello_editres2
> ./hello_editres2
embedded data
```
