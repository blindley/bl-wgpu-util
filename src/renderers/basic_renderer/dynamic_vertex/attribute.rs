use half::f16;
use wgpu::VertexFormat;

////////////////////////////////////////////////////////////////////////////////
// Public Functions
////////////////////////////////////////////////////////////////////////////////

/// Returns the required byte alignment for a given `wgpu::VertexFormat`.
pub const fn vertex_format_align_of(vf: VertexFormat) -> usize {
    use core::mem::align_of;
    use wgpu::VertexFormat as vf;
    match vf {
        vf::Uint8 | vf::Sint8 | vf::Unorm8 | vf::Snorm8 => align_of::<u8>(),
        vf::Uint8x2 | vf::Sint8x2 | vf::Unorm8x2 | vf::Snorm8x2 => align_of::<[u8; 2]>(),
        vf::Uint8x4 | vf::Sint8x4 | vf::Unorm8x4 | vf::Snorm8x4 => align_of::<[u8; 4]>(),
        vf::Uint16 | vf::Sint16 | vf::Unorm16 | vf::Snorm16 | vf::Float16 => align_of::<u16>(),
        vf::Uint16x2 | vf::Sint16x2 | vf::Unorm16x2 | vf::Snorm16x2 | vf::Float16x2 => {
            align_of::<[u16; 2]>()
        }
        vf::Uint16x4 | vf::Sint16x4 | vf::Unorm16x4 | vf::Snorm16x4 | vf::Float16x4 => {
            align_of::<[u16; 4]>()
        }
        vf::Uint32 | vf::Sint32 | vf::Float32 => align_of::<u32>(),
        vf::Uint32x2 | vf::Sint32x2 | vf::Float32x2 => align_of::<[u32; 2]>(),
        vf::Uint32x3 | vf::Sint32x3 | vf::Float32x3 => align_of::<[u32; 3]>(),
        vf::Uint32x4 | vf::Sint32x4 | vf::Float32x4 => align_of::<[u32; 4]>(),
        vf::Float64 => align_of::<f64>(),
        vf::Float64x2 => align_of::<[f64; 2]>(),
        vf::Float64x3 => align_of::<[f64; 3]>(),
        vf::Float64x4 => align_of::<[f64; 4]>(),
        vf::Unorm10_10_10_2 => align_of::<u32>(),
        vf::Unorm8x4Bgra => align_of::<[u8; 4]>(),
    }
}

////////////////////////////////////////////////////////////////////////////////
// DynamicAttribute
////////////////////////////////////////////////////////////////////////////////

/// An enum that can store a value for any variant of `wgpu::VertexFormat`.
///
/// This is used to hold vertex attribute data in memory in a type-safe way that
/// still allows for runtime layout definition.
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DynamicAttribute {
    Uint8(u8), Uint8x2([u8; 2]), Uint8x4([u8; 4]),
    Sint8(i8), Sint8x2([i8; 2]), Sint8x4([i8; 4]),
    Unorm8(u8), Unorm8x2([u8; 2]), Unorm8x4([u8; 4]),
    Snorm8(i8), Snorm8x2([i8; 2]), Snorm8x4([i8; 4]),
    Uint16(u16), Uint16x2([u16; 2]), Uint16x4([u16; 4]),
    Sint16(i16), Sint16x2([i16; 2]), Sint16x4([i16; 4]),
    Unorm16(u16), Unorm16x2([u16; 2]), Unorm16x4([u16; 4]),
    Snorm16(i16), Snorm16x2([i16; 2]), Snorm16x4([i16; 4]),
    Float16(f16), Float16x2([f16; 2]), Float16x4([f16; 4]),
    Float32(f32), Float32x2([f32; 2]), Float32x3([f32; 3]), Float32x4([f32; 4]),
    Uint32(u32), Uint32x2([u32; 2]), Uint32x3([u32; 3]), Uint32x4([u32; 4]),
    Sint32(i32), Sint32x2([i32; 2]), Sint32x3([i32; 3]), Sint32x4([i32; 4]),
    Float64(f64), Float64x2([f64; 2]), Float64x3([f64; 3]), Float64x4([f64; 4]),
    Unorm10_10_10_2(u32), Unorm8x4Bgra([u8; 4]),
}

impl DynamicAttribute {
    /// The size of the attribute in bytes.
    #[allow(dead_code)]
    pub const fn size(&self) -> usize {
        self.vertex_format().size() as usize
    }

    /// The required alignment of the attribute.
    #[allow(dead_code)]
    pub const fn align_of(&self) -> usize {
        vertex_format_align_of(self.vertex_format())
    }

    /// Returns the corresponding wgpu::VertexFormat variant
    #[allow(dead_code)]
    pub const fn vertex_format(&self) -> wgpu::VertexFormat {
        use DynamicAttribute::*;
        use wgpu::VertexFormat as vf;

        match self {
            Uint8(_) => vf::Uint8,
            Uint8x2(_) => vf::Uint8x2,
            Uint8x4(_) => vf::Uint8x4,
            Sint8(_) => vf::Sint8,
            Sint8x2(_) => vf::Sint8x2,
            Sint8x4(_) => vf::Sint8x4,
            Unorm8(_) => vf::Unorm8,
            Unorm8x2(_) => vf::Unorm8x2,
            Unorm8x4(_) => vf::Unorm8x4,
            Snorm8(_) => vf::Snorm8,
            Snorm8x2(_) => vf::Snorm8x2,
            Snorm8x4(_) => vf::Snorm8x4,
            Uint16(_) => vf::Uint16,
            Uint16x2(_) => vf::Uint16x2,
            Uint16x4(_) => vf::Uint16x4,
            Sint16(_) => vf::Sint16,
            Sint16x2(_) => vf::Sint16x2,
            Sint16x4(_) => vf::Sint16x4,
            Unorm16(_) => vf::Unorm16,
            Unorm16x2(_) => vf::Unorm16x2,
            Unorm16x4(_) => vf::Unorm16x4,
            Snorm16(_) => vf::Snorm16,
            Snorm16x2(_) => vf::Snorm16x2,
            Snorm16x4(_) => vf::Snorm16x4,
            Float16(_) => vf::Float16,
            Float16x2(_) => vf::Float16x2,
            Float16x4(_) => vf::Float16x4,
            Float32(_) => vf::Float32,
            Float32x2(_) => vf::Float32x2,
            Float32x3(_) => vf::Float32x3,
            Float32x4(_) => vf::Float32x4,
            Uint32(_) => vf::Uint32,
            Uint32x2(_) => vf::Uint32x2,
            Uint32x3(_) => vf::Uint32x3,
            Uint32x4(_) => vf::Uint32x4,
            Sint32(_) => vf::Sint32,
            Sint32x2(_) => vf::Sint32x2,
            Sint32x3(_) => vf::Sint32x3,
            Sint32x4(_) => vf::Sint32x4,
            Float64(_) => vf::Float64,
            Float64x2(_) => vf::Float64x2,
            Float64x3(_) => vf::Float64x3,
            Float64x4(_) => vf::Float64x4,
            Unorm10_10_10_2(_) => vf::Unorm10_10_10_2,
            Unorm8x4Bgra(_) => vf::Unorm8x4Bgra,
        }
    }

    /// Copy the value of the attribute into the given buffer.
    #[allow(dead_code)]
    pub const fn write(&self, buffer: &mut [u8]) {
        use DynamicAttribute::*;

        let size = self.size();
        assert!(size <= buffer.len(), "Buffer overflow");

        use std::ptr::copy_nonoverlapping;

        macro_rules! copy {
            ($v:expr) => {
                copy_nonoverlapping($v as *const _ as *const u8, buffer.as_mut_ptr(), size)
            };
        }

        unsafe {
            match self {
                Uint8(v) | Unorm8(v) => copy!(v),
                Sint8(v) | Snorm8(v) => copy!(v),
                Uint8x2(v) | Unorm8x2(v) => copy!(v),
                Sint8x2(v) | Snorm8x2(v) => copy!(v),
                Uint8x4(v) | Unorm8x4(v) => copy!(v),
                Sint8x4(v) | Snorm8x4(v) => copy!(v),
                Uint16(v) | Unorm16(v) => copy!(v),
                Sint16(v) | Snorm16(v) => copy!(v),
                Uint16x2(v) | Unorm16x2(v) => copy!(v),
                Sint16x2(v) | Snorm16x2(v) => copy!(v),
                Uint16x4(v) | Unorm16x4(v) => copy!(v),
                Sint16x4(v) | Snorm16x4(v) => copy!(v),
                Float16(v) => copy!(v),
                Float16x2(v) => copy!(v),
                Float16x4(v) => copy!(v),
                Float32(v) => copy!(v),
                Float32x2(v) => copy!(v),
                Float32x3(v) => copy!(v),
                Float32x4(v) => copy!(v),
                Uint32(v) => copy!(v),
                Uint32x2(v) => copy!(v),
                Uint32x3(v) => copy!(v),
                Uint32x4(v) => copy!(v),
                Sint32(v) => copy!(v),
                Sint32x2(v) => copy!(v),
                Sint32x3(v) => copy!(v),
                Sint32x4(v) => copy!(v),
                Float64(v) => copy!(v),
                Float64x2(v) => copy!(v),
                Float64x3(v) => copy!(v),
                Float64x4(v) => copy!(v),
                Unorm10_10_10_2(v) => copy!(v),
                Unorm8x4Bgra(v) => copy!(v),
            }
        }
    }

    /// Read an attribute from the given buffer
    pub fn read(ty: wgpu::VertexFormat, buffer: &[u8]) -> Self {
        use DynamicAttribute::*;
        use wgpu::VertexFormat as vf;
        let size = ty.size() as usize;
        assert!(size <= buffer.len(), "Buffer overflow");

        let mut data = vec![0u8; size as usize];
        data.copy_from_slice(&buffer[..size as usize]);

        macro_rules! copy {
            ($t:ty) => {{
                let v: $t = bytemuck::pod_read_unaligned(&data);
                v
            }};
        }

        match ty {
            vf::Uint8 => Uint8(copy!(u8)),
            vf::Uint8x2 => Uint8x2(copy!([u8; 2])),
            vf::Uint8x4 => Uint8x4(copy!([u8; 4])),
            vf::Sint8 => Sint8(copy!(i8)),
            vf::Sint8x2 => Sint8x2(copy!([i8; 2])),
            vf::Sint8x4 => Sint8x4(copy!([i8; 4])),
            vf::Unorm8 => Unorm8(copy!(u8)),
            vf::Unorm8x2 => Unorm8x2(copy!([u8; 2])),
            vf::Unorm8x4 => Unorm8x4(copy!([u8; 4])),
            vf::Snorm8 => Snorm8(copy!(i8)),
            vf::Snorm8x2 => Snorm8x2(copy!([i8; 2])),
            vf::Snorm8x4 => Snorm8x4(copy!([i8; 4])),
            vf::Uint16 => Uint16(copy!(u16)),
            vf::Uint16x2 => Uint16x2(copy!([u16; 2])),
            vf::Uint16x4 => Uint16x4(copy!([u16; 4])),
            vf::Sint16 => Sint16(copy!(i16)),
            vf::Sint16x2 => Sint16x2(copy!([i16; 2])),
            vf::Sint16x4 => Sint16x4(copy!([i16; 4])),
            vf::Unorm16 => Unorm16(copy!(u16)),
            vf::Unorm16x2 => Unorm16x2(copy!([u16; 2])),
            vf::Unorm16x4 => Unorm16x4(copy!([u16; 4])),
            vf::Snorm16 => Snorm16(copy!(i16)),
            vf::Snorm16x2 => Snorm16x2(copy!([i16; 2])),
            vf::Snorm16x4 => Snorm16x4(copy!([i16; 4])),
            vf::Float16 => Float16(copy!(f16)),
            vf::Float16x2 => Float16x2(copy!([f16; 2])),
            vf::Float16x4 => Float16x4(copy!([f16; 4])),
            vf::Float32 => Float32(copy!(f32)),
            vf::Float32x2 => Float32x2(copy!([f32; 2])),
            vf::Float32x3 => Float32x3(copy!([f32; 3])),
            vf::Float32x4 => Float32x4(copy!([f32; 4])),
            vf::Uint32 => Uint32(copy!(u32)),
            vf::Uint32x2 => Uint32x2(copy!([u32; 2])),
            vf::Uint32x3 => Uint32x3(copy!([u32; 3])),
            vf::Uint32x4 => Uint32x4(copy!([u32; 4])),
            vf::Sint32 => Sint32(copy!(i32)),
            vf::Sint32x2 => Sint32x2(copy!([i32; 2])),
            vf::Sint32x3 => Sint32x3(copy!([i32; 3])),
            vf::Sint32x4 => Sint32x4(copy!([i32; 4])),
            vf::Float64 => Float64(copy!(f64)),
            vf::Float64x2 => Float64x2(copy!([f64; 2])),
            vf::Float64x3 => Float64x3(copy!([f64; 3])),
            vf::Float64x4 => Float64x4(copy!([f64; 4])),
            vf::Unorm10_10_10_2 => Unorm10_10_10_2(copy!(u32)),
            vf::Unorm8x4Bgra => Unorm8x4Bgra(copy!([u8; 4])),
        }
    }
}
