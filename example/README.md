# Example

In order to run a demo app, run:

```
wasm-pack build --release
npm install
npm start
```

After that, go to [http://localhost:8080](http://localhost:8080) and open Console, you will see log messages. You can
try changing code in [src/lib.rs](https://github.com/wasm-rs/dbg/blob/master/example/src/lib.rs) or `features` in
[Cargo.toml](https://github.com/wasm-rs/dbg/blob/master/example/Cargo.toml) to send the output at different log levels.
