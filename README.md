
# Space Frontiers server

  

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg) ![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)

  

## Description

  

A modular & moddable multi-threaded sci-fi headless community game server written in Rust with the [Bevy ECS](https://bevyengine.org/) game engine. Made to communicate exclusively with the official moddable Space Frontiers client which is being built with the [Godot Engine](https://godotengine.org/).
This game server is designed to run well on modern processors that have multiple CPU cores.
  
### Features
* Parallelized ECS (Entity Component System) architecture. 🐆
* Pure Rust. No garbage collection & high parallel game logic execution speeds.
* Fearless multi-threading and resource access across multiple threads.
* Built with the cutting-edge & concurrent [Rapier 3D Physics engine](https://rapier.rs/).
* Netcoded 3D positions are broadcasted, rates dynamically throttled on a per client basis to meet bandwidth usage quotas.
* A concurrent [Doryen-FOV](https://github.com/jice-nospam/doryen-fov) (field of view) algorithm for all pawns.
* Moddable and cell based map support including a GUI editor with support for sizes up to 1km by 1km with 100k+ dynamic ship cells as map size is currently bottlenecked by the FOV algorithm. 
* Character meshes and animations are fully moddable and integrated with [Mixamo](https://www.mixamo.com/) for rigging.
* Entities are loaded from an external content folder for the client. Allowing modders to create new entities such as items, characters, sounds, ship cells and more.
* Godot Addressable references are used for efficient and dynamic netcode that works well with custom content.
* Data-oriented & modular, everything is its own entity with components in a strictly compiled and extremely fast ECS database-like approach. It is easy to add and remove systems, components and to turn them into plugins and more.
* Inventory system, pick up, wear, attach and equip items with character entities.
* Melee & projectile combat system, damage players, ship walls or other entities.
* Advanced bbcode chat, with support for examining entities, modular (radio) channels and proximity communication.
* Configurable console commands, including rcon admin commands.

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
