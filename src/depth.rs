pub struct DepthBuffer {
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
}

impl DepthBuffer {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let texture = create_depth_texture(device, width, height);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        Self {
            texture,
            texture_view,
        }
    }

    pub fn texture_view(&self) -> &wgpu::TextureView {
        &self.texture_view
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.texture = create_depth_texture(device, width, height);
        self.texture_view = self
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
    }

    /// You can call this directly, or use the depth_stencil_attachment()
    /// method to create a depth stencil attachment for use in another
    /// render pass.
    pub fn clear(&mut self, encoder: &mut wgpu::CommandEncoder) {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("depth clear pass"),
            color_attachments: &[],
            depth_stencil_attachment: Some(self.depth_stencil_attachment()),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }

    fn depth_stencil_attachment(&self) -> wgpu::RenderPassDepthStencilAttachment<'_> {
        wgpu::RenderPassDepthStencilAttachment {
            view: &self.texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        }
    }
}

pub fn create_depth_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    let size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth texture"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });
    texture
}
