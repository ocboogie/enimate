# Enimate

Enimate is a experimental programmatic animation engine, where you can create
complex visuals using code alone. The goal is to provide a simple yet powerful
API that allows rendering complex animations, in real-time, that a traditional
animation software would struggle to create.

The idea of Enimate was inspired by [Manim](https://github.com/3b1b/manim), a
mathematical animation engine created by
[3Blue1Brown](https://www.youtube.com/@3blue1brown). Doing motion design as a
hobby, I was amazed by the power of Manim, so I wanted to create something
similar but with a different goals.

### Goals

- Flexibility: should have the ability to create nearly any animation you could
  imagine, from traditional motion design (like moving text, images, and shapes
  with easing functions, transitions, and transformations) to generative art
  and mathematical animations (like dynamic graphs, fractals, point clouds,
  etc.).

- API: should have an elegantly simple API with powerful abstractions that
  allow for complete control over the animation process, while still having
  high-level constructs like a declarative component hierarchy with animatable
  properties.

- Performance: should be able to render complex animations in real-time,
  requiring a low-level custom built renderer built on top of [wgpu](https://github.com/gfx-rs/wgpu).

- Seekable: should be able to render any frame in the animation at any time,
  without having to render the entire animation from the beginning.

### Example

```rust
fn component_animations() -> Scene {
    let mut b = SceneBuilder::new();

    let grid = b.add(Grid {
        rows: 10,
        cols: 10,
        width: 8.0,
        height: 8.0,
        material: StrokeMaterial::new(Color32::BLUE, 0.1).into(),
    });

    b.play(Wait.with_duration(0.5));
    b.play(grid.draw_out(1.5).with_easing(Easing::EaseInOut));

    b.finish()
}
```

<https://github.com/user-attachments/assets/397dabf5-cc24-4592-a30a-c19879d94ddf>
