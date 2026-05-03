use super::{DynamicVertexDescriptor, DynamicVertexDescriptorBuilder};
use wgpu::VertexFormat::*;

use glam::{Vec2, Vec3, Vec4, vec2, vec3};

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
    fn position(&self) -> Vec2;
    fn position_mut(&mut self) -> &mut Vec2;

    fn translate(&mut self, offset: Vec2) {
        *self.position_mut() += offset;
    }

    fn scale(&mut self, scale: f32) {
        *self.position_mut() *= scale;
    }

    fn non_uniform_scale(&mut self, scale: Vec2) {
        *self.position_mut() *= scale;
    }

    fn transform(&mut self, mat: &glam::Mat4) {
        let position = mat * self.position().extend(0.0).extend(1.0);
        *self.position_mut() = Vec2::new(position.x / position.w, position.y / position.w);
    }

    /// Creates a rectangle of 2-d vertices from the given vertex and its opposite corner.
    ///
    /// ## Requirement
    /// `out.len() >= 6`
    fn make_rect(out: &mut [Self], v0: Vec2, v1: Vec2)
    where
        Self: Sized,
    {
        assert!(out.len() >= 6);

        *out[0].position_mut() = v0;
        *out[1].position_mut() = vec2(v1.x, v0.y);
        *out[2].position_mut() = v1;
        *out[3].position_mut() = v0;
        *out[4].position_mut() = v1;
        *out[5].position_mut() = vec2(v0.x, v1.y);
    }
}

pub trait BasicVertex3d {
    fn position(&self) -> Vec3;
    fn position_mut(&mut self) -> &mut Vec3;

    fn translate(&mut self, offset: Vec3) {
        *self.position_mut() += offset;
    }

    fn scale(&mut self, scale: f32) {
        *self.position_mut() *= scale;
    }

    fn non_uniform_scale(&mut self, scale: Vec3) {
        *self.position_mut() *= scale;
    }

    fn transform(&mut self, mat: &glam::Mat4) {
        let position = mat * self.position().extend(1.0);
        *self.position_mut() = Vec3::new(
            position.x / position.w,
            position.y / position.w,
            position.z / position.w,
        );
    }

    /// Creates a cube of 3-d vertices from two opposite corners.
    ///
    /// ## Requirement
    /// `out.len() >= 36`
    fn make_box(out: &mut [Self], v0: Vec3, v1: Vec3)
    where
        Self: Sized,
    {
        assert!(out.len() >= 36);

        let vertices = [
            v0,
            vec3(v1.x, v0.y, v0.z),
            vec3(v0.x, v1.y, v0.z),
            vec3(v1.x, v1.y, v0.z),
            vec3(v0.x, v0.y, v1.z),
            vec3(v1.x, v0.y, v1.z),
            vec3(v0.x, v1.y, v1.z),
            v1,
        ];

        //     6-------7
        //    /|      /|
        //   / |     / |
        //  2-------3  |
        //  |  4----|--5
        //  | /     | /
        //  |/      |/
        //  0-------1

        // Sides:
        // front    right   back    left
        //  2---3   3---7   7---6   6---2
        //  |   |   |   |   |   |   |   |
        //  |   |   |   |   |   |   |   |
        //  0---1   1---5   5---4   4---0

        // Bottom/Top:
        //  0---1   6---7
        //  |   |   |   |
        //  |   |   |   |
        //  4---5   2---3

        #[rustfmt::skip]
        const INDICES: [usize; 36] = [
            0, 1, 3, 0, 3, 2, // front
            1, 5, 7, 1, 7, 3, // right
            5, 4, 6, 5, 6, 7, // back
            4, 0, 2, 4, 2, 6, // left
            4, 5, 1, 4, 1, 0, // bottom
            2, 3, 7, 2, 7, 6, // top
        ];

        for (out_v, i) in out.iter_mut().zip(INDICES.iter()) {
            *out_v.position_mut() = vertices[*i];
        }
    }
}

pub trait BasicVertexRgb {
    fn color(&self) -> Vec3;
    fn color_mut(&mut self) -> &mut Vec3;
}

pub trait BasicVertexRgba {
    fn color(&self) -> Vec4;
    fn color_mut(&mut self) -> &mut Vec4;
}

pub trait BasicVertexUv {
    fn uv(&self) -> Vec2;
    fn uv_mut(&mut self) -> &mut Vec2;
}

macro_rules! pod_vertex {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $field:ident : $ty:ty),*
            $(,)?
        }
    ) => {
        #[repr(C)]
        #[derive(Debug, Clone, Copy, PartialEq, Default, bytemuck::Pod, bytemuck::Zeroable)]
        $(#[$meta])*
        $vis struct $name {
            $($field_vis $field: $ty),*
        }
    };
}

pod_vertex!(
    /// Vertex with 2D position.
    pub struct Vertex2d {
        pub position: Vec2,
    }
);

impl Vertex2d {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

impl BasicVertex for Vertex2d {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x2, offset_of!(position))
            .build()
    }
}

pod_vertex!(
    /// Vertex with 3D position.
    pub struct Vertex3d {
        pub position: Vec3,
    }
);

impl Vertex3d {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}

impl BasicVertex for Vertex3d {
    fn format() -> DynamicVertexDescriptor {
        DynamicVertexDescriptorBuilder::new()
            .with_attribute("position", Float32x3, offset_of!(position))
            .build()
    }
}

pod_vertex!(
    /// Vertex with 2D position and 3-channel color.
    pub struct Vertex2dRgb {
        pub position: Vec2,
        pub color: Vec3,
    }
);

impl Vertex2dRgb {
    pub fn new(position: Vec2, color: Vec3) -> Self {
        Self { position, color }
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

pod_vertex!(
    /// Vertex with 2D position and 4-channel color.
    pub struct Vertex2dRgba {
        pub position: Vec2,
        _pad: [u32; 2],
        pub color: Vec4,
    }
);

impl Vertex2dRgba {
    pub fn new(position: Vec2, color: Vec4) -> Self {
        Self {
            position,
            color,
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

pod_vertex!(
    /// Vertex with 3D position and 3-channel color.
    pub struct Vertex3dRgb {
        pub position: Vec3,
        pub color: Vec3,
    }
);

impl Vertex3dRgb {
    pub fn new(position: Vec3, color: Vec3) -> Self {
        Self { position, color }
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

pod_vertex!(
    /// Vertex with 3D position and 4-channel color.
    pub struct Vertex3dRgba {
        pub position: Vec3,
        _pad: [u32; 1],
        pub color: Vec4,
    }
);

impl Vertex3dRgba {
    pub fn new(position: Vec3, color: Vec4) -> Self {
        Self {
            position,
            color,
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

pod_vertex!(
    /// Vertex with 2D position and uv coordinates.
    pub struct Vertex2dUv {
        pub position: Vec2,
        pub uv: Vec2,
    }
);

impl Vertex2dUv {
    pub fn new(position: Vec2, uv: Vec2) -> Self {
        Self { position, uv }
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

pod_vertex!(
    /// Vertex with 3D position and uv coordinates.
    pub struct Vertex3dUv {
        pub position: Vec3,
        pub uv: Vec2,
    }
);

impl Vertex3dUv {
    pub fn new(position: Vec3, uv: Vec2) -> Self {
        Self { position, uv }
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

pod_vertex!(
    /// Vertex with 2D position, 3-channel color and uv coordinates.
    pub struct Vertex2dRgbUv {
        pub position: Vec2,
        pub color: Vec3,
        pub uv: Vec2,
    }
);

impl Vertex2dRgbUv {
    pub fn new(position: Vec2, color: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            color,
            uv,
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

pod_vertex!(
    /// Vertex with 2D position, 4-channel color and uv coordinates.
    pub struct Vertex2dRgbaUv {
        pub position: Vec2,
        pub uv: Vec2,
        pub color: Vec4,
    }
);

impl Vertex2dRgbaUv {
    pub fn new(position: Vec2, color: Vec4, uv: Vec2) -> Self {
        Self {
            position,
            color,
            uv,
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

pod_vertex!(
    /// Vertex with 3D position, 3-channel color and uv coordinates.
    pub struct Vertex3dRgbUv {
        pub position: Vec3,
        pub color: Vec3,
        pub uv: Vec2,
    }
);

impl Vertex3dRgbUv {
    pub fn new(position: Vec3, color: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            color,
            uv,
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

pod_vertex!(
    /// Vertex with 3D position, 4-channel color and uv coordinates.
    pub struct Vertex3dRgbaUv {
        pub position: Vec3,
        pub uv: Vec2,
        _pad: [u32; 3],
        pub color: Vec4,
    }
);

impl Vertex3dRgbaUv {
    pub fn new(position: Vec3, color: Vec4, uv: Vec2) -> Self {
        Self {
            position,
            color,
            uv,
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

////////////////////////////////////////////////////////////////////////////////
// Vertex Trait Macro Based Implementations
////////////////////////////////////////////////////////////////////////////////
///
///

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
        impl_accessor_trait!($type, BasicVertex2d, position, position_mut, Vec2);
    };
}

macro_rules! impl_3d {
    ($type:ty) => {
        impl_accessor_trait!($type, BasicVertex3d, position, position_mut, Vec3);
    };
}

macro_rules! impl_rgb {
    ($type:ty) => {
        impl_accessor_trait!($type, BasicVertexRgb, color, color_mut, Vec3);
    };
}

macro_rules! impl_rgba {
    ($type:ty) => {
        impl_accessor_trait!($type, BasicVertexRgba, color, color_mut, Vec4);
    };
}

macro_rules! impl_uv {
    ($type:ty) => {
        impl_accessor_trait!($type, BasicVertexUv, uv, uv_mut, Vec2);
    };
}

impl_2d!(Vertex2d);

impl_3d!(Vertex3d);

impl_2d!(Vertex2dRgb);
impl_rgb!(Vertex2dRgb);

impl_2d!(Vertex2dRgba);
impl_rgba!(Vertex2dRgba);

impl_3d!(Vertex3dRgb);
impl_rgb!(Vertex3dRgb);

impl_3d!(Vertex3dRgba);
impl_rgba!(Vertex3dRgba);

impl_2d!(Vertex2dUv);
impl_uv!(Vertex2dUv);

impl_3d!(Vertex3dUv);
impl_uv!(Vertex3dUv);

impl_2d!(Vertex2dRgbUv);
impl_rgb!(Vertex2dRgbUv);
impl_uv!(Vertex2dRgbUv);

impl_2d!(Vertex2dRgbaUv);
impl_rgba!(Vertex2dRgbaUv);
impl_uv!(Vertex2dRgbaUv);

impl_3d!(Vertex3dRgbUv);
impl_rgb!(Vertex3dRgbUv);
impl_uv!(Vertex3dRgbUv);

impl_3d!(Vertex3dRgbaUv);
impl_rgba!(Vertex3dRgbaUv);
impl_uv!(Vertex3dRgbaUv);
