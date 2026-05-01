pub const UNIFORMS_BIND_GROUP_INDEX: u32 = 0;
pub const UNIFORMS_BINDING_INDEX: u32 = 0;

pub const TEXTURE_BIND_GROUP_INDEX: u32 = 1;
pub const TEXTURE_BINDING_INDEX: u32 = 0;
pub const SAMPLER_BINDING_INDEX: u32 = 1;

use super::dynamic_vertex::DynamicVertexDescriptor;
use super::texture_data::TextureData;
use super::uniform::DynamicUniformBuffer;
use super::uniform::UniformBindingData;

use crate::viewport::Viewport;

////////////////////////////////////////////////////////////////////////////////
// (public) BasicRendererDescriptor
////////////////////////////////////////////////////////////////////////////////

/// Configuration descriptor for creating a `BasicRenderer`.
///
/// This descriptor allows you to heavily customize the pipeline and shader generation
/// for the `BasicRenderer`. It defines the vertex layout, whether uniforms and textures
/// are used, depth testing configuration, and whether to use hardcoded vertices or custom shader code.
#[derive(Clone, Debug, Default)]
pub struct BasicRendererDescriptor {
    /// The layout of the vertex data. This is used to automatically generate
    /// the vertex input struct in the WGSL shader, as well as the pipeline layout.
    pub vertex_format: DynamicVertexDescriptor,
    /// If provided, a uniform buffer and bind group will be created with the given members.
    /// When specified, the generated shader will expect a `Uniforms` struct in bind group 0, binding 0.
    pub uniform_buffer: Option<DynamicUniformBuffer>,
    /// If true, the generated shader and pipeline will expect a texture in bind group 1, binding 0,
    /// and a sampler in bind group 1, binding 1. The shader will sample this texture using the `uv`
    /// attribute of the vertex format (if present).
    pub has_texture: bool,
    /// If true, the pipeline will be configured with a depth/stencil state, expecting a depth texture
    /// (Depth32Float) to be provided during rendering for depth testing.
    pub has_depth: bool,
    /// If provided, these bytes represent the actual vertex data that will be hardcoded directly
    /// into the generated WGSL shader, instead of being passed via a vertex buffer.
    pub hardcoded_vertices: Option<Vec<u8>>,
    /// If provided, this WGSL string will be used directly for the shader module, bypassing
    /// the automatic shader generation step.
    pub custom_shader: Option<String>,
}

impl BasicRendererDescriptor {
    pub fn create_shader_code(&self) -> String {
        let has_color = self.vertex_format.index_of("color").is_some();
        let has_uv = self.vertex_format.index_of("uv").is_some();

        let (has_uniforms, transform_matrix_expr, uniform_color_expr) =
            if let Some(ub) = self.uniform_buffer.as_ref() {
                (true, ub.transform_matrix_expr(), ub.uniform_color_expr())
            } else {
                (false, None, None)
            };

        let hardcoded = self.hardcoded_vertices.is_some();

        let mut code_lines: Vec<String> = Vec::new();

        macro_rules! line {
            ($s:expr) => {
                code_lines.push($s.into())
            };
            () => {
                code_lines.push(String::new())
            };
        }

        line!(self.vertex_format.code_gen_vertex_input(!hardcoded));
        line!(self.vertex_format.code_gen_vertex_output());

        if has_uniforms {
            // line!("struct Uniforms {");
            // line!("    transform_matrix: mat4x4<f32>,");
            // line!("}");

            let ub = self.uniform_buffer.as_ref().unwrap();
            line!(ub.code_gen_uniform_struct("Uniforms"));

            line!();

            line!(format!(
                "@group({g}) @binding({b}) var<uniform> uniforms: Uniforms;",
                g = UNIFORMS_BIND_GROUP_INDEX,
                b = UNIFORMS_BINDING_INDEX
            ));

            line!();
        }

        if self.has_texture {
            line!(format!(
                "@group({g}) @binding({b}) var tex: texture_2d<f32>;",
                g = TEXTURE_BIND_GROUP_INDEX,
                b = TEXTURE_BINDING_INDEX
            ));

            line!(format!(
                "@group({g}) @binding({b}) var tex_sampler: sampler;",
                g = TEXTURE_BIND_GROUP_INDEX,
                b = SAMPLER_BINDING_INDEX
            ));

            line!();
        }

        if hardcoded {
            let vertices = self.hardcoded_vertices.as_ref().unwrap();
            let vertices_str = self.vertex_format.code_gen_hardcoded_vertices(vertices);
            line!(vertices_str);
            line!();
        }

        line!("@vertex");
        if hardcoded {
            line!("fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {");
            line!("    let in = hardcoded_vertices[vertex_index];");
        } else {
            line!("fn vs_main(in: VertexInput) -> VertexOutput {");
        }
        line!("    var out: VertexOutput;");

        let matmul = if let Some(expr) = &transform_matrix_expr {
            format!("{expr} *")
        } else {
            String::new()
        };

        line!(format!(
            "    out.position = {matmul}{};",
            self.vertex_format.code_gen_position_expr()
        ));

        if has_color {
            line!(format!(
                "    out.color = {};",
                self.vertex_format.code_gen_color_expr().unwrap()
            ));
        }

        if has_uv {
            line!(format!(
                "    out.uv = {};",
                self.vertex_format.code_gen_uv_expr().unwrap()
            ));
        }

        line!("    return out;");
        line!("}");

        line!();

        line!("@fragment");
        line!("fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {");

        let mut color_exprs = Vec::new();
        if let Some(expr) = &uniform_color_expr {
            color_exprs.push(expr.to_string());
        }

        if has_color {
            color_exprs.push("in.color".to_string());
        }

        if self.has_texture {
            color_exprs.push("textureSample(tex, tex_sampler, in.uv)".to_string());
        }

        if color_exprs.is_empty() {
            color_exprs.push("vec4<f32>(1.0, 1.0, 1.0, 1.0)".to_string());
        }

        let color_expr = color_exprs.join(" * ");
        line!(format!("    return {color_expr};"));

        line!("}");

        code_lines.join("\n")
    }
}

////////////////////////////////////////////////////////////////////////////////
// (public) BasicRenderer
////////////////////////////////////////////////////////////////////////////////

/// A basic renderer that handles the creation and management of a generic WGPU render pipeline.
///
/// `BasicRenderer` encapsulates the `wgpu::RenderPipeline`, along with optional
/// uniform and texture data. It is highly customizable via the `BasicRendererDescriptor`,
/// which dictates the generated WGSL shader, vertex layout, and bind group layouts.
///
/// It provides several methods for rendering, either with or without vertex buffers,
/// and methods to update uniform data and texture views.
pub struct BasicRenderer {
    label: Option<String>,
    device: wgpu::Device,
    queue: wgpu::Queue,

    #[allow(dead_code)]
    format: wgpu::TextureFormat,
    has_depth: bool,

    pipeline: wgpu::RenderPipeline,
    vertex_stride: usize,

    uniform_data: Option<UniformBindingData>,
    texture_data: Option<TextureData>,
}

impl BasicRenderer {
    pub fn new(
        label: Option<String>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: &wgpu::TextureFormat,
        desc: &BasicRendererDescriptor,
    ) -> Self {
        let uniform_data = desc.uniform_buffer.as_ref().map(|uniform_buffer| {
            UniformBindingData::new(device, UNIFORMS_BINDING_INDEX, uniform_buffer.min_size())
        });

        let texture_data = desc
            .has_texture
            .then(|| TextureData::new(device, TEXTURE_BINDING_INDEX, SAMPLER_BINDING_INDEX));

        let pipeline = create_pipeline(
            label.clone(),
            device,
            format,
            desc,
            uniform_data.as_ref().map(|e| e.bind_group_layout()),
            texture_data.as_ref().map(|e| e.bind_group_layout()),
        );

        Self {
            label,
            device: device.clone(),
            queue: queue.clone(),
            format: *format,
            has_depth: desc.has_depth,
            pipeline,
            vertex_stride: desc.vertex_format.stride(),

            uniform_data,
            texture_data,
        }
    }

    fn _make_label(&self, suffix: &str) -> Option<String> {
        if let Some(label) = &self.label {
            Some(format!("{}.{}", label, suffix))
        } else {
            Some(suffix.to_string())
        }
    }

    fn _render_impl(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        depth_view: Option<&wgpu::TextureView>,
        viewport: Option<Viewport>,
        buffer: Option<&wgpu::Buffer>,
        range: std::ops::Range<u32>,
    ) {
        let depth_stencil_attachment = {
            if self.has_depth {
                if let Some(depth_view) = depth_view {
                    Some(wgpu::RenderPassDepthStencilAttachment {
                        view: depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    })
                } else {
                    panic!("depth_view is required if has_depth is true");
                }
            } else {
                None
            }
        };

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: self._make_label("render pass").as_deref(),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: depth_stencil_attachment,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        render_pass.set_pipeline(&self.pipeline);

        if let Some(viewport) = viewport {
            render_pass.set_viewport(
                viewport.x,
                viewport.y,
                viewport.width,
                viewport.height,
                0.0,
                1.0,
            );
        }

        if let Some(uniform_data) = self.uniform_data.as_ref() {
            render_pass.set_bind_group(UNIFORMS_BIND_GROUP_INDEX, uniform_data.bind_group(), &[]);
        }

        let mut draw = true;
        if let Some(texture_data) = self.texture_data.as_ref() {
            draw = false;
            if let Some(bg) = texture_data.bind_group() {
                render_pass.set_bind_group(TEXTURE_BIND_GROUP_INDEX, bg, &[]);
                draw = true;
            }
        }

        if let Some(buffer) = buffer {
            render_pass.set_vertex_buffer(0, buffer.slice(..));
        }

        if draw {
            render_pass.draw(range, 0..1);
        }
    }

    pub fn render_vertices(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        depth_view: Option<&wgpu::TextureView>,
        viewport: Option<Viewport>,
        vertices: &[u8],
    ) {
        use wgpu::util::DeviceExt;

        let buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: self._make_label("render_vertices.buffer").as_deref(),
                contents: vertices,
                usage: wgpu::BufferUsages::VERTEX,
            });

        let vertex_count = vertices.len() / self.vertex_stride;

        self._render_impl(
            encoder,
            view,
            depth_view,
            viewport,
            Some(&buffer),
            0..vertex_count as u32,
        );
    }

    pub fn render_buffer_range(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        depth_view: Option<&wgpu::TextureView>,
        viewport: Option<Viewport>,
        buffer: &wgpu::Buffer,
        range: std::ops::Range<u32>,
    ) {
        self._render_impl(encoder, view, depth_view, viewport, Some(buffer), range);
    }

    pub fn render_bufferless(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        depth_view: Option<&wgpu::TextureView>,
        viewport: Option<Viewport>,
        range: std::ops::Range<u32>,
    ) {
        self._render_impl(encoder, view, depth_view, viewport, None, range);
    }

    pub fn update_uniforms<U: encase::ShaderType + encase::internal::WriteInto>(
        &mut self,
        uniform: &U,
    ) {
        if let Some(uniform_data) = self.uniform_data.as_mut() {
            uniform_data.update_uniforms(&self.queue, uniform);
        }
    }

    pub fn update_uniforms_bytes(&mut self, bytes: &[u8]) {
        if let Some(uniform_data) = self.uniform_data.as_mut() {
            uniform_data.update_uniforms_bytes(&self.queue, bytes);
        }
    }

    pub fn set_texture_view(&mut self, texture_view: wgpu::TextureView) {
        if let Some(texture_data) = self.texture_data.as_mut() {
            texture_data.set_texture_view(&self.device, texture_view);
        }
    }

    pub fn set_sampler(&mut self, sampler: wgpu::Sampler) {
        if let Some(texture_data) = self.texture_data.as_mut() {
            texture_data.set_sampler(&self.device, sampler);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// (private) Utility Functions
////////////////////////////////////////////////////////////////////////////////

fn create_pipeline(
    label: Option<String>,
    device: &wgpu::Device,
    format: &wgpu::TextureFormat,
    desc: &BasicRendererDescriptor,
    uniforms_bind_group_layout: Option<&wgpu::BindGroupLayout>,
    texture_bind_group_layout: Option<&wgpu::BindGroupLayout>,
) -> wgpu::RenderPipeline {
    let shader_code = desc
        .custom_shader
        .clone()
        .unwrap_or_else(|| desc.create_shader_code());

    let make_label = |suffix: &str| -> Option<String> {
        if let Some(label) = label.clone() {
            Some(format!("{}.{}", label, suffix))
        } else {
            Some(suffix.to_string())
        }
    };

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: make_label("shader").as_deref(),
        source: wgpu::ShaderSource::Wgsl(shader_code.into()),
    });

    let layout_builder = desc.vertex_format.vertex_buffer_layout_builder();
    let vertex_layout = layout_builder.build();

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: make_label("pipeline_layout").as_deref(),

        // TODO: The order of the bind group layouts within the array indicates the
        // group index for the corresponding bind group. So the order here should
        // correspond to UNIFORM_BIND_GROUP_INDEX and TEXTURE_BIND_GROUP_INDEX,
        // rather than being explicitly ordered as they are here.
        bind_group_layouts: &[uniforms_bind_group_layout, texture_bind_group_layout],
        immediate_size: 0,
    });

    let mut vertex_layouts = Vec::new();
    if desc.hardcoded_vertices.is_none() {
        vertex_layouts.push(vertex_layout);
    }

    let depth_stencil = desc.has_depth.then(|| wgpu::DepthStencilState {
        format: wgpu::TextureFormat::Depth32Float,
        depth_write_enabled: Some(true),
        depth_compare: Some(wgpu::CompareFunction::Less),
        stencil: wgpu::StencilState::default(),
        bias: wgpu::DepthBiasState::default(),
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: make_label("pipeline").as_deref(),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: None,
            buffers: &vertex_layouts,
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: None,
            targets: &[Some(wgpu::ColorTargetState {
                format: *format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    });

    render_pipeline
}
