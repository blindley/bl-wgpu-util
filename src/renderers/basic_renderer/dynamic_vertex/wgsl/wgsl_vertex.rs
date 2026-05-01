use super::super::vertex::DynamicVertex;
use super::super::vertex::DynamicVertexDescriptor;
use super::wgsl_attribute::{WgslAttribute, WgslAttributeType};

////////////////////////////////////////////////////////////////////////////////
// (private) WgslVertexDescriptor
////////////////////////////////////////////////////////////////////////////////

/// Attaches a name to an attribute type for placement in a vertex struct.
#[derive(Debug, Clone, PartialEq)]
struct NamedWgslAttributeDescriptor {
    name: String,
    attribute: WgslAttributeType,
}

////////////////////////////////////////////////////////////////////////////////
// WgslVertexDescriptor
////////////////////////////////////////////////////////////////////////////////

/// Describes a wgsl vertex layout. Location bindings are implicitly assigned
/// based on the order of attributes.
#[derive(Debug, Clone, PartialEq)]
pub struct WgslVertexDescriptor {
    attributes: Vec<NamedWgslAttributeDescriptor>,
}

impl WgslVertexDescriptor {
    /// Returns the index of the attribute with the given name.
    pub fn index_of(&self, attr_name: &str) -> Option<usize> {
        self.attributes
            .iter()
            .position(|attr| attr.name == attr_name)
    }

    /// Generates the WGSL code for the vertex input struct
    pub fn code_gen_vertex_input(&self, with_locations: bool) -> String {
        let mut s = String::new();
        s.push_str("struct VertexInput {\n");

        let loc_str = |loc: usize| {
            if with_locations {
                format!("@location({}) ", loc)
            } else {
                String::new()
            }
        };

        for (i, attr) in self.attributes.iter().enumerate() {
            s.push_str(&format!(
                "    {}{}: {},\n",
                loc_str(i),
                attr.name,
                attr.attribute.to_string()
            ));
        }

        s.push_str("}\n");
        s
    }

    /// Generates the WGSL code for the vertex output struct
    pub fn code_gen_vertex_output(&self) -> String {
        let mut s = String::new();
        s.push_str("struct VertexOutput {\n");
        s.push_str("    @builtin(position) position: vec4<f32>,\n");

        if self.index_of("color").is_some() {
            s.push_str(&format!("    @location(0) color: vec4<f32>,\n"));
        }

        if self.index_of("uv").is_some() {
            s.push_str(&format!("    @location(1) uv: vec2<f32>,\n"));
        }

        s.push_str("}\n");
        s
    }

    /// Generates the WGSL code for the position expression for output from
    /// the vertex shader. Does not generate the whole statement, as it may
    /// need to be combined with a transform matrix, for example.
    pub fn code_gen_position_expr(&self) -> String {
        let default = "vec4<f32>(0.0, 0.0, 0.0, 1.0)";
        if let Some(index) = self.index_of("position") {
            use WgslAttributeType::*;
            match self.attributes[index].attribute {
                F32 => "vec4<f32>(in.position, 0.0, 0.0, 1.0)",
                F32x2 => "vec4<f32>(in.position, 0.0, 1.0)",
                F32x3 => "vec4<f32>(in.position, 1.0)",
                F32x4 => "in.position",
                _ => default,
            }
        } else {
            default
        }
        .to_string()
    }

    /// Generates the WGSL code for the color expression for output from
    /// the vertex shader.
    pub fn code_gen_color_expr(&self) -> Option<String> {
        if let Some(index) = self.index_of("color") {
            use WgslAttributeType::*;
            match self.attributes[index].attribute {
                F32 => Some("vec4<f32>(vec3(in.color), 1.0)".to_string()),
                F32x2 => Some("vec4<f32>(vec3(in.color.x), in.color.y)".to_string()),
                F32x3 => Some("vec4<f32>(in.color, 1.0)".to_string()),
                F32x4 => Some("in.color".to_string()),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Generates the WGSL code for the uv expression for output from
    /// the vertex shader.
    pub fn code_gen_uv_expr(&self) -> Option<String> {
        if let Some(index) = self.index_of("uv") {
            use WgslAttributeType::*;
            match self.attributes[index].attribute {
                F32 => Some("vec2(in.uv)".to_string()),
                F32x2 => Some("in.uv".to_string()),
                F32x3 | F32x4 => Some("in.uv.xy".to_string()),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl From<DynamicVertexDescriptor> for WgslVertexDescriptor {
    fn from(value: DynamicVertexDescriptor) -> Self {
        let attributes = value
            .attributes()
            .into_iter()
            .map(|attr| NamedWgslAttributeDescriptor {
                name: attr.name.clone(),
                attribute: attr.format.into(),
            })
            .collect();

        Self { attributes }
    }
}

////////////////////////////////////////////////////////////////////////////////
// WgslVertex
////////////////////////////////////////////////////////////////////////////////

/// This exists mainly for the purpose of embedding hardcoded vertices in a
/// shader.
pub struct WgslVertex {
    attributes: Vec<WgslAttribute>,
}

impl std::fmt::Display for WgslVertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VertexInput(")?;

        let s = self
            .attributes
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", s)?;

        write!(f, ")")?;
        Ok(())
    }
}

/// Used to create a WGSL vertex from a dynamic vertex. Converts the attribute
/// storage type to the corresponding wgsl type. (i.e. 8, 16, and 64-bit attributes
/// get converted into 32-bit, and normalized integer attributes get converted
/// to f32.)
impl From<DynamicVertex> for WgslVertex {
    fn from(value: DynamicVertex) -> Self {
        let attributes = value
            .attributes
            .into_iter()
            .map(|attr| attr.into())
            .collect();
        Self { attributes }
    }
}
