use crate::Viewport;
use crate::wgpu;

use super::basic_renderer::{BasicRenderer, BasicRendererDescriptor, DynamicUniformBuffer};

macro_rules! renderer_name {
    () => {
        "Solid Color Renderer"
    };
}
macro_rules! make_label {
    ($object:expr) => {
        concat!(renderer_name!(), " : ", $object)
    };
}

#[derive(encase::ShaderType)]
pub struct ColorOnlyUniform {
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

pub struct Renderer {
    basic_renderer: BasicRenderer,
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

        self.basic_renderer
            .render_bufferless(encoder, view, None, Some(viewport), 0..6);
    }

    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_conf: &wgpu::SurfaceConfiguration,
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
            Some(make_label!("BasicRenderer - Solid Color").to_string()),
            device,
            queue,
            &surface_conf.format,
            &desc,
        );

        Ok(Self { basic_renderer })
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        let uniform = ColorOnlyUniform {
            color: glam::Vec4::from(color),
        };
        self.basic_renderer.update_uniforms_bytes(&uniform.bytes());
    }
}
