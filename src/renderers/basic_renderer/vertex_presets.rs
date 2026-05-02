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
    fn position_mut(&mut self) -> &mut glam::Vec2;

    fn translate(&mut self, offset: impl Into<glam::Vec2>) {
        *self.position_mut() += offset.into();
    }

    fn scale(&mut self, scale: impl Into<f32>) {
        *self.position_mut() *= scale.into();
    }

    fn non_uniform_scale(&mut self, scale: impl Into<glam::Vec2>) {
        *self.position_mut() *= scale.into();
    }

    fn transform(&mut self, mat: &glam::Mat4) {
        let position = mat * self.position().extend(0.0).extend(1.0);
        *self.position_mut() = glam::Vec2::new(position.x / position.w, position.y / position.w);
    }

    /// Creates a rectangle of 2-d vertices from the given vertex and its opposite corner.
    ///
    /// ## Requirement
    /// `out.len() >= 6`
    fn make_rect(&self, other: glam::Vec2, out: &mut [Self])
    where
        Self: Clone,
    {
        assert!(out.len() >= 6);
        out[0] = self.clone();
        out[1] = self.clone();
        out[2] = self.clone();
        out[3] = self.clone();
        out[4] = self.clone();
        out[5] = self.clone();

        out[1].position_mut().y = other.y;
        *out[2].position_mut() = other;

        *out[4].position_mut() = other;
        out[5].position_mut().x = other.x;
    }
}

pub trait BasicVertex3d {
    fn position(&self) -> glam::Vec3;
    fn position_mut(&mut self) -> &mut glam::Vec3;

    fn translate(&mut self, offset: impl Into<glam::Vec3>) {
        *self.position_mut() += offset.into();
    }

    fn scale(&mut self, scale: impl Into<f32>) {
        *self.position_mut() *= scale.into();
    }

    fn non_uniform_scale(&mut self, scale: impl Into<glam::Vec3>) {
        *self.position_mut() *= scale.into();
    }

    fn transform(&mut self, mat: &glam::Mat4) {
        let position = mat * self.position().extend(1.0);
        *self.position_mut() = glam::Vec3::new(
            position.x / position.w,
            position.y / position.w,
            position.z / position.w,
        );
    }
}

pub trait BasicVertexRgb {
    fn color(&self) -> glam::Vec3;
    fn color_mut(&mut self) -> &mut glam::Vec3;
}

pub trait BasicVertexRgba {
    fn color(&self) -> glam::Vec4;
    fn color_mut(&mut self) -> &mut glam::Vec4;
}

pub trait BasicVertexUv {
    fn uv(&self) -> glam::Vec2;
    fn uv_mut(&mut self) -> &mut glam::Vec2;
}

macro_rules! impl_accessor_trait {
    ($type:ty, $trait:ty, $field:ident, $mut_accessor:ident, $ret:ty) => {
        impl $trait for $type {
            fn $field(&self) -> $ret {
                self.$field
            }

            fn $mut_accessor(&mut self) -> &mut $ret {
                &mut self.$field
            }
        }
    };
}

macro_rules! impl_2d {
    ($type:ty) => {
        impl_accessor_trait!($type, BasicVertex2d, position, position_mut, glam::Vec2);
    };
}

macro_rules! impl_3d {
    ($type:ty) => {
        impl_accessor_trait!($type, BasicVertex3d, position, position_mut, glam::Vec3);
    };
}

macro_rules! impl_rgb {
    ($type:ty) => {
        impl_accessor_trait!($type, BasicVertexRgb, color, color_mut, glam::Vec3);
    };
}

macro_rules! impl_rgba {
    ($type:ty) => {
        impl_accessor_trait!($type, BasicVertexRgba, color, color_mut, glam::Vec4);
    };
}

macro_rules! impl_uv {
    ($type:ty) => {
        impl_accessor_trait!($type, BasicVertexUv, uv, uv_mut, glam::Vec2);
    };
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

impl_2d!(Vertex2d);

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

impl_3d!(Vertex3d);

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

impl_2d!(Vertex2dRgb);
impl_rgb!(Vertex2dRgb);

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

impl_2d!(Vertex2dRgba);
impl_rgba!(Vertex2dRgba);

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

impl_3d!(Vertex3dRgb);
impl_rgb!(Vertex3dRgb);

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

impl_3d!(Vertex3dRgba);
impl_rgba!(Vertex3dRgba);

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

impl_2d!(Vertex2dUv);
impl_uv!(Vertex2dUv);

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

impl_3d!(Vertex3dUv);
impl_uv!(Vertex3dUv);

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

impl_2d!(Vertex2dRgbUv);
impl_rgb!(Vertex2dRgbUv);
impl_uv!(Vertex2dRgbUv);

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

impl_2d!(Vertex2dRgbaUv);
impl_rgba!(Vertex2dRgbaUv);
impl_uv!(Vertex2dRgbaUv);

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

impl_3d!(Vertex3dRgbUv);
impl_rgb!(Vertex3dRgbUv);
impl_uv!(Vertex3dRgbUv);

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

impl_3d!(Vertex3dRgbaUv);
impl_rgba!(Vertex3dRgbaUv);
impl_uv!(Vertex3dRgbaUv);
