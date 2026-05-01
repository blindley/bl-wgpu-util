/// Represents the supported types for uniform members in WGSL.
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UniformType {
    Bool,
    U32, U32x2, U32x3, U32x4,
    I32, I32x2, I32x3, I32x4,
    F32, F32x2, F32x3, F32x4,
    Mat2, Mat3, Mat4,
    Mat2x3, Mat2x4,
    Mat3x2, Mat3x4,
    Mat4x2, Mat4x3,
}

impl UniformType {
    /// Returns the WGSL type string for this uniform type.
    pub const fn to_string(&self) -> &'static str {
        use UniformType::*;
        match self {
            Bool => "bool",
            U32 => "u32",
            U32x2 => "vec2<u32>",
            U32x3 => "vec3<u32>",
            U32x4 => "vec4<u32>",
            I32 => "i32",
            I32x2 => "vec2<i32>",
            I32x3 => "vec3<i32>",
            I32x4 => "vec4<i32>",
            F32 => "f32",
            F32x2 => "vec2<f32>",
            F32x3 => "vec3<f32>",
            F32x4 => "vec4<f32>",
            Mat2 => "mat2x2f",
            Mat3 => "mat3x3f",
            Mat4 => "mat4x4f",
            Mat2x3 => "mat2x3f",
            Mat2x4 => "mat2x4f",
            Mat3x2 => "mat3x2f",
            Mat3x4 => "mat3x4f",
            Mat4x2 => "mat4x2f",
            Mat4x3 => "mat4x3f",
        }
    }
}

/// Represents a single member in a uniform struct.
#[derive(Debug, Clone)]
struct NamedUniform {
    name: String,
    uniform_type: UniformType,
}

/// A builder for creating a [`DynamicUniformBuffer`].
pub struct DynamicUniformBufferBuilder {
    min_size: std::num::NonZeroU64,
    members: Vec<NamedUniform>,
}

impl DynamicUniformBufferBuilder {
    /// Creates a new builder with the specified minimum buffer size.
    pub fn new(min_size: std::num::NonZeroU64) -> Self {
        Self {
            min_size,
            members: Vec::new(),
        }
    }

    /// Adds a member to the uniform struct.
    ///
    /// # Panics
    /// Panics if a member with the same name already exists.
    pub fn with_member(mut self, name: &str, uniform_type: UniformType) -> Self {
        if self.members.iter().any(|m| m.name == name) {
            panic!("Duplicate uniform member name: {}", name);
        }

        self.members.push(NamedUniform {
            name: name.to_string(),
            uniform_type,
        });
        self
    }

    /// Builds the [`DynamicUniformBuffer`].
    pub fn build(self) -> DynamicUniformBuffer {
        DynamicUniformBuffer {
            min_size: self.min_size,
            members: self.members,
        }
    }
}

/// Metadata and code generation for a dynamically defined uniform struct.
///
/// This struct tracks the structure of a uniform block and can generate the
/// corresponding WGSL struct definition.
#[derive(Debug, Clone)]
pub struct DynamicUniformBuffer {
    min_size: std::num::NonZeroU64,
    members: Vec<NamedUniform>,
}

impl DynamicUniformBuffer {
    /// Returns the minimum size of the uniform buffer in bytes.
    pub fn min_size(&self) -> std::num::NonZeroU64 {
        self.min_size
    }

    /// Returns the type of the member with the specified name, if it exists.
    pub fn member_type(&self, name: &str) -> Option<UniformType> {
        self.members
            .iter()
            .find(|m| m.name == name)
            .map(|m| m.uniform_type)
    }

    pub(crate) fn transform_matrix_expr(&self) -> Option<String> {
        if let Some(ty) = self.member_type("transform_matrix") {
            use UniformType::*;
            match ty {
                Mat4 => Some("uniforms.transform_matrix".into()),
                _ => None,
            }
        } else {
            None
        }
    }

    pub(crate) fn uniform_color_expr(&self) -> Option<String> {
        if let Some(ty) = self.member_type("color") {
            use UniformType::*;
            match ty {
                F32 => Some("vec4(vec3(uniforms.color), 1.0)".into()),
                F32x2 => Some("vec4(vec3(uniforms.color.x), uniforms.color.y)".into()),
                F32x3 => Some("vec4(uniforms.color, 1.0)".into()),
                F32x4 => Some("uniforms.color".into()),
                _ => None,
            }
        } else {
            None
        }
    }

    /// Generates a WGSL struct definition for the uniform data.
    ///
    /// # Arguments
    /// * `struct_name` - The name to give the generated WGSL struct.
    pub(crate) fn code_gen_uniform_struct(&self, struct_name: &str) -> String {
        let mut s = String::new();
        s.push_str(&format!("struct {struct_name} {{\n"));

        for member in self.members.iter() {
            s.push_str(&format!(
                "    {}: {},\n",
                member.name,
                member.uniform_type.to_string()
            ));
        }

        s.push_str("}");
        s
    }
}
