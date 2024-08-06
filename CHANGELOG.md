# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Bevy Migration - [v0.3.0](https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.3.0) - 2024-08-06

[![v0.3.0](https://img.shields.io/badge/v0.3.0-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.3.0)](https://github.com/Fabinistere/cats_destroyer_2000/releases/tag/v0.3.0)
[![**Full Commits History**](https://img.shields.io/badge/GitHubLog-gray?style=flat&logo=github&logoColor=181717&link=https://github.com/fabinistere/cats_destroyer_2000/commits/v0.3.0)](https://github.com/fabinistere/cats_destroyer_2000/commits/v0.3.0)

- [Migration Guide Bevy 0.10 -> 0.11](https://bevyengine.org/learn/migration-guides/0.10-0.11/)
- *not needed* [Changelog Bevy Rapier 0.21 -> 0.22](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0220-10-july-2023)

### [Bevy 0.11](https://bevyengine.org/learn/migration-guides/0.10-0.11/) Migration

- ECS
  - `in_set(OnUpdate(*))` -> `run_if(in_state(*))`
  - Add the `#[derive(Event)]` macro for events.
  - Allow tuples and single plugins in `add_plugins`, deprecate `add_plugin`
  - [Schedule-First: the new and improved `add_systems`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#schedule-first-the-new-and-improved-add-systems)
- UI
  - Flatten UI Style properties that use Size + remove Size
    - The `size`, `min_size`, `max_size`, and `gap` properties have been replaced by the `width`, `height`, `min_width`, `min_height`, `max_width`, `max_height`, `row_gap`, and `column_gap` properties. Use the new properties instead.
  - [Remove `Val::Undefinded`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#remove-val-undefined)
    - `Val::Undefined` has been removed. Bevy UI’s behaviour with default values should remain the same.
    The default values of `UiRect`’s fields have been changed to `Val::Px(0.)`.
    `Style`’s position field has been removed. Its `left`, `right`, `top` and `bottom` fields have been added to `Style` directly.
    For the `size`, `margin`, `border`, and `padding` fields of `Style`, `Val::Undefined` should be replaced with `Val::Px(0.)`.
    For the `min_size`, `max_size`, `left`, `right`, `top` and `bottom` fields of `Style`, `Val::Undefined` should be replaced with `Val::Auto`
  - [Rename keys like `LAlt` to `AltLeft`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#rename-keys-like-lalt-to-altleft)
  - [Delay asset hot reloading](https://bevyengine.org/learn/migration-guides/0.10-0.11/#delay-asset-hot-reloading)
  - [`Interaction::Clicked` replaced by `Interaction::Pressed`](https://bevyengine.org/learn/migration-guides/0.10-0.11/#rename-interaction-clicked-interaction-pressed)
- Dependencies
  - bevy_rapier_2d `0.22`
  - bevy-inspector-egui `0.20`

### [Bevy 0.10](https://bevyengine.org/learn/migration-guides/0.9-0.10/) Migration

- Dependencies
  - remove the `dynamic` feature fr
  - bevy-inspector-egui `0.18`
  - bevy_rapier2d `0.21` - [changelog](https://github.com/dimforge/bevy_rapier/blob/master/CHANGELOG.md#0210--07-march-2023)
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
