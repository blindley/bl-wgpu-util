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

pub struct Renderer {
    basic_renderer: BasicRenderer,
    texture_view: wgpu::TextureView,
    linear_sampler: wgpu::Sampler,
    nearest_sampler: wgpu::Sampler,
    custom_sampler: Option<wgpu::Sampler>,
}

impl Renderer {
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        viewport: Option<Viewport>,
    ) {
        let viewport = viewport.unwrap_or(Viewport::from(view.texture().size()));

        if !viewport.area_is_positive() {
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

        let linear_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let nearest_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        basic_renderer.set_sampler(linear_sampler.clone());

        Ok(Self {
            basic_renderer,
            texture_view,
            linear_sampler,
            nearest_sampler,
            custom_sampler: None,
        })
    }

    pub fn set_linear_sampling(&mut self) {
        self.basic_renderer.set_sampler(self.linear_sampler.clone());
    }

    pub fn set_nearest_sampling(&mut self) {
        self.basic_renderer
            .set_sampler(self.nearest_sampler.clone());
    }

    pub fn set_custom_sampler(&mut self, sampler: wgpu::Sampler) {
        self.basic_renderer.set_sampler(sampler.clone());
        self.custom_sampler = Some(sampler);
    }

    pub fn set_texture(&mut self, texture: &wgpu::Texture) {
        if self.texture_view.texture() != texture {
            self.texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
            self.basic_renderer
                .set_texture_view(self.texture_view.clone());
        }
    }
}
