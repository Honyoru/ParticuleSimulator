# Particule

A real-time 2D particle simulation written in Rust, built as a project to learn the language.

> 🚧 Work in progress

![Latest demo: mouse attract and repel](assets/demo-03-mouse.gif)

## Features

- [x] Window, render loop and FPS counter
- [x] Particles with gravity and edge bounces
- [x] Particle-particle collisions (naive O(n²))
- [x] Spatial grid to speed up collisions
- [x] Mouse attract / repel
- [ ] Inter-particle forces / boids
- [ ] Settings panel with sliders (egui)
- [ ] Multithreading with rayon

## Progress

Demos recorded along the way, newest first.

### 03 · Mouse attract and repel

Hold left click to pull the particles inside the ring toward the cursor, right click to push them away.
Collisions now use a spatial grid, so only nearby particles are compared.

![Mouse attract and repel](assets/demo-03-mouse.gif)

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

### Controls

| Input | Action |
|---|---|
| Left click (hold) | Attract particles inside the ring |
| Right click (hold) | Repel particles inside the ring |

## Built with

- [macroquad](https://github.com/not-fl3/macroquad): window, rendering, input
