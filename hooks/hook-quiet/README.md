# hook-quiet

Observes every lifecycle event and changes nothing. It is the test source's case 2: a hook in a mutable release, published while release immutability was off.

Build the component, then pack it:

```bash
cargo build --release --target wasm32-wasip2 -p hook-quiet
cp target/wasm32-wasip2/release/hook_quiet.wasm hooks/hook-quiet/
mur build hooks/hook-quiet --output release/
```
