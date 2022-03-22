# Space Frontiers pre-alpha
<img src="https://i.imgur.com/4CI2rb4.png" data-canonical-src="https://starwolves.io/images/sflogo.png" width="175" height="175"/>
  

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg) ![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)
![Screenshot of Space Frontiers gameplay](https://starwolves.io/images/sfss.png)
  

## Description

  

A modular & moddable multi-threaded sci-fi headless community game server written in Rust with the [Bevy ECS](https://bevyengine.org/) game engine. Made to communicate exclusively with the official moddable Space Frontiers client which is being built with the [Godot Engine](https://godotengine.org/).
This game server is designed to run well on modern processors that have multiple CPU cores.

### Media
You can see gameplay videos of Space Frontiers on [YouTube](https://youtu.be/Qa-Y_PxzeiI).
  
### Features (All Moddable & Modular)
* Parallelized ECS (Entity Component System) architecture. 📡
* Pure Rust. No garbage collection & high parallel game logic execution speeds. 🌟
* Data-oriented, everything is its own entity with components within a thread-safe and strictly compiled environment. It is easy to add and remove entities, systems, components, map cells and more simply by managing [plugins](https://bevyengine.org/learn/book/getting-started/plugins/) that will get compiled with the project. 🔭
* Using the cutting-edge [Rapier 3D Physics engine](https://rapier.rs/). 🚀
* Character meshes and animations are fully integrated with [Mixamo](https://www.mixamo.com/) for rigging. ☄
* Inventory system, pick up, wear, attach, place and equip items with character entities.
* Melee & projectile combat systems, damage player, ship walls or other entities with various types of damage and the ability to target specific body parts.
* Advanced bbcode chat, with support for examining entities, modular (radio) channels and proximity communication.
* Actions and tab menu's to easily interact with the world while also offering protection against cheaters.
* Configurable console commands, including rcon admin commands.
* Clients can load in custom content on a per server basis thanks to a traditional content folder approach. Allowing modders to create new entities such as items, characters, sounds, ship cells and more.
* Godot Addressable references are used for efficient and dynamic netcode that works well with custom content.
* Cell based map support including a GUI editor with support for sizes up to 1km by 1km with 100k+ dynamic (de)constructable ship cells as map size is currently bottlenecked by the FOV algorithm. 
* Atmospherics simulation including temperature, pressure, diffusion, gravity and the vacuum of space.
![Screenshot of Space Frontiers atmospherics simulation](https://starwolves.io/images/sfatmosss.png)

## Getting Started

### Dependencies



* [Rust](https://www.rust-lang.org/)

  

  

### Executing game server

  

To compile and run the game server:
* Select latest branch from this repository and download that code.
* In your terminal navigate to the project folder you have just obtained and run:

```
cargo run
```

### Space Frontiers client
You can get the latest stable releases of the closed-source client on [Discord](https://discord.gg/yYpMun9CTT).
Ensure your server has the right git branch with the same version as the client obtain, not the master branch!

The client is built on top of the latest stable Godot 3.4.x release. This also means that there are graphical artifacts present on certain hardware. The client is relatively demanding of hardware it runs on due to the limited dynamic lighting rendering performance of Godot 3.
However, most devices made for video-games should expect no such problems.

When Godot 4 is stable enough, the client will be upgraded and moved to Godot 4 for better 3D rendering in favour of the Vulkan API  which aims to resolve the aforementioned issues.


### Space Frontiers community & contributing
Space Frontiers now has an official brand new community [Discord server](https://discord.gg/yYpMun9CTT).


This project is oriented towards long-term development, meaning it is here to stay and to be developed for years to come.

Feedback, bug reports, suggestions and critique are very much appreciated. Github issues will be reviewed and considered.

The idea is to financially reward and/or hire people for their contributions in the future, but it is too early for that kind of money now.
It is possible to contribute in all kinds of ways and you reaching out for possibilities will be very appreciated!
Also looking for both 2D & 3D digital artists, writers and game(play) designers.
People who are genuinely interested in contributing are suggested to contact the developer on Discord, when this interest arises high priority will be put into creating project documentation, tutorial videos and releasing the GUI tools of the project for custom map and custom entity creation.

Space Frontiers allows each community of players to compile and host a server instance themselves meaning the gameplay, community moderation & servers remain sovereign.

A developer documentation website is planned.

A developer web forum is planned.
![Screenshot of Space Frontiers GUI project map and server editor](https://starwolves.io/images/sfeditorss.png)


### [StarWolves.io](https://starwolves.io)
Star Wolves is a gaming community that is pioneering the game Space Frontiers by hosting official servers for it and more.
The roots of Space Frontiers are partially in this community, but Space Frontiers is managed in a way that effectively makes Space Frontiers and its community remain a predominantly separated entity.
Star Wolves will be first community to host a 24/7 server for Space Frontiers when this project is out of pre-alpha and enters the alpha stage.
