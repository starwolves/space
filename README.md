<img src="/data/project/sflogo.png?raw=true" data-canonical-src="/data/project/sflogo.png?raw=true" width="175" height="175"/>

# Space Frontiers pre-alpha

*"You gotta die for something." - STARWOLF*

<a href="https://discord.gg/yYpMun9CTT">
    <img src="https://img.shields.io/discord/942798229953716274.svg?logo=discord&colorB=7289DA">
</a>

[![Chat on Matrix](https://matrix.to/img/matrix-badge.svg)](https://comms.starwolves.io)

![Continuous integration](https://gitlab.starwolves.io/starwolves/space/badges/master/pipeline.svg)

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg) ![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)

![Screenshot of Space Frontiers gameplay](/data/project/sfss.png?raw=true)

## Description

*Space Frontiers is a next-generation online community multiplayer game in which players find themselves within a galaxy consisting of multiple sectors and spaceships. Active co-operative and PVP gameplay takes place in and around manned spaceships together with a strong foundation of social community-oriented gameplay features. Developed as a sandbox, shaped by the players.*

*Space frontiers seeks to deliver an experience that offers high amounts of supported players for decentralized gaming communities that provide their own selections of (both client- and server-side) content, mods, gamemodes, gameplay, moderation and plugins. Communities will be tasked to host a cloud of servers, rather than just a single server. This is to provide reliable authorative computational power for the partially persistent Galaxy they host. Each server within the cloud represents an active sub-sector of the Galaxy, usually with one spaceship. We intend to support galaxies with more than one thousand active players in real-time.*

**[Support and play today (link).](https://store.starwolves.io/)**

**There are gameplay videos of the old prototype on [YouTube](https://youtu.be/Qa-Y_PxzeiI).**

### Technology

The repository contains both the server and client. Both are entirely written in Rust with the [Bevy ECS](https://bevyengine.org/) game engine. The technology stack is cutting-edge and will serve next-generation video-games well. Space Frontiers is designed from the ground up to support modern CPUs to efficiently take advantage of any amount of available CPU cores without increasing code complexity and without chances of data races. The old prototype client is made with Godot 3.

### Community

Socials:

* [Forum Board](https://starwolves.io)
* [Discord](https://discord.gg/yYpMun9CTT)
* [Matrix](https://matrix.to/#/#space-frontiers:comms.starwolves.io)

#### Community description

We are not ordinary, revolutionary would be a better word.
There is community sovereignty, integrity, values and strong personalities together with honest beings and experienced leaders that operate in truth and good faith.

**Space Frontiers and the community are age rated 18+ 🔞. Minors may not be present without parental approval and parental supervision.**

Brought to you by

<a href="https://starwolves.io">
<img src="/data/project/starwolveslogo_text.png?raw=true" data-canonical-src="/data/project/starwolveslogo_text.png?raw=true" width="175" height="175"/>
</a>

### Features (Including old prototype)

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
* Cell based map support with a graphical user interface map, world and entities editor with support for sizes up to 1km by 1km with 100,000+ dynamic (de)constructable ship cells.
* Atmospherics simulation including temperature, pressure, diffusion, gravity and the vacuum of space.

![Screenshot of Space Frontiers atmospherics simulation](/data/project/sfatmosss.png?raw=true)

## How to test-play (Launcher)
You can get the official game launcher with automatic updates by supporting us on [the store](https://store.starwolves.io). After payment, simply login to get your download link.

[Launcher source code](https://gitlab.starwolves.io/starwolves/launcher)

## Compile from source

### Dependencies

* [Rust](https://www.rust-lang.org/)
* [Bevy ECS dependencies](https://bevyengine.org/learn/book/getting-started/setup/#install-os-dependencies)
  
### Developing Space Frontiers

To compile Space Frontiers:

* Select latest versioned branch from this repository and clone it.
* In your terminal navigate to the project folder you have just obtained.

To start the server run:

```bash
cargo run server
```

To start the new Bevy client run:

```bash
cargo run
```

To run Space Frontiers at maximum performance with a slower compile time add the following flag to the run command:

```bash
--release
```

### Old Prototype Godot Client

You can obtain the feature-rich but old prototype client on [Discord](https://discord.gg/yYpMun9CTT).
Ensure your server has the right git branch with the same version as the obtained Godot client and not the master branch!

The old prototype client is built on top of a Godot 3.4 release.

### The new 1st person Bevy client

Currently there is full focus on recreating the client in Bevy ECS. The new Bevy client is developed open-source in this repository.

## Documentation

The technical documentation of the most recent stable snap branch is found at [docs.sf.starwolves.io](https://docs.sf.starwolves.io).

There is also a [(currently outdated) guide](https://guide.docs.sf.starwolves.io) available.

Generate documentation yourself for master (latest code and docs!!) or other versions:

```bash
cargo doc --no-deps --document-private-items --open
```

## Contributing

This project is oriented towards long-term development, meaning it is here to stay and to be developed for some years to come.
Feedback, bug reports, suggestions and critique are very much appreciated. [Gitlab](https://gitlab.starwolves.io/starwolves/space) issues and pull requests will be reviewed and considered.

You can fund the project [here](https://github.com/sponsors/starwolfy/) and get in contact with us afterwards for special and lasting roles and titles!

The hopes are to financially reward and/or hire the most suitable people for their contributions in the much further future.

Contributors of this project have to agree to our [Contributor License Agreement](https://gitlab.starwolves.io/starwolves/contributor-license-agreement). You may send your signed version to cla@starwolves.io so your associated [Gitlab](https://gitlab.starwolves.io/) account will get activated.

![Screenshot of Space Frontiers GUI project map and content editor](/data/project/sfeditorss.png?raw=true)

## License

The code of this repository is licensed under [the proprietary code license](https://gitlab.starwolves.io/starwolves/space/-/blob/master/LICENSE). The assets of this repository are licensed under the [the proprietary assets license](https://gitlab.starwolves.io/starwolves/space/-/blob/master/LICENSE_ASSETS). Both licenses we intend to support with value to authenticity and community freedoms without elements of modern day cancel culture 👑.

### AGPLv3 & CC BY-SA 4.0 Milestone

In the much further future after having obtained enough paid subscriptions: the codebase and assets for Space Frontiers at that moment and onwards will go Free Open Source (FOSS) under The GNU Affero General Public License version 3 and Attribution-ShareAlike 4.0 International License. The code base would continue to be subject to the Contributor License Agreement.
