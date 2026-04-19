use crate::Viewport;
use crate::wgpu;

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

pub struct Renderer {
    queue: wgpu::Queue,
    pipeline: wgpu::RenderPipeline,
    uniform_bind_group: wgpu::BindGroup,
    uniform_buffer: wgpu::Buffer,
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

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(make_label!("Render Pass")),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
        });

        viewport.apply(&mut render_pass);
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }

    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_conf: &wgpu::SurfaceConfiguration,
    ) -> anyhow::Result<Self> {
        let uniform_bind_group_layout = create_uniform_bind_group_layout(device);
        let pipeline = create_pipeline(device, queue, surface_conf, &uniform_bind_group_layout)?;

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(make_label!("Uniform Buffer")),
            size: std::mem::size_of::<Viewport>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(make_label!("Uniform Bind Group")),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Ok(Self {
            queue: queue.clone(),
            pipeline,
            uniform_bind_group,
            uniform_buffer,
        })
    }

    pub fn set_color(&mut self, color: [f32; 4]) {
        self.queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[color]));
    }
}

fn create_uniform_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(make_label!("Uniform Bind Group Layout")),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    })
}

fn create_pipeline(
    device: &wgpu::Device,
    _queue: &wgpu::Queue,
    surface_conf: &wgpu::SurfaceConfiguration,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
) -> anyhow::Result<wgpu::RenderPipeline> {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(make_label!("Shader Module")),
        source: wgpu::ShaderSource::Wgsl(include_str!("solid_color_renderer.wgsl").into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(make_label!("Pipeline Layout")),
        bind_group_layouts: &[Some(uniform_bind_group_layout)],
        immediate_size: 0,
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(make_label!("Render Pipeline")),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: None,
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: None,
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_conf.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
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
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache: None,
    });

    Ok(pipeline)
}
