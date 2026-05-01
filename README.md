# bl-wgpu-util

A personal utility library for building [wgpu](https://github.com/gfx-rs/wgpu) applications in Rust.
It provides a small set of reusable building blocks—depth buffers, viewports, image/texture helpers,
and a flexible renderer abstraction—to reduce boilerplate when writing wgpu-based projects.

> **Note:** This crate is personal/experimental and not published to crates.io. APIs may change without notice.

## Features

- **`DepthBuffer`** — Creates and manages a `Depth32Float` texture with resize support.
- **`Viewport`** — A lightweight rectangle type that can be constructed from `wgpu::Extent3d` or `&wgpu::Texture` and applied directly to a `RenderPass`.
- **`image` module** — An RGBA8 CPU-side `Image` type with file/byte loading, sub-image extraction, blit operations, and one-shot `wgpu::Texture` creation.
- **`BasicRenderer`** — A configurable wgpu render pipeline wrapper with automatic WGSL shader generation.
- **`SolidColorRenderer`** — A ready-to-use renderer that fills a viewport with a solid colour.
- **`ImageRenderer`** — A ready-to-use renderer that displays a texture, automatically correcting aspect ratio.

## Modules

### `depth`

```rust
use bl_wgpu_util::DepthBuffer;

let depth_buffer = DepthBuffer::new(&device, width, height);

// resize when the window changes
depth_buffer.resize(&device, new_width, new_height);

// use the view in your render pass
let view = depth_buffer.texture_view();
```

### `viewport`

```rust
use bl_wgpu_util::Viewport;

// construct from explicit coordinates
let vp = Viewport::new(0.0, 0.0, 800.0, 600.0);

// or directly from a texture
let vp = Viewport::from(&my_texture);

// apply to a render pass
vp.apply(&mut render_pass);
```

### `image`

```rust
use bl_wgpu_util::image::{Image, load_texture};
use wgpu::TextureUsages;

// Load from a file path
let img = Image::load("assets/sprite.png")?;

// Load from embedded bytes
let img = Image::load_from_bytes(include_bytes!("assets/sprite.png"))?;

// Upload to the GPU
let texture = img.create_texture(&device, &queue, TextureUsages::TEXTURE_BINDING)?;

// Convenience one-liners
let texture = load_texture(&device, &queue, "assets/sprite.png", TextureUsages::TEXTURE_BINDING)?;
```

`Image` also supports sub-image extraction and blitting:

```rust
let sub = img.extract_sub_image(x, y, w, h);
sub.blit(&mut target_image, dst_x, dst_y);
```

### `renderers`

#### `BasicRenderer`

A general-purpose renderer configured via `BasicRendererDescriptor`. It automatically generates WGSL shader code from the descriptor but also accepts a custom shader string.

```rust
use bl_wgpu_util::renderers::basic_renderer::{
    BasicRenderer, BasicRendererDescriptor, DynamicVertexDescriptorBuilder,
};

let desc = BasicRendererDescriptor {
    vertex_format: DynamicVertexDescriptorBuilder::new()
        .with_attribute("position", wgpu::VertexFormat::Float32x2, None)
        .with_attribute("color", wgpu::VertexFormat::Float32x4, None)
        .build(),
    has_depth: true,
    ..Default::default()
};

let renderer = BasicRenderer::new(Some("My Renderer".into()), &device, &queue, &format, &desc);

// Render a slice of raw vertex bytes
renderer.render_vertices(&mut encoder, &view, Some(&depth_view), Some(viewport), &vertex_bytes);
```

Key descriptor options:

| Field | Description |
|---|---|
| `vertex_format` | Vertex layout; drives auto-generated WGSL input struct and pipeline layout |
| `uniform_buffer` | Optional `DynamicUniformBuffer`; creates bind group 0 with a `Uniforms` struct |
| `has_texture` | Expects a texture + sampler in bind group 1 |
| `has_depth` | Enables `Depth32Float` depth testing |
| `hardcoded_vertices` | Bakes vertex data directly into the generated shader |
| `custom_shader` | Bypasses auto-generation entirely |

#### `SolidColorRenderer`

Fills a viewport with a single RGBA colour.

```rust
use bl_wgpu_util::renderers::solid_color_renderer::Renderer as SolidColorRenderer;

let mut renderer = SolidColorRenderer::new(&device, &queue, &surface_config)?;
renderer.set_color([0.2, 0.4, 0.8, 1.0]);
renderer.render(&mut encoder, &view, viewport);
```

#### `ImageRenderer`

Displays a texture centred in a viewport with correct aspect ratio.

```rust
use bl_wgpu_util::renderers::image_renderer::Renderer as ImageRenderer;

let mut renderer = ImageRenderer::new(&device, &queue, &surface_config, &texture)?;
renderer.set_linear_sampling();  // or set_nearest_sampling()
renderer.render(&mut encoder, &view, viewport);

// Swap the displayed texture at any time
renderer.set_texture(&new_texture);
```

## Dependencies

| Crate | Purpose |
|---|---|
| `wgpu` | GPU API |
| `image` | Image file decoding |
| `encase` | WGSL-compatible uniform buffer layout |
| `glam` | Math types (`Vec4`, `Mat4`, …) with `encase` support |
| `bytemuck` | Safe byte casting for vertex/uniform data |
| `half` | f16 support via `bytemuck` |
| `anyhow` | Error handling |

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT License](LICENSE-MIT)

at your option.
