##Interactive Raycasting Demo in Rust
A simple interactive raycasting demo built with Rust and SDL2 that visually simulates light rays and shadows using circles.

#Overview
This project demonstrates basic 2D raycasting and collision detection by rendering light rays originating from a circle and casting shadows on another circle. The user can interactively drag both the light source and the shadow-casting object with the mouse.

#Key features:

Real-time drawing of light rays emanating from a circular light source.

Dynamic shadow casting by detecting collisions with shadow objects.

Interactive dragging of the light source and shadow circle with mouse input.

Uses SDL2 for rendering and input handling.

#How It Works
A "light" circle emits 360 rays in all directions.

Rays stop when they collide with the "shadow" circle.

The light circle and shadow circle can be moved by dragging them with the mouse.

The scene updates in real-time at approximately 60 FPS.

Requirements
Rust (tested on 1.70+)

SDL2 development libraries installed on your system

Setup Instructions
Install SDL2

On Ubuntu/Debian:

bash
Copy
Edit
sudo apt-get install libsdl2-dev
On macOS (using Homebrew):

bash
Copy
Edit
brew install sdl2
On Windows:
Download and install SDL2 development libraries from the SDL website.

Clone the repository

bash
Copy
Edit
git clone https://github.com/SiddharthV147/Interactive-Raycasting-Demo-in-Rust.git
cd Interactive-Raycasting-Demo-in-Rust
Build and run

bash
Copy
Edit
cargo run
Usage
Click and drag the white light circle to move the light source.

Click and drag the larger white shadow circle to move the object casting the shadow.

Press Esc or close the window to exit.

Code Structure
main.rs contains:

Circle struct representing circles with collision detection.

draw_light_circle and draw_shadow_circle functions for rendering.

draw_light_rays function emitting and drawing rays with collision checking.

SDL2 event loop handling mouse interaction and rendering loop.
