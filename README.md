## To reproduce

```bash
git clone https://github.com/akiomik/nostr-sdk-timeout-issue --recursive 
git apply patch
RUST_LOG=debug cargo run --release
```
