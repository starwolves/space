# Space Frontiers pre-alpha
<img src="/data/project/sflogo.png?raw=true" data-canonical-src="/data/project/sflogo.png?raw=true" width="175" height="175"/>

<a href="https://discord.gg/yYpMun9CTT">
    <img src="https://img.shields.io/discord/942798229953716274.svg?logo=discord&colorB=7289DA">
</a>

![Continuous integration](https://github.com/starwolves/space/actions/workflows/rust.yml/badge.svg?branch=0.0.3-snap)

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg) ![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)

![Screenshot of Space Frontiers gameplay](/data/project/sfss.png?raw=true)

## Description

*Space Frontiers is an online multiplayer game in which players find themselves in a large spaceship operations crew who have to work together to run and maintain a spaceship. Crew members are assigned different roles with different levels of responsibilities and authorization. The manned spaceship will be tasked to jump from sector to sector which triggers random events; events will include player antagonists, traitor roles, alien infestations and more. Therefore player communication is key, putting emphasis on the chat with optional roleplay features. A wide variety of simulated spaceship and gameplay elements will be included to ensure these scenarios will be as fun as possible.*

The repository of a modular and moddable multi-threaded sci-fi headless community game server written in Rust with the [Bevy ECS](https://bevyengine.org/) game engine. Made to communicate exclusively with the official closed-source Space Frontiers client. Credits to [Godot Engine](https://godotengine.org/) and [godot-rust](https://github.com/godot-rust/godot-rust)!

There is an [official Discord server](https://discord.gg/yYpMun9CTT).

You can see gameplay videos of Space Frontiers on [YouTube](https://www.youtube.com/channel/UC6D7lcx9eL_ChA7HzzvhLtA).

There is also a work-in-progress [documentation](https://sf-docs.starwolves.io) available for code contributors or modders.

Check out our [Github project workspace](https://github.com/starwolves/space/projects/1) for upcoming improvements to the game!

### Features (All Moddable & Modular)
* Decentralized gameplay, each community can host their own server. 👑
* Parallelized Entity Component System architecture. 📡
* Pure Rust. No garbage collection. Fast code execution. 🌟
* Data-oriented and modular within a thread-safe and strictly compiled environment. It is easy to add and remove entities, systems, components, map cells and more simply by managing [plugins](https://bevyengine.org/learn/book/getting-started/plugins/) that will get compiled with the project. 🔭
* Using the cutting-edge [Rapier 3D Physics engine](https://rapier.rs/). 🚀
* Character meshes and animations are integrated with [Mixamo](https://www.mixamo.com/) for rigging. ☄
* Inventory system, pick up, wear, attach, place and equip items with character entities.
* Melee & projectile combat systems, damage player, ship walls or other entities with various types of damage and the ability to target specific body parts.
* Advanced bbcode chat, with support for examining entities, modular (radio) channels and proximity communication.
* Actions and tab menu's to easily interact with the world while also offering protection against cheaters.
* Configurable console commands, including rcon admin commands.
* Clients can load in custom content on a per server basis thanks to a traditional automatically shared and downloaded content folder approach.
* Godot Addressable references are used for efficient and dynamic netcode that works well with custom content.
* Cell based map support with a graphical user interface map, world and entities editor with support for sizes up to 1km by 1km with 100,000+ dynamic (de)constructable ship cells.
* Atmospherics simulation including temperature, pressure, diffusion, gravity and the vacuum of space.

![Screenshot of Space Frontiers atmospherics simulation](/data/project/sfatmosss.png?raw=true)

## Getting Started

### Dependencies



* [Rust](https://www.rust-lang.org/)
* [Bevy ECS dependencies](https://bevyengine.org/learn/book/getting-started/setup/#install-os-dependencies)
  

  

### Executing game server

  

To compile and run the game server:
* Select latest versioned branch (not master) from this repository and clone it.
* In your terminal navigate to the project folder you have just obtained and run:

```
cargo run
```

### Client
You can obtain the latest stable release of the client on [Discord](https://discord.gg/yYpMun9CTT).
Ensure your server has the right git branch with the same version as the obtained client and not the master branch!

The client is built on top of the latest stable Godot 3.4.x release. The client will be upgraded and moved to Godot 4 for better 3D rendering in favour of the Vulkan API.

## Contributing
This project is oriented towards long-term development, meaning it is here to stay and to be developed for some years to come.
Feedback, bug reports, suggestions and critique are very much appreciated. Github issues and pull requests will be reviewed and considered.

The hopes are to financially reward and/or hire the most suitable people for their contributions in the much further future.

Contributors of this project have to agree to our [Collaberative License Agreement](https://github.com/starwolves/contributor-license-agreement/blob/main/CLA). Our automated CLA assisstant will give you instructions on how to agree the first time you contribute on Github.

It is recommended to develop on the latest snapshot branch and to submit pull requests. Snapshot branches get regularly merged with the master branch. Each time a merge with the master branch occurs a new snapshot releases. To stay up to date with this process check out our [Discord](https://discord.gg/yYpMun9CTT) and our [Github project workspace](https://github.com/starwolves/space/projects/1).

![Screenshot of Space Frontiers GUI project map and content editor](/data/project/sfeditorss.png?raw=true)

## License

This repository is licensed under a [special license](https://github.com/starwolves/space/blob/master/LICENSE).

