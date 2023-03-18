# Rustcaster

Pseudo-3D raycaster implementation in Rust without OpenGL.

Based on: [3DSage's Make Your Own Raycaster series](https://www.youtube.com/watch?v=gYRrGTC7GtA)

Why? Just for fun and learning. 🙂

## 🖥 On desktop

```shell
$ cargo run --release
```
In debug mode it's too slow to be playable. 😅

## 🛠️ Build for web

```shell
$ wasm-pack build --target web
// OR
$ npm run build
```

## 🚀 Run

```shell
$ npm run serve
// Then navigate to http://localhost:8080/public
```

## 🕹 Controls

- Move: `W`, `A`, `S`, `D`
