# Section 3: Project: Game Prototype with Rusty Engine

## Lesson 26: Project Overview

This section 3 is divided into 2 parts. Both parts will make use of [Rusty Engine](https://github.com/CleanCut/rusty_engine), an open source game engine around [Bevy](https://bevyengine.org/).
- Part 1 will be the tutorial on how to use Rusty Engine.
- Part 2 will run through the game scenarios.

## Lesson 27: Configuration

- First let make a new project with Cargo
```shell
cargo new project-game-prototype-with-rusty-engine
```
- then add `rusty_engine` dependencies.
- at this time, we can run `cargo run` and it will install all dependencies.
- next to download assets and the recommended way is execute below command inside your project directory
```shell
curl -L https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz | tar -zxv --strip-components=1 rusty_engine-main/assets
```

