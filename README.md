# Rustcaster

Pseudo-3D raycaster implementation in Rust without any graphics API.

(I only use OpenGL/HtmlCanvas for rendering the final image to the screen)

You can try it out [here](https://rustcaster-r53tnd3k4a-lz.a.run.app/).

## 🤨 But Why? 

Just for fun and learning. 🙂

## 🖥 Run on desktop

```shell
$ cargo run
// OR
$ cargo run --release
```

## 🌐 Run on web

```shell
$ npm run build
$ npm run serve
// Then navigate to http://localhost:8080/
```

## 🕹 Controls

- Move: `W`, `S`,
- Loook: `A`, `D`
- Strafe: `Q`, `E`
- Open door: `SPACE` (when close enough, a bit buggy though 😅)

## 📝 Plans

- [ ] Refactor the raycasting code (it's a bit messy)
- [ ] Add proper texture support (32x32 sRBG pngs or a similar format, TextureAtlas?)
- [ ] Implement billboard sprites (https://www.youtube.com/watch?v=w0Bm4IA-Ii8)
- [ ] Restructure the code to be a bit better, clean up main.rs messy OpenGL code.
- [ ] Maybe add a gun and shooting enemies?
- [ ] TODOs and FIXMEs
- [ ] GBA port? 😅 (gba crate, agb crate)

## 📚 Resources
- [3DSage's Make Your Own Raycaster series](https://www.youtube.com/watch?v=gYRrGTC7GtA)
- [Lodev's Raycasting Tutorial](https://lodev.org/cgtutor/raycasting.html)
- [Ray-Casting Tutorial For Game Development And Other Purposes](https://permadi.com/1996/05/ray-casting-tutorial-table-of-contents/)
- [JS Raycaster](https://andrewmushel.com/portfolio/js-raycaster.html)