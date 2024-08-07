# Space Frontiers [pre-alpha]

![Continuous integration](https://gitlab.starwolves.io/starwolves/space/badges/master/pipeline.svg)

*"You gotta die for something." - STARWOLF*

<img src="/data/project/sflogo.png?raw=true" data-canonical-src="/data/project/sflogo.png?raw=true" width="175" height="175"/>

## Description

*The start to a next-generation online community multiplayer game that seeks to support a galaxy consisting of multiple sectors and spaceships. Active cooperative and PVP gameplay takes place in and around manned spaceships together with a foundation of Role-Playing-Game features. Gamemodes and content are customizable and shaped by the players.*

*Designed from the ground up to deliver an experience that offers high amounts of supported players for decentralized gaming communities that provide their own selections of (both client- and server-side) content, mods, game modes, gameplay, moderation and plugins. Communities can also choose to host a cloud of servers, rather than just a single server.*

*If Faster Than Light (FTL), System Shock and Space Station 13 had a child.*

**[Support and try today (link).](https://store.starwolves.io/)**

**There are gameplay videos of the prototype on [YouTube](https://youtu.be/u1K3T5uzebE).**

<img src="/data/project/sf_door_ss.png?raw=true" data-canonical-src="/data/project/sf_door_ss.png?raw=true" width="352" height="211"/><img src="/data/project/sf_hallway_ss.png?raw=true" data-canonical-src="/data/project/sf_hallway_ss.png?raw=true" width="352" height="211"/><img src="/data/project/sf_secondarybay_ss.png?raw=true" data-canonical-src="/data/project/sf_secondarybay_ss.png?raw=true" width="352" height="211"/><img src="/data/project/sf_old_ss.png?raw=true" data-canonical-src="/data/project/sf_old_ss.png?raw=true" width="352" height="211"/>


### Technology

The repository contains both the server and the client. Both are entirely written in Rust with the [Bevy ECS](https://bevyengine.org/) game engine. The technology stack is cutting-edge and will serve next-generation online community server videogames well. Space Frontiers is designed from the ground up to support modern CPUs to efficiently take advantage of any amount of available CPU cores without increasing code complexity and without chances of data races. The server and client have shared libraries and deterministic behavior for synchronizing physics states reducing bandwidth usage for each connection.

### Community

Socials:

* [Forum Board](https://starwolves.io)

**Space Frontiers and the community are age rated 18+ 🔞. Minors may not be present without parental approval and parental supervision.**

### Revenue sharing
The codebase is open-source. Artists and programmers are invited to become part of the development of this game.

See [financial rewards.](https://starwolves.io/thread-8.html)

See [pricing.](https://store.starwolves.io)

Brought to you by

<a href="https://starwolves.io">
<img src="/data/project/starwolveslogo_text.png?raw=true" data-canonical-src="/data/project/starwolveslogo_text.png?raw=true" width="175" height="175"/>
</a>

### Features new prototype (1st person)

* Decentralized gameplay, each community can host their own server. 👑
* Highly parallelized Entity Component System Architecture. 📡
* Pure Rust. No garbage collection. Fast code execution. 🌟
* Data-oriented and modular within a thread-safe and strictly compiled environment. It is easy to add and remove entities, systems, components, map cells and more simply by managing [plugins](https://bevyengine.org/learn/book/getting-started/plugins/) that will get compiled with the project. 🔭
* Using the cutting-edge [bevy_xpbd](https://github.com/Jondolf/bevy_xpbd). 🚀
* Low-bandwidth synchronization netcode that keeps clients at desired ticks based on their latency.
* Smooth client-side network physics prediction, rollback and corrections for low to medium latency connections.
* A modular 3D and dynamically destructible / constructible map framework. ☄
* Early inventory system implementation, equip items with character entities.
* Global chat.
* Actions and tab menus to interact with the world and entities while also offering protection against cheaters.
* Configurable console commands, including rcon admin commands.
* Clients can load custom content on a per-server basis thanks to a traditional automatically shared and downloaded content folder approach.

### Features old prototype (top-down isometric)

* Character meshes and animations are integrated with [Mixamo](https://www.mixamo.com/) for rigging.
* Inventory system, pick up, wear, attach, place and equip items with character entities.
* Melee & projectile combat systems, damage player, ship walls or other entities with various types of damage and the ability to target specific body parts.
* Advanced BBCode chat, with support for examining entities, modular (radio) channels and proximity communication.
* Cell-based map support with a graphical user interface map, world and entities editor with support for sizes up to 1km by 1km with 100,000+ dynamic (de)constructible ship cells.
* Atmospherics simulation including temperature, pressure, diffusion, gravity and the vacuum of space.

![Screenshot of old prototype atmospherics simulation](/data/project/sfatmosss.png?raw=true)

## How to test-play (Launcher)
You can get the official game launcher with automatic updates of the new prototype by supporting the project on [the store](https://store.starwolves.io). After payment, simply log in to get your download link.

Being logged in with the launcher is a requirement to try out and play this codebase.

[Launcher source code](https://gitlab.starwolves.io/starwolves/launcher)

## Compile from source

### Dependencies

* [Rust](https://www.rust-lang.org/)
* [Bevy ECS dependencies](https://bevyengine.org/learn/book/getting-started/setup/#install-os-dependencies)
  
### Developing Space Frontiers

To compile Space Frontiers:

* Select latest a versioned branch from this repository and clone it.
* In your terminal navigate to the project folder you have just obtained.

Ensure you are logged in with the [Space Frontiers launcher](https://store.starwolves.io).
You may be eligible for a free Space Frontiers account, for more information check the store page.

To start the server run:

```bash
cargo run server
```
In optimized mode:
```bash
cargo run --release server
```
To start the new Bevy client run:

```bash
cargo run
```
In optimized mode:
```bash
cargo run -- release
```

## Documentation

Generate documentation yourself for master (latest code and docs!!) or other versions:

```bash
cargo doc --no-deps --document-private-items --open
```

## Contributing

This project is oriented towards long-term development, meaning it is here to stay and to be developed for some years to come.
Feedback, bug reports, suggestions and critique are very much appreciated.

The hopes are to financially reward and/or hire the most suitable people for their contributions in the much further future.

Contributors of this project have to agree to our [Contributor License Agreement](https://gitlab.starwolves.io/starwolves/contributor-license-agreement). You may send your signed version to cla@starwolves.io so your associated [Gitlab](https://gitlab.starwolves.io/) account will get activated.

![Screenshot of Space Frontiers GUI project map and content editor](/data/project/sfeditorss.png?raw=true)

## License

The code of this repository is licensed under [the proprietary code license](https://gitlab.starwolves.io/starwolves/space/-/blob/master/LICENSE). The assets of this repository are licensed under the [the proprietary assets license](https://gitlab.starwolves.io/starwolves/space/-/blob/master/LICENSE_ASSETS). Both licenses we intend to support with value to authenticity and community freedoms without elements of modern-day cancel culture 👑.

### AGPLv3 & CC BY-SA 4.0 Milestone

In the much further future after having obtained enough paid subscriptions: the codebase and assets for Space Frontiers at that moment and onwards will go Free Open Source (FOSS) under The GNU Affero General Public License version 3 and Attribution-ShareAlike 4.0 International License. The codebase would continue to be subject to the Contributor License Agreement. This is likely to happen at the very last stage of the game when most commercial opportunities for Starwolves have already been exploited.
