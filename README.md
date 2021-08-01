
# Space Frontiers server

  

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)

  

## Description

  

A multi-threaded sci-fi community game server written in Rust with the [Bevy ECS](https://bevyengine.org/) game engine. Made to exclusively communicate with the official moddable Space Frontiers client made with the [Godot Engine](https://godotengine.org/).
This game server is designed to work and scale well on modern processors that have multiple CPU cores.
  
### Features
* Parallelized ECS (Entity Component System) architecture.
* High player & entity counts support for matches.
* Server-side moddable map support with sizes up to 1km by 1km with 100k+ dynamic ship cells that make up the map.
* Server-side moddable inventory support.
* Server-side moddable console commands, including rcon admin commands.
* Server-side moddable proximity and radio chat.
* Built with the parallel [Rapier 3D Physics engine](https://rapier.rs/).
* Interpolation throttling on a per client basis to meet bandwidth usage quotas.
* Entity and netcode are expandable and dynamic to be able to interact with per server custom content for the clients.
* Fearless multi-threading and resource access across multiple threads.
* A concurrent [Doryen-FOV](https://github.com/jice-nospam/doryen-fov) algorithm.

## Getting Started

  

### Dependencies

  

* [Rust](https://www.rust-lang.org/)

  

  

### Executing program

  

* To compile & run the game server:

```
cargo run --release
```

### Space Frontiers client
You need to get the official Space Frontiers client to be able to connect to the server yourself. There is currently no public release of this client.


## Contributions
Contributions in the form of pushed code, feedback, suggestions and critique are very much appreciated.
