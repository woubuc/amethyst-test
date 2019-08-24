# amethyst-test
This is a test project and contains a lot of bad code while I'm learning how to use Amethyst. __I repeat, this project is terrible. Do not use any of this in any real world situation.__

## How to run

To run the game, use

```
cargo run --features "vulkan"
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.

For building without any graphics backend, you can use

```
cargo run --features "empty"
```

but be aware that as soon as you need any rendering you won't be able to run your game when using
the `empty` feature.

## Release
Performance right now is terrible, if you want to get any kind of framerate out of the game you'll need to build with the `--release` flag.
