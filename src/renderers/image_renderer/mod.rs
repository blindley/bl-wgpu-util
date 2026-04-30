use super::basic_renderer::{BasicRenderer, BasicRendererDescriptor};
use crate::Viewport;

use crate::wgpu;

macro_rules! renderer_name {
    () => {
        "Image Renderer"
    };
}
macro_rules! make_label {
    ($object:expr) => {
        concat!(renderer_name!(), " : ", $object)
    };
}

const LINEAR_SAMPLER_INDEX: usize = 0;
const NEAREST_SAMPLER_INDEX: usize = 1;

pub struct Renderer {
    basic_renderer: BasicRenderer,
    texture_view: wgpu::TextureView,
}

impl Renderer {
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        viewport: Viewport,
    ) {
        if viewport.width <= 0.0 || viewport.height <= 0.0 {
            return;
        }

        // Correct aspect ratio
        let viewport = {
            let texture_size = self.texture_view.texture().size();

            let viewport_aspect = viewport.width / viewport.height;
            let texture_aspect = texture_size.width as f32 / texture_size.height as f32;

            let viewport = if viewport_aspect > texture_aspect {
                let scale = viewport.height / texture_size.height as f32;
                let w = texture_size.width as f32 * scale;
                let h = texture_size.height as f32 * scale;
                let x = viewport.x + (viewport.width - w) / 2.0;
                let y = viewport.y;
                Viewport::new(x, y, w, h)
            } else {
                let scale = viewport.width / texture_size.width as f32;
                let w = texture_size.width as f32 * scale;
                let h = texture_size.height as f32 * scale;
                let x = viewport.x;
                let y = viewport.y + (viewport.height - h) / 2.0;
                Viewport::new(x, y, w, h)
            };

            viewport
        };

        self.basic_renderer
            .render_bufferless(encoder, view, None, Some(viewport), 0..6);
    }

    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_conf: &wgpu::SurfaceConfiguration,
        texture: &wgpu::Texture,
    ) -> anyhow::Result<Self> {
        use super::basic_renderer::DynamicVertexDescriptorBuilder;

        let vertices = {
            #[rustfmt::skip]
            let vertices: [f32; 24] = [
                -1.0, 1.0, 0.0, 0.0,
                -1.0, -1.0, 0.0, 1.0,
                1.0, -1.0, 1.0, 1.0,

                -1.0, 1.0, 0.0, 0.0,
                1.0, -1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 0.0,
            ];

            bytemuck::cast_slice(&vertices).to_vec()
        };

        let desc = BasicRendererDescriptor {
            vertex_format: DynamicVertexDescriptorBuilder::new()
                .with_attribute("position", wgpu::VertexFormat::Float32x2, None)
                .with_attribute("uv", wgpu::VertexFormat::Float32x2, None)
                .build(),
            has_texture: true,
            hardcoded_vertices: Some(vertices),
            ..Default::default()
        };

        let mut basic_renderer = BasicRenderer::new(
            Some(make_label!("BasicRenderer - Image").to_string()),
            device,
            queue,
            &surface_conf.format,
            &desc,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        basic_renderer.set_texture_view(texture_view.clone());

        Ok(Self {
            basic_renderer,
            texture_view,
        })
    }

    pub fn set_linear_sampling(&mut self) {
        self.set_sampler_index(LINEAR_SAMPLER_INDEX);
    }

    pub fn set_nearest_sampling(&mut self) {
        self.set_sampler_index(NEAREST_SAMPLER_INDEX);
    }

    pub fn set_texture(&mut self, texture: &wgpu::Texture) {
        if self.texture_view.texture() != texture {
            self.texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            self.basic_renderer
                .set_texture_view(self.texture_view.clone());
            // self.recreate_texture_bind_group();
        }
    }

    fn set_sampler_index(&mut self, _index: usize) {
        // TODO: implement sampler configuration in basic renderer

        // assert!(index < self.samplers.len());
        // if self.current_sampler_index != index {
        //     self.current_sampler_index = index;
        //     self.recreate_texture_bind_group();
        // }
        println!("set_sampler_index not implemented");
    }
}
