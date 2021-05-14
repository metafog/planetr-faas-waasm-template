<div align="center">

  <h1><code>planetr-faas-waasm-template</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly function as a service project using <a href="https://planetr.io">Planetr</a>.</strong>
</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
running the resulting package on Planetr.

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/planetrio/planetr-faas-waasm-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```
