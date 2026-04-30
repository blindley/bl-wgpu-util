use super::attribute::{DynamicAttribute, vertex_format_align_of};
use super::wgsl::wgsl_vertex::WgslVertex;
use wgpu::VertexFormat;

////////////////////////////////////////////////////////////////////////////////
// VertexBufferLayoutBuilder
////////////////////////////////////////////////////////////////////////////////

/// Used to build a `wgpu::VertexBufferLayout` from a `DynamicVertexDescriptor`.
///
/// This builder manages the lifetime of the `wgpu::VertexAttribute` array,
/// which is required because `wgpu::VertexBufferLayout` holds a reference to it.
pub struct VertexBufferLayoutBuilder {
    array_stride: u64,
    attributes: Vec<wgpu::VertexAttribute>,
}

impl VertexBufferLayoutBuilder {
    /// Builds the `wgpu::VertexBufferLayout`. The returned layout has a lifetime
    /// tied to this builder.
    pub fn build(&self) -> wgpu::VertexBufferLayout<'_> {
        wgpu::VertexBufferLayout {
            array_stride: self.array_stride,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &self.attributes,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// NamedAttributeDescriptor
////////////////////////////////////////////////////////////////////////////////

/// Describes a single attribute within a vertex layout, including its name, format, and byte offset.
#[derive(Debug, Clone)]
pub struct NamedAttributeDescriptor {
    /// The name of the attribute (e.g., "position", "color", "uv").
    pub name: String,
    /// The WGPU format of the attribute data.
    pub format: VertexFormat,
    /// The byte offset of this attribute from the start of the vertex.
    pub offset: usize,
}

////////////////////////////////////////////////////////////////////////////////
// DynamicVertexDescriptor
////////////////////////////////////////////////////////////////////////////////

/// A descriptor that defines the layout of a vertex buffer.
///
/// Any attributes can be specified, but there are three "special" attributes: `position`, `color`, and `uv`.
/// Attributes with these names will automatically be used appropriately by the default shader code:
///
/// * **`position`**: Mapped to the clip position in the vertex shader output.
///   Missing components (x, y, z) default to 0.0, and w defaults to 1.0.
/// * **`color`**: Mapped to the vertex color. Supports f32 (grayscale), vec2 (grayscale+alpha),
///   vec3 (RGB), and vec4 (RGBA).
/// * **`uv`**: Mapped to texture coordinates. Mapped (x, y) -> (u, v); extra components are discarded.
///
/// Any other attributes will be passed to the vertex shader but may require a custom shader to be useful.
#[derive(Debug, Clone, Default)]
pub struct DynamicVertexDescriptor {
    /// The total size of a single vertex in bytes.
    pub stride: usize,
    /// The required alignment for this vertex structure.
    pub _align: usize,
    /// The list of attributes that make up the vertex.
    pub attributes: Vec<NamedAttributeDescriptor>,
}

impl DynamicVertexDescriptor {
    /// Finds the index of the first attribute with the given name.
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.attributes.iter().position(|a| a.name == name)
    }

    /// Returns a VertexBufferLayoutBuilder that can be used to build a wgpu::VertexBufferLayout.
    ///
    /// You need to do it this way instead of returning a wgpu::VertexBufferLayout directly
    /// because wgpu::VertexBufferLayout contains a reference to an array of attributes,
    /// so it has a lifetime tied to the lifetime of the attributes array. This attribute
    /// array is stored in VertexBufferLayoutBuilder
    pub fn vertex_buffer_layout_builder(&self) -> VertexBufferLayoutBuilder {
        let attributes = self
            .attributes
            .iter()
            .enumerate()
            .map(|(loc, attr)| {
                let result = wgpu::VertexAttribute {
                    shader_location: loc as u32,
                    format: attr.format,
                    offset: attr.offset as u64,
                };
                result
            })
            .collect::<Vec<_>>();

        VertexBufferLayoutBuilder {
            array_stride: self.stride as u64,
            attributes,
        }
    }

    pub fn code_gen_vertex_input(&self, with_locations: bool) -> String {
        // TODO: this is so inefficient...
        use super::wgsl::wgsl_vertex::WgslVertexDescriptor;
        let w = WgslVertexDescriptor::from(self.clone());
        w.code_gen_vertex_input(with_locations)
    }

    pub fn code_gen_vertex_output(&self) -> String {
        // TODO: this is so inefficient...
        use super::wgsl::wgsl_vertex::WgslVertexDescriptor;
        let w = WgslVertexDescriptor::from(self.clone());
        w.code_gen_vertex_output()
    }

    pub fn code_gen_position_expr(&self) -> String {
        // TODO: this is so inefficient...
        use super::wgsl::wgsl_vertex::WgslVertexDescriptor;
        let w = WgslVertexDescriptor::from(self.clone());
        w.code_gen_position_expr()
    }

    pub fn code_gen_color_expr(&self) -> String {
        // TODO: this is so inefficient...
        use super::wgsl::wgsl_vertex::WgslVertexDescriptor;
        let w = WgslVertexDescriptor::from(self.clone());
        w.code_gen_color_expr()
    }

    pub fn code_gen_uv_expr(&self) -> Option<String> {
        // TODO: this is so inefficient...
        use super::wgsl::wgsl_vertex::WgslVertexDescriptor;
        let w = WgslVertexDescriptor::from(self.clone());
        w.code_gen_uv_expr()
    }

    pub fn code_gen_hardcoded_vertices(&self, vertex_buffer: &[u8]) -> String {
        let mut s = String::new();

        let vertex_count = vertex_buffer.len() / self.stride;
        s.push_str(&format!(
            "const hardcoded_vertices = array<VertexInput, {vertex_count}>(\n"
        ));

        for chunk in vertex_buffer.chunks_exact(self.stride) {
            let v = DynamicVertex::read(self, chunk);
            let w = WgslVertex::from(v);
            s.push_str(&format!("    {},\n", w.to_string()));
        }

        s.push_str(");");
        s
    }
}

////////////////////////////////////////////////////////////////////////////////
// DynamicVertex
////////////////////////////////////////////////////////////////////////////////

/// Represents a single vertex read into memory, containing a list of dynamic attributes.
pub struct DynamicVertex {
    /// The actual attribute values.
    pub attributes: Vec<DynamicAttribute>,
}

impl DynamicVertex {
    pub fn read(desc: &DynamicVertexDescriptor, buffer: &[u8]) -> Self {
        assert!(buffer.len() >= desc.stride);
        let mut attributes = Vec::new();
        for attr_desc in &desc.attributes {
            let offset = attr_desc.offset;
            let attr = DynamicAttribute::read(attr_desc.format, &buffer[offset..]);
            attributes.push(attr)
        }

        DynamicVertex { attributes }
    }
}

////////////////////////////////////////////////////////////////////////////////
// DynamicVertexDescriptorBuilder
////////////////////////////////////////////////////////////////////////////////

/// A builder for creating a `DynamicVertexDescriptor` with automatic offset and alignment calculation.
pub struct DynamicVertexDescriptorBuilder {
    stride: Option<usize>,
    align: usize,
    attributes: Vec<NamedAttributeDescriptor>,
    next_offset: usize,
}

impl DynamicVertexDescriptorBuilder {
    pub fn new() -> Self {
        Self {
            stride: None,
            align: 1,
            attributes: Vec::new(),
            next_offset: 0,
        }
    }

    pub fn build(self) -> DynamicVertexDescriptor {
        assert!(!self.attributes.is_empty());

        let stride = self
            .stride
            .unwrap_or_else(|| (self.next_offset + self.align - 1) / self.align * self.align);

        assert_eq!(0, stride % self.align);

        DynamicVertexDescriptor {
            stride,
            _align: self.align,
            attributes: self.attributes,
        }
    }

    pub fn with_stride(mut self, stride: usize) -> Self {
        self.stride = Some(stride);
        self
    }

    pub fn with_attribute(
        mut self,
        name: impl Into<String>,
        format: VertexFormat,
        offset: Option<usize>,
    ) -> Self {
        let align = vertex_format_align_of(format);
        let align_min_offset = (self.next_offset + align - 1) / align * align;
        let offset = offset.unwrap_or(align_min_offset);

        assert!(offset >= self.next_offset);
        assert_eq!(0, offset % align);

        self.attributes.push(NamedAttributeDescriptor {
            name: name.into(),
            format,
            offset,
        });

        self.next_offset = offset + format.size() as usize;
        self.align = self.align.max(align);
        self
    }
}
