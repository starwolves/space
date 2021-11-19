
# Space Frontiers server

  

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg) ![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)

  

## Description

  

A modular & moddable multi-threaded sci-fi headless community game server written in Rust with the [Bevy ECS](https://bevyengine.org/) game engine. Made to communicate exclusively with the official moddable Space Frontiers client which is being built with the [Godot Engine](https://godotengine.org/).
This game server is designed to run well on modern processors that have multiple CPU cores.
  
### Features
* Parallelized ECS (Entity Component System) architecture. 🐆
* Pure Rust. No garbage collection & high parallel game logic execution speeds.
* Data-oriented & modular, everything is its own entity with components within a thread-safe and strictly compiled environment. It is easy to add and remove entities, systems, components, map cells and more simply by managing [plugins that will get compiled with the project.](https://bevyengine.org/learn/book/getting-started/plugins/)
* Inventory system, pick up, wear, attach, place and equip items with character entities.
* Melee & projectile combat systems, damage player, ship walls or other entities with various types of damage and the ability to target specific body parts.
* Advanced bbcode chat, with support for examining entities, modular (radio) channels and proximity communication.
* Configurable console commands, including rcon admin commands.
* Fearless multi-threading and resource access across multiple threads.
* Built with the cutting-edge & concurrent [Rapier 3D Physics engine](https://rapier.rs/).
* Netcoded 3D positions are broadcasted, rates dynamically throttled on a per client basis to meet bandwidth usage quotas.
* A concurrent [Doryen-FOV](https://github.com/jice-nospam/doryen-fov) (field of view) algorithm for all pawns.
* Clients can load in custom entities and custom game data on a per server basis thanks to a traditional content folder approach. Allowing modders to create new entities such as items, characters, sounds, ship cells and MUCH more.
* Godot Addressable references are used for efficient and dynamic netcode that works well with custom content.
* Moddable and cell based map support including a GUI editor with support for sizes up to 1km by 1km with 100k+ dynamic ship cells as map size is currently bottlenecked by the FOV algorithm. 
* Character meshes and animations are fully moddable and integrated with [Mixamo](https://www.mixamo.com/) for rigging.


## Getting Started

### [License](https://github.com/starwolves/space/blob/master/LICENSE)
This server-side project has a non-standard and non-commercial license, please read it and agree to it before you continue. Find the license [here](https://github.com/starwolves/space/blob/master/LICENSE).

### Dependencies



* [Rust](https://www.rust-lang.org/)

  

  

### Executing program

  

* To compile & run the game server:

```
cargo run --release
```

### [Documentation](https://docs.starwolves.io)
Find the documentation of this project [here](https://docs.starwolves.io).

### Contributions
Pushed code, feedback, bug reports, suggestions and critique are very much appreciated. Github issues will be reviewed and considered.
You can get in contact with the developers through a [Matrix client](https://matrix.to/#/#live:comms.starwolves.io) that interfaces with the decentralized communication server of Star Wolves.

[Matrix](https://matrix.org) is like a decentralized Discord, where data & community sovereignty matters.


### Space Frontiers client
You need to get the official closed source Space Frontiers client together with the standard client-side content folder to be able to connect to the server.
You can find the latest stable releases of the client at [Star Wolves](https://starwolves.io).

The client is built on top of the latest stable Godot 3.4.x release. This also means that there are graphical artifacts present on certain hardware. The client is relatively demanding of hardware it runs on due to the limited dynamic lighting rendering performance of Godot 3.
However, most devices made for video-games should expect no such problems.

When Godot 4 is stable enough, the client will be upgraded and moved to Godot 4 for better 3D rendering in favour of the Vulkan API  which aims to resolve the aforementioned issues.

### Testing the game
Please make sure you download and compile the latest stable branch of Space Frontiers server and not from the master branch. Also make sure that the stable server version matches the version of the Space Frontiers client that you have acquired at [comms.starwolves.io](https://comms.starwolves.io).

### [StarWolves.io](https://starwolves.io)
Star Wolves is a brand new **sovereign gaming community** that is pioneering the game Space Frontiers by hosting official servers for it and more.
It stays true to the sovereignty Space Frontiers gives communities in that it allows each community of players to compile and host a server istance themselves meaning the i/o gameplay and servers remain sovereign.
