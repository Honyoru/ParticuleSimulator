# Particule

A real-time 2D particle simulation written in Rust, built as a project to learn the language.

> 🚧 Work in progress

![demo](assets/demo.gif)

## Features

- [x] Window, render loop and FPS counter
- [x] Particles with gravity and edge bounces
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
