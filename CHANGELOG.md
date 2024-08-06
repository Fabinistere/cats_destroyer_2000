# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Bevy Migration - [v0.3.0](https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.3.0) - 2024-08-06

[![v0.3.0](https://img.shields.io/badge/v0.3.0-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.3.0)](https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.3.0)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/fabinistere/cats_destroyer_2000/commits/v0.3.0)](https://github.com/fabinistere/cats_destroyer_2000/commits/v0.3.0)

### [Bevy 0.10](https://bevyengine.org/learn/migration-guides/0.9-0.10/) Migration

- Dependencies
  - remove the `dynamic` feature fr
  - bevy-inspector-egui 0.18
  - bevy_rapier2d [0.21](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0210--07-march-2023)
    - feature `debug-render` change to `debug-render-2d`
- ECS
  - [Migrate engine to Schedule v3 (stageless)](https://bevyengine.org/learn/migration-guides/0.9-0.10/#migrate-engine-to-schedule-v3-stageless)
  - [System sets (Bevy 0.9)](https://bevyengine.org/learn/migration-guides/0.9-0.10/#system-sets-bevy-0-9)
  - [States](https://bevyengine.org/learn/migration-guides/0.9-0.10/#states)
  - Replace `RemovedComponents<T>` backing with `Events<Entity>`
- UI
  - [Windows as Entities](https://bevyengine.org/learn/migration-guides/0.9-0.10/#windows-as-entities)

## Final Cinematic - [v0.2](https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.2) - 2023-06-11

[![v0.2](https://img.shields.io/badge/v0.2-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.2)](https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.2)

## Add

- End cinematic
- WebAssembly

## Must Have

- Back to jail (when touched by a enemy)

## Should Have

- UI for the Tablet
- Vision Feature
  - Tile per Tile
  - Can only interact with seen entities.
  - Entities can be seen by camera, any mindControled entity (including the player)
- Music
- SFX
- Start Menu

## Level 1000 - [v0.1](https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.1) - 2023-02-12

[![v0.1](https://img.shields.io/badge/v0.1-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.1)](https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.1)

## Currently Have

- NPC
  - triggered by player if around (and in the same area)
  - Movement
- Player
  - Basic Movement
- Tablet
  - MindControl
    - Take the body of another cat
    - Stun after a short time after mindctrl
  - Hack
    - Open One door
    - Can't Hack while mindcontrol
- Map
  - Hitbox
  - Doors
    - Closed
  - Physical Button
    - which opens front and exit doors

## Must Have

- Start/End cinematic
- Back to jail (when touched by a enemy)
- Web Exe

## Should Have

- UI for the Tablet
- Vision Feature
  - Tile per Tile
  - Can only interact with seen entities.
  - Entities can be seen by camera, any mindControled entity (including the player)
- Music
- SFX
- Start Menu

## Blue Cat Flex - [v0.0](https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.0) - 2023-02-03

[![v0.0](https://img.shields.io/badge/v0.0-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.0)](https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.0)

- Simple Animation
- Blue Cat Flexing in the center

![fast_blue_cat](https://user-images.githubusercontent.com/73140258/216720606-6e8f7768-3170-4956-a5d1-5124741783aa.gif)
