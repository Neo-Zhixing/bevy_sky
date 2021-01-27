# Bevy Sky

Procedurally generated sun and skies using Rayleigh / Mie.

Shader code taking reference from https://github.com/Tw1ddle/Sky-Shader.

### Usage:

Simply add SkyPlugin to your app.
```rust
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(SkyPlugin)
        .add_startup_system(setup.system())
        .run();
}
```

### How it works

The SkyPlugin will create a new render graph node, SkyNode. This will run another runder pass,
SkyPass, in front of your MainPass. The SkyPass will render a cube, and the vertex shader will make sure
that the camera is always located at the center of your camera.

For now, Bevy Sky requires a modified version of bevy to run, because it is using wgpu 0.7
for its push constants support. The bevy engine was also modified so that the camera transform
matrix will be placed on the uniform buffer in addition to the ViewProj matrix. I will try to get
these required features merged into bevy master in the upcoming days.
