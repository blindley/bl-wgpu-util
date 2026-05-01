use crate::Viewport;
use crate::wgpu;

use super::basic_renderer::{BasicRenderer, BasicRendererDescriptor};
use super::basic_renderer::{BasicUniform, UniformRgba};

macro_rules! renderer_name {
    () => {
        "Clear Color Renderer"
    };
}
macro_rules! make_label {
    ($object:expr) => {
        concat!(renderer_name!(), " : ", $object)
    };
}

pub struct ClearColorRenderer {
    basic_renderer: BasicRenderer,
}

impl ClearColorRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: &wgpu::TextureFormat,
    ) -> anyhow::Result<Self> {
        let vertices: [f32; 12] = [
            -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];

        let vertices = bytemuck::cast_slice(&vertices).to_vec();

        let desc = BasicRendererDescriptor {
            vertex_format: super::basic_renderer::DynamicVertexDescriptorBuilder::new()
                .with_attribute("position", wgpu::VertexFormat::Float32x2, None)
                .build(),
            uniform_buffer: Some(UniformRgba::descriptor()),
            hardcoded_vertices: Some(vertices),
            ..Default::default()
        };

        let basic_renderer = BasicRenderer::new(
            Some(make_label!("BasicRenderer - Clear Color").to_string()),
            device,
            queue,
            format,
            &desc,
        );

        Ok(Self { basic_renderer })
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        viewport: Option<Viewport>,
    ) {
        self.basic_renderer
            .render_bufferless(encoder, view, None, viewport, 0..6);
    }

    pub fn set_color(&mut self, color: glam::Vec4) {
        let uniform = UniformRgba { color };
        self.basic_renderer.update_uniforms_bytes(&uniform.buffer());
    }
}
