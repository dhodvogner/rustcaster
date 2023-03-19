# Rustcaster

Pseudo-3D raycaster implementation in Rust without OpenGL.

Based on: [3DSage's Make Your Own Raycaster series](https://www.youtube.com/watch?v=gYRrGTC7GtA)

Why? Just for fun and learning. 🙂

## 🖥 Run on desktop

```shell
$ cargo run
// OR
$ cargo run --release
```

## 🌐 Run on web

```shell
$ wasm-pack build --target web
// OR
$ npm run build
// Then
$ npm run serve
// Then navigate to http://localhost:8080/public
```

## 🕹 Controls

- Move: `W`, `S`,
- Loook: `A`, `D`
- Strafe: `Q`, `E`
