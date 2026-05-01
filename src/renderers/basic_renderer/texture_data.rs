pub struct TextureData {
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    texture_view: Option<wgpu::TextureView>,
    bind_group: Option<wgpu::BindGroup>,

    texture_binding_index: u32,
    sampler_binding_index: u32,
}

impl TextureData {
    pub fn new(
        device: &wgpu::Device,
        texture_binding_index: u32,
        sampler_binding_index: u32,
    ) -> Self {
        let bind_group_layout =
            create_texture_bind_group_layout(device, texture_binding_index, sampler_binding_index);
        let sampler = create_texture_sampler(device);

        Self {
            bind_group_layout,
            sampler,
            texture_view: None,
            bind_group: None,
            texture_binding_index,
            sampler_binding_index,
        }
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &Option<wgpu::BindGroup> {
        &self.bind_group
    }

    fn _recreate_bind_group(&mut self, device: &wgpu::Device) {
        if let Some(texture_view) = self.texture_view.as_ref() {
            self.bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("texture bind group"),
                layout: &self.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: self.texture_binding_index,
                        resource: wgpu::BindingResource::TextureView(texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: self.sampler_binding_index,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                ],
            }));
        }
    }

    pub fn set_texture_view(&mut self, device: &wgpu::Device, texture_view: wgpu::TextureView) {
        if let Some(old_texture_view) = self.texture_view.as_ref() {
            if old_texture_view.texture() == texture_view.texture() {
                return;
            }
        }

        self.texture_view = Some(texture_view.clone());
        self._recreate_bind_group(device);
    }

    pub fn set_sampler(&mut self, device: &wgpu::Device, sampler: wgpu::Sampler) {
        if self.sampler == sampler {
            return;
        }

        self.sampler = sampler.clone();
        self._recreate_bind_group(device);
    }
}

fn create_texture_bind_group_layout(
    device: &wgpu::Device,
    texture_binding_index: u32,
    sampler_binding_index: u32,
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("texture bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: texture_binding_index,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: sampler_binding_index,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    })
}

fn create_texture_sampler(device: &wgpu::Device) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("texture sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::MipmapFilterMode::Nearest,
        ..Default::default()
    })
}
