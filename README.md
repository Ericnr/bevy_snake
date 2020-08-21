# Per Spatium

## About the project

This is my first game written in [Bevy Engine](https://bevyengine.org) and,
at the same time, my first "pure ECS" game created from scratch.
It is a simple scroll-down space shooter game, but I intend to add some basic progression.
This project aims to be a showcase of the `Bevy Engine` by
being an actually playable game. The name of the game means "Through Space" in latin.

## How to build

This game depends on the `Bevy Engine` and uses it's "fast build configuration".
See Bevy's documentation on how to enable it.
Also, this project uses nightly rust toolchain to further speed up the building.
However, if the "fast build configuration" is modified to stable toolchain, it should work.
I do not use other nightly features.
Otherwise, this project can be build/run using simple `cargo run --release`.

**NOTE:** The game may crash the first time it is run.
There is a bug related to font loading, but on second run, it tends to "disappear".
There is also a memory leak that happens because I had to circumvent one `Bevy` bug, see this [issue](https://github.com/bevyengine/bevy/issues/135).

## How to play

You can control the ship using `WASD` keys. There is thrust/velocity
mechanic, so even after you stop pressing the key, the ship will continue it's original direction,
but it will eventually loose speed.
(Yes, space physics nor common game logic work like this. I am aware of this and
the mechanic might get removed eventually).

When you press `Space`, ship will fire it's weapons, draining some energy. Energy replenishes with time,
but you just cannot keep `Space` pressed all the time.

Destroying incoming asteroids increases your score.

There are 3 powerups:

- *Red*: Replenishes some health.
- *Blue*: Replenishes some energy and increases energy storage.
- *Yellow*: Boost's ship speed (indefinitely, so ship speed can reach unplayability, sorry).

You can also use your ship to crush the asteroids, at the cost of ship's health.

The game gets faster with time passed.

## State of the project/code

`Bevy Engine` is a new and WIP game engine. There are many missing or unfinished features,
lack of optimizations and many bugs/quirks. I did not touched the engine code,
but I did a lot of hacks to make the game work (somewhat).
Yet, at the `Bevy`'s current development speed, I believe that it will be possible to remove those hacks soon.

Also, as this is the first time I make an ECS game from scratch, there are blank spots.
I try to make the game extensible and flexible, but I am learning in the process,
so if you are an experienced ECS game developer, do not expect miracles.
