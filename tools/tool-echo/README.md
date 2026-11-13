# tool-echo

Returns the `text` it is given. It is the test source's case 1: a tool released as immutable, with a release attestation.

Build the component, then pack it:

```bash
cargo build --release --target wasm32-wasip2 -p tool-echo
cp target/wasm32-wasip2/release/tool_echo.wasm tools/tool-echo/
mur build tools/tool-echo --output release/
```
