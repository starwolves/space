
# Space Frontiers server

  

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg) ![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)

  

## Description

  

A modular & moddable multi-threaded sci-fi headless community game server written in Rust with the [Bevy ECS](https://bevyengine.org/) game engine. Made to communicate exclusively with the official moddable Space Frontiers client which is being built with the [Godot Engine](https://godotengine.org/).
This game server is designed to run well on modern processors that have multiple CPU cores.
  
### Features (All Moddable & Modular)
* Parallelized ECS (Entity Component System) architecture. 🐆
* Pure Rust. No garbage collection & high parallel game logic execution speeds.
* Data-oriented, everything is its own entity with components within a thread-safe and strictly compiled environment. It is easy to add and remove entities, systems, components, map cells and more simply by managing [plugins](https://bevyengine.org/learn/book/getting-started/plugins/) that will get compiled with the project.
* Inventory system, pick up, wear, attach, place and equip items with character entities.
* Melee & projectile combat systems, damage player, ship walls or other entities with various types of damage and the ability to target specific body parts.
* Advanced bbcode chat, with support for examining entities, modular (radio) channels and proximity communication.
* Actions and tab menu's to easily interact with the world while also offering protection against cheaters.
* Configurable console commands, including rcon admin commands.
* Fearless multi-threading and resource access across multiple threads.
* Built with the cutting-edge [Rapier 3D Physics engine](https://rapier.rs/).
* Netcoded 3D positions are broadcasted, rates dynamically throttled on a per client basis to meet bandwidth usage quotas.
* A concurrent [Doryen-FOV](https://github.com/jice-nospam/doryen-fov) (field of view) algorithm for all pawns.
* Clients can load in custom entities and custom game data on a per server basis thanks to a traditional content folder approach. Allowing modders to create new entities such as items, characters, sounds, ship cells and MUCH more.
* Godot Addressable references are used for efficient and dynamic netcode that works well with custom content.
* Cell based map support including a GUI editor with support for sizes up to 1km by 1km with 100k+ dynamic (de)constructable ship cells as map size is currently bottlenecked by the FOV algorithm. 
* Character meshes and animations are fully integrated with [Mixamo](https://www.mixamo.com/) for rigging.


## Getting Started

### [License](https://github.com/starwolves/space/blob/master/LICENSE)
This server-side project has a non-standard and non-commercial license, please read it and agree to it before you continue. Find the license [here](https://github.com/starwolves/space/blob/master/LICENSE).

### Dependencies



* [Rust](https://www.rust-lang.org/)

  

  

### Executing program

  

* To compile & run the game server:

```
cargo run
```

### [Documentation](https://docs.starwolves.io)
Yet to be created.

### Contributions
Pushed code, feedback, bug reports, suggestions and critique are very much appreciated. Github issues will be reviewed and considered.
You can get in contact with the developers through [Discord](https://discord.gg/qcg4zPuHyU).
Aditionally you can also get in contact on Discord by contacting STARWOLF#5816 .

### Space Frontiers client
You need to get the official closed source Space Frontiers client together with the standard client-side content folder to be able to connect to the server.
You can get the latest stable releases of the client at [Discord](https://discord.gg/qcg4zPuHyU) by contacting STARWOLF#5816.
Aditionally the client can be obtained at our own Matrix server without requiring you to contact anyone, more information about our Matrix on [our website](https://starwolves.io)
Ensure you select the right branch of the server that has the same version number as the client you have obtained.

The client is built on top of the latest stable Godot 3.4.x release. This also means that there are graphical artifacts present on certain hardware. The client is relatively demanding of hardware it runs on due to the limited dynamic lighting rendering performance of Godot 3.
However, most devices made for video-games should expect no such problems.

When Godot 4 is stable enough, the client will be upgraded and moved to Godot 4 for better 3D rendering in favour of the Vulkan API  which aims to resolve the aforementioned issues.

### Media
You can see a video of Space Frontiers in action [here](https://youtu.be/Qa-Y_PxzeiI).

### Testing the game
Please make sure you download and compile the latest stable branch of Space Frontiers server and not from the master branch. Also make sure that the stable server version matches the version of the Space Frontiers client that you have acquired at [comms.starwolves.io](https://comms.starwolves.io).

### [StarWolves.io](https://starwolves.io)
Star Wolves is a brand new gaming community that is pioneering the game Space Frontiers by hosting official servers for it and more.
Space Frontiers allows each community of players to compile and host a server instance themselves meaning the i/o gameplay and servers remain sovereign.
