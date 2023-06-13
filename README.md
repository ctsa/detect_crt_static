Minimal demonstration of `crt-static` detection confusion. Why isn't this detected in the build.rs when compiled with:

```
RUSTFLAGS='-C target-feature=+crt-static -C relocation-model=static' cargo run --release --target x86_64-unknown-linux-gnu
```