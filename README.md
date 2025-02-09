# rust-pwa-example

A simple example of building a PWA using Rust

Build using [wasm-pack](https://rustwasm.github.io/wasm-pack/):

```bash
wasm-pack build --dev --target web --out-name index
```

Run using [http-server](https://www.npmjs.com/package/http-server):

```bash
http-server -g -c-1 .
```

