use super::{DynamicVertexDescriptor, DynamicVertexDescriptorBuilder};
use wgpu::VertexFormat::*;

macro_rules! offset_of {
    ($field:ident) => {
        Some(::std::mem::offset_of!(Self, $field) as _)
    };
}

/// Trait for vertex formats.
pub trait BasicVertex {
    fn format() -> DynamicVertexDescriptor;
}

pub trait BasicVertex2d {
    fn position(&self) -> glam::Vec2;
}

pub trait BasicVertex3d {
    fn position(&self) -> glam::Vec3;
}

pub trait BasicVertexRgb {
    fn color(&self) -> glam::Vec3;
}

pub trait BasicVertexRgba {
    fn color(&self) -> glam::Vec4;
}

pub trait BasicVertexUv {
    fn uv(&self) -> glam::Vec2;
}

/// Vertex with 2D position.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2d {
    pub position: glam::Vec2,
}

impl Vertex2d {
    pub fn new(position: impl Into<glam::Vec2>) -> Self {
        Self {
            position: position.into(),
        }
    }
}

impl BasicVertex for Vertex2d {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x2, offset_of!(position))
            .build()
    }
}

impl BasicVertex2d for Vertex2d {
    fn position(&self) -> glam::Vec2 {
        self.position
    }
}

/// Vertex with 3D position.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3d {
    pub position: glam::Vec3,
}

impl Vertex3d {
    pub fn new(position: impl Into<glam::Vec3>) -> Self {
        Self {
            position: position.into(),
        }
    }
}

impl BasicVertex for Vertex3d {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x3, offset_of!(position))
            .build()
    }
}

impl BasicVertex3d for Vertex3d {
    fn position(&self) -> glam::Vec3 {
        self.position
    }
}

/// Vertex with 2D position and 3-channel color.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2dRgb {
    pub position: glam::Vec2,
    pub color: glam::Vec3,
}

impl Vertex2dRgb {
    pub fn new(position: impl Into<glam::Vec2>, color: impl Into<glam::Vec3>) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
        }
    }
}

impl BasicVertex for Vertex2dRgb {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x2, offset_of!(position))
            .with_attribute("color", Float32x3, offset_of!(color))
            .build()
    }
}

impl BasicVertex2d for Vertex2dRgb {
    fn position(&self) -> glam::Vec2 {
        self.position
    }
}

impl BasicVertexRgb for Vertex2dRgb {
    fn color(&self) -> glam::Vec3 {
        self.color
    }
}

/// Vertex with 2D position and 4-channel color.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2dRgba {
    pub position: glam::Vec2,
    _pad: [u32; 2],
    pub color: glam::Vec4,
}

impl Vertex2dRgba {
    pub fn new(position: impl Into<glam::Vec2>, color: impl Into<glam::Vec4>) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
            _pad: [0; 2],
        }
    }
}

impl BasicVertex for Vertex2dRgba {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x2, offset_of!(position))
            .with_attribute("color", Float32x4, offset_of!(color))
            .build()
    }
}

impl BasicVertex2d for Vertex2dRgba {
    fn position(&self) -> glam::Vec2 {
        self.position
    }
}

impl BasicVertexRgba for Vertex2dRgba {
    fn color(&self) -> glam::Vec4 {
        self.color
    }
}

/// Vertex with 3D position and 3-channel color.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3dRgb {
    pub position: glam::Vec3,
    pub color: glam::Vec3,
}

impl Vertex3dRgb {
    pub fn new(position: impl Into<glam::Vec3>, color: impl Into<glam::Vec3>) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
        }
    }
}

impl BasicVertex for Vertex3dRgb {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x3, offset_of!(position))
            .with_attribute("color", Float32x3, offset_of!(color))
            .build()
    }
}

impl BasicVertex3d for Vertex3dRgb {
    fn position(&self) -> glam::Vec3 {
        self.position
    }
}

impl BasicVertexRgb for Vertex3dRgb {
    fn color(&self) -> glam::Vec3 {
        self.color
    }
}

/// Vertex with 3D position and 4-channel color.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3dRgba {
    pub position: glam::Vec3,
    _pad: [u32; 1],
    pub color: glam::Vec4,
}

impl Vertex3dRgba {
    pub fn new(position: impl Into<glam::Vec3>, color: impl Into<glam::Vec4>) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
            _pad: [0; 1],
        }
    }
}

impl BasicVertex for Vertex3dRgba {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x3, offset_of!(position))
            .with_attribute("color", Float32x4, offset_of!(color))
            .build()
    }
}

impl BasicVertex3d for Vertex3dRgba {
    fn position(&self) -> glam::Vec3 {
        self.position
    }
}

impl BasicVertexRgba for Vertex3dRgba {
    fn color(&self) -> glam::Vec4 {
        self.color
    }
}

/// Vertex with 2D position and 2-channel texture coordinate.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2dUv {
    pub position: glam::Vec2,
    pub uv: glam::Vec2,
}

impl Vertex2dUv {
    pub fn new(position: impl Into<glam::Vec2>, uv: impl Into<glam::Vec2>) -> Self {
        Self {
            position: position.into(),
            uv: uv.into(),
        }
    }
}

impl BasicVertex for Vertex2dUv {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x2, offset_of!(position))
            .with_attribute("uv", Float32x2, offset_of!(uv))
            .build()
    }
}

impl BasicVertex2d for Vertex2dUv {
    fn position(&self) -> glam::Vec2 {
        self.position
    }
}

impl BasicVertexUv for Vertex2dUv {
    fn uv(&self) -> glam::Vec2 {
        self.uv
    }
}

/// Vertex with 3D position.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3dUv {
    pub position: glam::Vec3,
    pub uv: glam::Vec2,
}

impl Vertex3dUv {
    pub fn new(position: impl Into<glam::Vec3>, uv: impl Into<glam::Vec2>) -> Self {
        Self {
            position: position.into(),
            uv: uv.into(),
        }
    }
}

impl BasicVertex for Vertex3dUv {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x3, offset_of!(position))
            .with_attribute("uv", Float32x2, offset_of!(uv))
            .build()
    }
}

impl BasicVertex3d for Vertex3dUv {
    fn position(&self) -> glam::Vec3 {
        self.position
    }
}

impl BasicVertexUv for Vertex3dUv {
    fn uv(&self) -> glam::Vec2 {
        self.uv
    }
}

/// Vertex with 2D position and 3-channel color.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2dRgbUv {
    pub position: glam::Vec2,
    pub color: glam::Vec3,
    pub uv: glam::Vec2,
}

impl Vertex2dRgbUv {
    pub fn new(
        position: impl Into<glam::Vec2>,
        color: impl Into<glam::Vec3>,
        uv: impl Into<glam::Vec2>,
    ) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
            uv: uv.into(),
        }
    }
}

impl BasicVertex for Vertex2dRgbUv {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x2, offset_of!(position))
            .with_attribute("color", Float32x3, offset_of!(color))
            .with_attribute("uv", Float32x2, offset_of!(uv))
            .build()
    }
}

impl BasicVertex2d for Vertex2dRgbUv {
    fn position(&self) -> glam::Vec2 {
        self.position
    }
}

impl BasicVertexRgb for Vertex2dRgbUv {
    fn color(&self) -> glam::Vec3 {
        self.color
    }
}

impl BasicVertexUv for Vertex2dRgbUv {
    fn uv(&self) -> glam::Vec2 {
        self.uv
    }
}

/// Vertex with 2D position and 4-channel color.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex2dRgbaUv {
    pub position: glam::Vec2,
    pub uv: glam::Vec2,
    pub color: glam::Vec4,
}

impl Vertex2dRgbaUv {
    pub fn new(
        position: impl Into<glam::Vec2>,
        color: impl Into<glam::Vec4>,
        uv: impl Into<glam::Vec2>,
    ) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
            uv: uv.into(),
        }
    }
}

impl BasicVertex for Vertex2dRgbaUv {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x2, offset_of!(position))
            .with_attribute("color", Float32x4, offset_of!(color))
            .with_attribute("uv", Float32x2, offset_of!(uv))
            .build()
    }
}

impl BasicVertex2d for Vertex2dRgbaUv {
    fn position(&self) -> glam::Vec2 {
        self.position
    }
}

impl BasicVertexRgba for Vertex2dRgbaUv {
    fn color(&self) -> glam::Vec4 {
        self.color
    }
}

impl BasicVertexUv for Vertex2dRgbaUv {
    fn uv(&self) -> glam::Vec2 {
        self.uv
    }
}

/// Vertex with 3D position and 3-channel color.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3dRgbUv {
    pub position: glam::Vec3,
    pub color: glam::Vec3,
    pub uv: glam::Vec2,
}

impl Vertex3dRgbUv {
    pub fn new(
        position: impl Into<glam::Vec3>,
        color: impl Into<glam::Vec3>,
        uv: impl Into<glam::Vec2>,
    ) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
            uv: uv.into(),
        }
    }
}

impl BasicVertex for Vertex3dRgbUv {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x3, offset_of!(position))
            .with_attribute("color", Float32x3, offset_of!(color))
            .with_attribute("uv", Float32x2, offset_of!(uv))
            .build()
    }
}

impl BasicVertex3d for Vertex3dRgbUv {
    fn position(&self) -> glam::Vec3 {
        self.position
    }
}

impl BasicVertexRgb for Vertex3dRgbUv {
    fn color(&self) -> glam::Vec3 {
        self.color
    }
}

impl BasicVertexUv for Vertex3dRgbUv {
    fn uv(&self) -> glam::Vec2 {
        self.uv
    }
}

/// Vertex with 3D position and 4-channel color.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex3dRgbaUv {
    pub position: glam::Vec3,
    pub uv: glam::Vec2,
    _pad: [u32; 3],
    pub color: glam::Vec4,
}

impl Vertex3dRgbaUv {
    pub fn new(
        position: impl Into<glam::Vec3>,
        color: impl Into<glam::Vec4>,
        uv: impl Into<glam::Vec2>,
    ) -> Self {
        Self {
            position: position.into(),
            color: color.into(),
            uv: uv.into(),
            _pad: [0; 3],
        }
    }
}

impl BasicVertex for Vertex3dRgbaUv {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x3, offset_of!(position))
            .with_attribute("color", Float32x4, offset_of!(color))
            .with_attribute("uv", Float32x2, offset_of!(uv))
            .build()
    }
}

impl BasicVertex3d for Vertex3dRgbaUv {
    fn position(&self) -> glam::Vec3 {
        self.position
    }
}

impl BasicVertexRgba for Vertex3dRgbaUv {
    fn color(&self) -> glam::Vec4 {
        self.color
    }
}

impl BasicVertexUv for Vertex3dRgbaUv {
    fn uv(&self) -> glam::Vec2 {
        self.uv
    }
}
