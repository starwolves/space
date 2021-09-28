
# Space Frontiers server

  

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg) ![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)

  

## Description

  

A modular & moddable multi-threaded sci-fi headless community game server written in Rust with the [Bevy ECS](https://bevyengine.org/) game engine. Made to communicate exclusively with the official moddable Space Frontiers client which is being built with the [Godot Engine](https://godotengine.org/).
This game server is designed to run well on modern processors that have multiple CPU cores.
  
### Features
* Parallelized ECS (Entity Component System) architecture. 🐆
* Pure Rust. No garbage collection & high execution speeds.
* Fearless multi-threading and resource access across multiple threads.
* Built with the concurrent [Rapier 3D Physics engine](https://rapier.rs/).
* Interpolation throttling on a per client basis to meet bandwidth usage quotas.
* Built from the ground up to support safe and secure server-side modding with content folders. 
* A concurrent [Doryen-FOV](https://github.com/jice-nospam/doryen-fov) (field of view) algorithm for all pawns.
* Server-side moddable map support with sizes up to 1km by 1km with 100k+ dynamic ship cells that make up the map.
* Server-side moddable inventory system support.
* Server-side moddable combat system support.
* Server-side moddable advanced chat, with support for radio and proximity channels.
* Server-side moddable console commands, including rcon admin commands.

## Getting Started

  

### Dependencies



* [Rust](https://www.rust-lang.org/)

  

  

### Executing program

  

* To compile & run the game server:

```
cargo run --release
```

### Space Frontiers client
You need to get the official Space Frontiers client together with the standard client content folder to be able to connect to the server. There are no public releases of the client and its content folder yet.

The client is built on top of the latest stable Godot 3 release. This also means that there are graphical artifacts present on certain hardware. The client is also relatively demanding of hardware it runs on due to the limited dynamic lighting rendering performance of Godot 3.
However, most devices made for video-games should have no problem with the client.

When Godot 4 is stable enough, the client will be upgraded and moved to Godot 4 for better 3D rendering in favour of the Vulkan API  which aims to resolve the aforementioned issues.


## Contributions
Contributions in the form of pushed code, feedback, suggestions and critique are very much appreciated.
