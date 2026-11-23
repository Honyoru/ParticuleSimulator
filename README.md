# Particule

A real-time 2D particle simulation written in Rust, built as a project to learn the language.

> 🚧 Work in progress

<!-- Add a GIF here once particles are moving: ![demo](assets/demo.gif) -->

## Features

- [ ] Window, render loop and FPS counter
- [ ] Particles with gravity and edge bounces
- [ ] Particle-particle collisions (spatial grid)
- [ ] Mouse attract / repel
- [ ] Inter-particle forces / boids
- [ ] Settings panel with sliders (egui)
- [ ] Multithreading with rayon

## Running

Requires [Rust](https://rustup.rs/).

```bash
cargo run --release
```

## Built with

- [macroquad](https://github.com/not-fl3/macroquad): window, rendering, input
