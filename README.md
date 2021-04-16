# netns - network namespaces in Rust #

The netns crate provides a simple and intuitive interface for handling
network namespaces in Rust. It should be noted that since the switching 
of namespace requires privileged authority, the authority requirements 
for compliance should be met in practice.

## Quick Start ##

To use `netns`, add the dependencies to your `Cargo.toml`
```toml
[dependencies]
netns = { git = 'https://github.com/inkhare/netns' }
```

Testing (requires root):

    cargo run --example test

## Example ##

```Rust
use std::process::Command;

use netns::NetNS;
use netns::{apply_in_ns, ExecStatus};

fn main() {
    let closeure = || {
        let output = Command::new("sh").arg("-c").arg("ip a").output();
        println!("ip a = {:?}", output);
        ExecStatus::Success(0)
    };

    // 2667 is the namespace pid
    apply_in_ns(2667, closeure);
}

```

## NOTE

This library is verified to work in rustc 1.51.0 (nightly), and the support 
of other versions needs more testing.
