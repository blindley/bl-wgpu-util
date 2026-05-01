use super::{DynamicUniformBuffer, DynamicUniformBufferBuilder, UniformType};
use encase::{ShaderType, internal::WriteInto};

pub trait BasicUniform: ShaderType + WriteInto {
    fn descriptor() -> DynamicUniformBuffer;

    fn buffer(&self) -> Vec<u8> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(self).expect("Failed to write uniform buffer");
        buffer.into_inner()
    }
}

#[derive(Clone, Copy, ShaderType)]
pub struct UniformTransform {
    pub transform_matrix: glam::Mat4,
}

impl BasicUniform for UniformTransform {
    fn descriptor() -> DynamicUniformBuffer {
        DynamicUniformBufferBuilder::new(Self::min_size())
            .with_member("transform_matrix", UniformType::Mat4)
            .build()
    }
}

#[derive(Clone, Copy, ShaderType)]
pub struct UniformRgb {
    pub color: glam::Vec3,
}

impl BasicUniform for UniformRgb {
    fn descriptor() -> DynamicUniformBuffer {
        DynamicUniformBufferBuilder::new(Self::min_size())
            .with_member("color", UniformType::F32x3)
            .build()
    }
}

#[derive(Clone, Copy, ShaderType)]
pub struct UniformRgba {
    pub color: glam::Vec4,
}

impl BasicUniform for UniformRgba {
    fn descriptor() -> DynamicUniformBuffer {
        DynamicUniformBufferBuilder::new(Self::min_size())
            .with_member("color", UniformType::F32x4)
            .build()
    }
}

#[derive(Clone, Copy, ShaderType)]
pub struct UniformTransformRgb {
    pub transform_matrix: glam::Mat4,
    pub color: glam::Vec3,
}

impl BasicUniform for UniformTransformRgb {
    fn descriptor() -> DynamicUniformBuffer {
        DynamicUniformBufferBuilder::new(Self::min_size())
            .with_member("transform_matrix", UniformType::Mat4)
            .with_member("color", UniformType::F32x3)
            .build()
    }
}

#[derive(Clone, Copy, ShaderType)]
pub struct UniformTransformRgba {
    pub transform_matrix: glam::Mat4,
    pub color: glam::Vec4,
}

impl BasicUniform for UniformTransformRgba {
    fn descriptor() -> DynamicUniformBuffer {
        DynamicUniformBufferBuilder::new(Self::min_size())
            .with_member("transform_matrix", UniformType::Mat4)
            .with_member("color", UniformType::F32x4)
            .build()
    }
}
