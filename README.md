# Space Frontiers pre-alpha
<img src="/data/project/sflogo.png?raw=true" data-canonical-src="/data/project/sflogo.png?raw=true" width="175" height="175"/>

*"You gotta die for something." - STARWOLF*

<a href="https://discord.gg/yYpMun9CTT">
    <img src="https://img.shields.io/discord/942798229953716274.svg?logo=discord&colorB=7289DA">
</a>

![Continuous integration](https://gitlab.starwolves.io/starwolves/space/badges/master/pipeline.svg)

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg) ![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)

![Screenshot of Space Frontiers gameplay](/data/project/sfss.png?raw=true)

## Description

*Space Frontiers is an online multiplayer game in which players find themselves in a large spaceship operations crew who have to work together to run and maintain a spaceship. Crew members are assigned different roles with different levels of responsibilities and authorization. The manned spaceship will be tasked to jump from sector to sector which triggers random events; events will include player antagonists, traitor roles, alien infestations and more. Therefore player communication is key, putting emphasis on the chat with optional roleplay features. A wide variety of simulated spaceship and gameplay elements will be included to ensure these scenarios will be as fun as possible.*

The repository of a modular and moddable multi-threaded sci-fi headless community game server written in Rust with the [Bevy ECS](https://bevyengine.org/) game engine. The prototype client it communicates with is made with Godot as the official closed-source prototype Space Frontiers client. Credits to [Godot Engine](https://godotengine.org/) and [godot-rust](https://github.com/godot-rust/godot-rust)!

You can see gameplay videos of Space Frontiers on [YouTube](https://youtu.be/Qa-Y_PxzeiI).

### Community & Organization
Socials:
* [Discord](https://discord.gg/yYpMun9CTT)
* [Matrix](https://matrix.to/#/#space-frontiers:comms.starwolves.io)

We are by no means ordinary, revolutionary would be a better word.
At the top there is integrity, values and strong personalities together with honest beings and leaders that operate in truth and good faith.


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



## Documentation:
The technical documentation of this project is found at [docs.sf.starwolves.io](https://docs.sf.starwolves.io).

There is also a [(currently outdated) guide](https://guide.docs.sf.starwolves.io) available for code contributors.


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

To run the game server, but optimized:
```
cargo run --release
```

### Prototype Godot Client
You can obtain the prototype client on [Discord](https://discord.gg/yYpMun9CTT).
Ensure your server has the right git branch with the same version as the obtained client and not the master branch!

The prototype client is built on top of a Godot 3.4 release.

### The new Bevy client
Currently there is full focus on recreating the client in Bevy ECS. This is the main priority and we are literally just getting started so it is being made from scratch and it will be a while before it is of any use. The client is going to be integrated in this same repository. 

## Contributing
This project is oriented towards long-term development, meaning it is here to stay and to be developed for some years to come.
Feedback, bug reports, suggestions and critique are very much appreciated. [Gitlab](https://gitlab.starwolves.io/starwolves/space) issues and pull requests will be reviewed and considered.

The hopes are to financially reward and/or hire the most suitable people for their contributions in the much further future.

Contributors of this project have to agree to our [Collaberative License Agreement](https://gitlab.starwolves.io/starwolves/contributor-license-agreement). You may send your signed version to cla@starwolves.io so your associated [Gitlab](https://gitlab.starwolves.io/) account will get activated.


![Screenshot of Space Frontiers GUI project map and content editor](/data/project/sfeditorss.png?raw=true)

## License

This repository is licensed under a [restrictive proprietary license](https://gitlab.starwolves.io/starwolves/space/-/blob/master/LICENSE) that we intend to support with good faith and integrity to both grant and support western freedoms for individual communities and thus without elements of modern day cancel culture 👑.

### AGPLv3 Milestone

In the further future when a yet to be-decided amount of copies have been sold: the entire codebase will go Free Open Source (FOSS) under The GNU Affero General Public License version 3 and Attribution-ShareAlike 4.0 International.
