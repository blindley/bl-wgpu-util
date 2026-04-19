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
    device: wgpu::Device,
    pipeline: wgpu::RenderPipeline,
    texture_view: wgpu::TextureView,
    samplers: [wgpu::Sampler; 2],
    current_sampler_index: usize,
    texture_bind_group: wgpu::BindGroup,
    texture_bind_group_layout: wgpu::BindGroupLayout,
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

        // Correct aspect ratio
        {
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

            viewport.apply(&mut render_pass);
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }

    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        surface_conf: &wgpu::SurfaceConfiguration,
        texture: &wgpu::Texture,
    ) -> anyhow::Result<Self> {
        let texture_bind_group_layout = create_texture_bind_group_layout(device);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let linear_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(make_label!("Linear Sampler")),
            mag_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let nearest_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(make_label!("Nearest Sampler")),
            mag_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let samplers = [linear_sampler, nearest_sampler];

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(make_label!("Texture Bind Group")),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&samplers[LINEAR_SAMPLER_INDEX]),
                },
            ],
        });

        let pipeline = create_pipeline(device, queue, surface_conf, &texture_bind_group_layout)?;

        Ok(Self {
            device: device.clone(),
            pipeline,
            texture_view,
            samplers,
            current_sampler_index: LINEAR_SAMPLER_INDEX,
            texture_bind_group,
            texture_bind_group_layout,
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
            self.recreate_texture_bind_group();
        }
    }

    fn recreate_texture_bind_group(&mut self) {
        self.texture_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(make_label!("Texture Bind Group")),
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(
                        &self.samplers[self.current_sampler_index],
                    ),
                },
            ],
        });
    }

    fn set_sampler_index(&mut self, index: usize) {
        assert!(index < self.samplers.len());
        if self.current_sampler_index != index {
            self.current_sampler_index = index;
            self.recreate_texture_bind_group();
        }
    }
}

fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(make_label!("Texture Bind Group Layout")),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

fn create_pipeline(
    device: &wgpu::Device,
    _queue: &wgpu::Queue,
    surface_conf: &wgpu::SurfaceConfiguration,
    texture_bind_group_layout: &wgpu::BindGroupLayout,
) -> anyhow::Result<wgpu::RenderPipeline> {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(make_label!("Shader Module")),
        source: wgpu::ShaderSource::Wgsl(include_str!("image_renderer.wgsl").into()),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(make_label!("Pipeline Layout")),
        bind_group_layouts: &[Some(texture_bind_group_layout)],
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
