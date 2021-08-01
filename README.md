
# Space Frontiers server

  

Pre-Alpha Stage.

  

## Description

  

A multi-threaded sci-fi community game server written in Rust with the Bevy ECS game engine. Made to exclusively communicate with a moddable client with the Godot engine.
This game server is designed to work and scale well on modern processors that have multiple CPU cores.
  
### Features
* Parallelized ECS (Entity Component System) architecture.
* High player & entity counts support for matches.
* Server-side moddable map support with sizes up to 1km by 1km with 100k+ dynamic ship cells that make up the map.
* Server-side moddable inventory support.
* Server-side console commands, including admin RCON commands to spawn in entities and the like.
* Built with the parallel [Rapier 3D Physics engine](https://rapier.rs/).
* Interpolation throttling on a per client basis to meet bandwidth usage quotas.
* Entity and netcode are expandable and dynamic to be able to interact with per server custom content for the clients.
* Fearless resource access across multiple threads.
* Doryen-FOV algorithm.

## Getting Started

  

### Dependencies

  

* [Rust](https://www.rust-lang.org/)

  

  

### Executing program

  

* To compile & run the game server:

```

cargo run --release

```

### Space Frontiers client
You need to download the official Space Frontiers client to be able to connect to yourself and inspect it. There is no public release of this client as of yet.


## Contributions
Contributions in the form of pushed code, feedback, suggestions and critique are very much appreciated.