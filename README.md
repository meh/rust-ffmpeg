### Installation

First add the dependency to your `Cargo.toml`
```toml
[dependencies]
...
ffmpeg = "0.3.0"
```
Then update (will likely [take a while](https://stackoverflow.com/questions/53361052/how-do-i-debug-cargo-build-hanging-at-updating-crates-io-index) due to size of downloaded ffmpeg lib)
```bash
cargo update
```

### Usage

```rust
use std::{fs::File};

fn main() {
    ffmpeg::init().unwrap();
    let file = File::open("path/to/your/video/file.mkv").unwrap();
    match ffmpeg::format::input(file) {
        Ok(context) => {
            for (k, v) in context.metadata().iter() {
                println!("{}: {}", k, v);
            }
        }
        Err(error) => println!("error: {}", error),
    }
}
```
By running it via:
```bash
cargo run
```
The output is:
```
error: failed to run custom build command for `ffmpeg-sys v4.2.1`
```

For more examples see [`/examples/`](https://github.com/meh/rust-ffmpeg/blob/master/examples/metadata.rs).
