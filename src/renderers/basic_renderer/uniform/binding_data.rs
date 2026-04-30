/// Manages a uniform buffer, its bind group layout, and the bind group itself.
///
/// This struct simplifies the process of creating and updating uniform data for shaders.
pub struct UniformBindingData {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}

impl UniformBindingData {
    /// Creates a new `UniformBindingData` with the specified binding index and minimum buffer size.
    ///
    /// # Arguments
    /// * `device` - The wgpu device used to create resources.
    /// * `binding_index` - The binding index in the shader (e.g., `@group(N) @binding(binding_index)`).
    /// * `min_size` - The minimum size of the uniform buffer in bytes.
    pub fn new(device: &wgpu::Device, binding_index: u32, min_size: std::num::NonZeroU64) -> Self {
        let bind_group_layout =
            create_uniforms_bind_group_layout(device, binding_index, Some(min_size));
        let buffer = create_uniforms_buffer(device, min_size);
        let bind_group =
            create_uniforms_bind_group(device, binding_index, &bind_group_layout, &buffer);

        Self {
            bind_group_layout,
            bind_group,
            buffer,
        }
    }

    /// Returns a reference to the wgpu bind group.
    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    /// Returns a reference to the wgpu bind group layout.
    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    /// Updates the uniform buffer with new data.
    ///
    /// This method uses the `encase` crate to handle correct alignment and padding
    /// for the uniform data according to WGSL specifications.
    ///
    /// # Arguments
    /// * `queue` - The wgpu queue used to write data to the buffer.
    /// * `uniform` - The data to write to the uniform buffer. Must implement `ShaderType` and `WriteInto`.
    pub fn update_uniforms<U: encase::ShaderType + encase::internal::WriteInto>(
        &mut self,
        queue: &wgpu::Queue,
        uniform: &U,
    ) {
        use encase::UniformBuffer;
        let mut staging = UniformBuffer::new(Vec::new());
        staging.write(uniform).unwrap();
        queue.write_buffer(&self.buffer, 0, staging.into_inner().as_slice());
    }
}

/// Creates a bind group layout for a uniform buffer.
fn create_uniforms_bind_group_layout(
    device: &wgpu::Device,
    binding_index: u32,
    min_size: Option<std::num::NonZeroU64>,
) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("uniforms bind group layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: binding_index,
            visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: min_size,
            },
            count: None,
        }],
    })
}

/// Creates a wgpu buffer for uniform data.
fn create_uniforms_buffer(device: &wgpu::Device, min_size: std::num::NonZeroU64) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("uniforms buffer"),
        size: min_size.get(),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

/// Creates a bind group for a uniform buffer.
fn create_uniforms_bind_group(
    device: &wgpu::Device,
    binding_index: u32,
    bind_group_layout: &wgpu::BindGroupLayout,
    buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("uniforms bind group"),
        layout: bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: binding_index,
            resource: buffer.as_entire_binding(),
        }],
    })
}

