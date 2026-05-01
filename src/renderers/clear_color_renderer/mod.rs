use crate::Viewport;
use crate::wgpu;

use super::basic_renderer::{BasicRenderer, BasicRendererDescriptor, DynamicUniformBuffer};

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

#[derive(encase::ShaderType)]
struct ColorOnlyUniform {
    pub color: glam::Vec4,
}

impl ColorOnlyUniform {
    pub fn create_uniform_buffer() -> DynamicUniformBuffer {
        use super::basic_renderer::{DynamicUniformBufferBuilder, UniformType};
        use encase::ShaderType;

        DynamicUniformBufferBuilder::new(Self::min_size())
            .with_member("color", UniformType::F32x4)
            .build()
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self).unwrap();
        buffer.into_inner()
    }
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
            uniform_buffer: Some(ColorOnlyUniform::create_uniform_buffer()),
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
        let uniform = ColorOnlyUniform { color };
        self.basic_renderer.update_uniforms_bytes(&uniform.bytes());
    }
}
