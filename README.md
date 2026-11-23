# Particule

A real-time 2D particle simulation written in Rust, built as a project to learn the language.

> 🚧 Work in progress

![Latest demo: collisions](assets/demo-02-collisions.gif)

## Features

- [x] Window, render loop and FPS counter
- [x] Particles with gravity and edge bounces
- [x] Particle-particle collisions (naive O(n²))
- [ ] Spatial grid to speed up collisions
- [ ] Mouse attract / repel
- [ ] Inter-particle forces / boids
- [ ] Settings panel with sliders (egui)
- [ ] Multithreading with rayon

## Progress

Demos recorded along the way, newest first.

### 02 · Particle collisions

Particles bounce off each other and stack up instead of passing through (naive O(n²) check).

![Collisions](assets/demo-02-collisions.gif)

### 01 · Gravity and edge bounces

500 random particles falling with gravity, bouncing off the window edges with damping.

![Gravity and bounces](assets/demo-01-gravity.gif)

## Running

Requires [Rust](https://rustup.rs/).

```bash
cargo run --release
```

## Built with

- [macroquad](https://github.com/not-fl3/macroquad): window, rendering, input
