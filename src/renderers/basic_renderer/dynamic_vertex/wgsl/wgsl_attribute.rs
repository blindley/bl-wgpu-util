use super::super::attribute::DynamicAttribute;
use wgpu::VertexFormat;

////////////////////////////////////////////////////////////////////////////////
// Private Utility Functions
////////////////////////////////////////////////////////////////////////////////

const fn unorm8(v: u8) -> f32 {
    v as f32 / 255.0
}

const fn unorm16(v: u16) -> f32 {
    v as f32 / 65535.0
}

const fn snorm8(v: i8) -> f32 {
    v as f32 / 127.0
}

const fn snorm16(v: i16) -> f32 {
    v as f32 / 32767.0
}

const fn unorm10_10_10_2(v: u32) -> [f32; 4] {
    let r = ((v >> 22) & 0x3FF) as f32 / 1023.0;
    let g = ((v >> 12) & 0x3FF) as f32 / 1023.0;
    let b = ((v >> 2) & 0x3FF) as f32 / 1023.0;
    let a = (v & 0x3) as f32 / 3.0;
    [r, g, b, a]
}

const fn unorm8x4_bgra(v: [u8; 4]) -> [f32; 4] {
    [unorm8(v[2]), unorm8(v[1]), unorm8(v[0]), unorm8(v[3])]
}

////////////////////////////////////////////////////////////////////////////////
// WgslAttributeType
////////////////////////////////////////////////////////////////////////////////

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WgslAttributeType {
    U32, U32x2, U32x3, U32x4,
    S32, S32x2, S32x3, S32x4,
    F32, F32x2, F32x3, F32x4,
}

impl WgslAttributeType {
    #[cfg(any())]
    pub const fn comp(&self) -> usize {
        use WgslAttributeType::*;
        match self {
            U32 | S32 | F32 => 1,
            U32x2 | S32x2 | F32x2 => 2,
            U32x3 | S32x3 | F32x3 => 3,
            U32x4 | S32x4 | F32x4 => 4,
        }
    }

    #[cfg(any())]
    pub const fn size(&self) -> usize {
        self.comp() * 4
    }

    pub const fn to_string(&self) -> &'static str {
        use WgslAttributeType::*;
        match self {
            U32 => "u32",
            U32x2 => "vec2<u32>",
            U32x3 => "vec3<u32>",
            U32x4 => "vec4<u32>",
            S32 => "i32",
            S32x2 => "vec2<i32>",
            S32x3 => "vec3<i32>",
            S32x4 => "vec4<i32>",
            F32 => "f32",
            F32x2 => "vec2<f32>",
            F32x3 => "vec3<f32>",
            F32x4 => "vec4<f32>",
        }
    }

    pub const fn from_vertex_format(value: VertexFormat) -> Self {
        use VertexFormat::*;
        use WgslAttributeType::*;
        match value {
            Uint8 | Uint16 | Uint32 => U32,
            Sint8 | Sint16 | Sint32 => S32,
            Unorm8 | Unorm16 | Snorm8 | Snorm16 => F32,
            Float16 | Float32 | Float64 => F32,

            Uint8x2 | Uint16x2 | Uint32x2 => U32x2,
            Sint8x2 | Sint16x2 | Sint32x2 => S32x2,
            Unorm8x2 | Unorm16x2 | Snorm8x2 | Snorm16x2 => F32x2,
            Float16x2 | Float32x2 | Float64x2 => F32x2,

            Uint32x3 => U32x3,
            Sint32x3 => S32x3,
            Float32x3 | Float64x3 => F32x3,

            Uint8x4 | Uint16x4 | Uint32x4 => U32x4,
            Sint8x4 | Sint16x4 | Sint32x4 => S32x4,
            Unorm8x4 | Unorm16x4 | Snorm8x4 | Snorm16x4 => F32x4,
            Float16x4 | Float32x4 | Float64x4 => F32x4,

            Unorm10_10_10_2 | Unorm8x4Bgra => F32x4,
        }
    }
}

impl From<VertexFormat> for WgslAttributeType {
    fn from(value: VertexFormat) -> Self {
        Self::from_vertex_format(value)
    }
}

impl std::fmt::Display for WgslAttributeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

////////////////////////////////////////////////////////////////////////////////
// WgslAttribute
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WgslAttribute {
    U32(u32),
    U32x2([u32; 2]),
    U32x3([u32; 3]),
    U32x4([u32; 4]),
    S32(i32),
    S32x2([i32; 2]),
    S32x3([i32; 3]),
    S32x4([i32; 4]),
    F32(f32),
    F32x2([f32; 2]),
    F32x3([f32; 3]),
    F32x4([f32; 4]),
}

impl WgslAttribute {
    #[cfg(any())]
    pub fn type_(&self) -> WgslAttributeType {
        use WgslAttribute::*;
        use WgslAttributeType as ty;
        match self {
            U32(_) => ty::U32,
            U32x2(_) => ty::U32x2,
            U32x3(_) => ty::U32x3,
            U32x4(_) => ty::U32x4,
            S32(_) => ty::S32,
            S32x2(_) => ty::S32x2,
            S32x3(_) => ty::S32x3,
            S32x4(_) => ty::S32x4,
            F32(_) => ty::F32,
            F32x2(_) => ty::F32x2,
            F32x3(_) => ty::F32x3,
            F32x4(_) => ty::F32x4,
        }
    }

    #[cfg(any())]
    pub fn comp(&self) -> usize {
        self.type_().comp()
    }

    #[cfg(any())]
    pub fn size(&self) -> usize {
        self.type_().size()
    }

    pub fn to_string(&self) -> String {
        use WgslAttribute::*;

        fn join<T: std::fmt::Display, const N: usize>(v: &[T; N]) -> String {
            v.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        }

        fn format_uvec<const N: usize>(v: &[u32; N]) -> String {
            format!("vec{N}<u32>({})", join(v))
        }

        fn format_ivec<const N: usize>(v: &[i32; N]) -> String {
            format!("vec{N}<i32>({})", join(v))
        }

        fn format_fvec<const N: usize>(v: &[f32; N]) -> String {
            format!("vec{N}<f32>({})", join(v))
        }

        match self {
            U32(v) => v.to_string(),
            U32x2(v) => format_uvec(v),
            U32x3(v) => format_uvec(v),
            U32x4(v) => format_uvec(v),
            S32(v) => v.to_string(),
            S32x2(v) => format_ivec(v),
            S32x3(v) => format_ivec(v),
            S32x4(v) => format_ivec(v),
            F32(v) => v.to_string(),
            F32x2(v) => format_fvec(v),
            F32x3(v) => format_fvec(v),
            F32x4(v) => format_fvec(v),
        }
    }
}

macro_rules! impl_from_for_wgsl_attribute {
    ($t:ty, $variant:ident) => {
        impl From<$t> for WgslAttribute {
            fn from(value: $t) -> Self {
                WgslAttribute::$variant(value)
            }
        }
    };
}

impl_from_for_wgsl_attribute!(u32, U32);
impl_from_for_wgsl_attribute!([u32; 2], U32x2);
impl_from_for_wgsl_attribute!([u32; 3], U32x3);
impl_from_for_wgsl_attribute!([u32; 4], U32x4);
impl_from_for_wgsl_attribute!(i32, S32);
impl_from_for_wgsl_attribute!([i32; 2], S32x2);
impl_from_for_wgsl_attribute!([i32; 3], S32x3);
impl_from_for_wgsl_attribute!([i32; 4], S32x4);
impl_from_for_wgsl_attribute!(f32, F32);
impl_from_for_wgsl_attribute!([f32; 2], F32x2);
impl_from_for_wgsl_attribute!([f32; 3], F32x3);
impl_from_for_wgsl_attribute!([f32; 4], F32x4);

impl From<DynamicAttribute> for WgslAttribute {
    fn from(value: DynamicAttribute) -> Self {
        use DynamicAttribute::*;
        use WgslAttribute::*;
        match value {
            Uint8(v) => U32(v.into()),
            Uint8x2(v) => U32x2(v.map(Into::into)),
            Uint8x4(v) => U32x4(v.map(Into::into)),
            Sint8(v) => S32(v.into()),
            Sint8x2(v) => S32x2(v.map(Into::into)),
            Sint8x4(v) => S32x4(v.map(Into::into)),
            Unorm8(v) => F32(unorm8(v)),
            Unorm8x2(v) => F32x2(v.map(unorm8)),
            Unorm8x4(v) => F32x4(v.map(unorm8)),
            Snorm8(v) => F32(snorm8(v)),
            Snorm8x2(v) => F32x2(v.map(snorm8)),
            Snorm8x4(v) => F32x4(v.map(snorm8)),
            Uint16(v) => U32(v.into()),
            Uint16x2(v) => U32x2(v.map(Into::into)),
            Uint16x4(v) => U32x4(v.map(Into::into)),
            Sint16(v) => S32(v.into()),
            Sint16x2(v) => S32x2(v.map(Into::into)),
            Sint16x4(v) => S32x4(v.map(Into::into)),
            Unorm16(v) => F32(unorm16(v)),
            Unorm16x2(v) => F32x2(v.map(unorm16)),
            Unorm16x4(v) => F32x4(v.map(unorm16)),
            Snorm16(v) => F32(snorm16(v)),
            Snorm16x2(v) => F32x2(v.map(snorm16)),
            Snorm16x4(v) => F32x4(v.map(snorm16)),
            Float16(v) => F32(v.to_f32()),
            Float16x2(v) => F32x2(v.map(Into::into)),
            Float16x4(v) => F32x4(v.map(Into::into)),
            Float32(v) => F32(v),
            Float32x2(v) => F32x2(v),
            Float32x3(v) => F32x3(v),
            Float32x4(v) => F32x4(v),
            Uint32(v) => U32(v),
            Uint32x2(v) => U32x2(v),
            Uint32x3(v) => U32x3(v),
            Uint32x4(v) => U32x4(v),
            Sint32(v) => S32(v),
            Sint32x2(v) => S32x2(v),
            Sint32x3(v) => S32x3(v),
            Sint32x4(v) => S32x4(v),
            Float64(v) => F32(v as f32),
            Float64x2(v) => F32x2(v.map(|x| x as f32)),
            Float64x3(v) => F32x3(v.map(|x| x as f32)),
            Float64x4(v) => F32x4(v.map(|x| x as f32)),
            Unorm10_10_10_2(v) => F32x4(unorm10_10_10_2(v)),
            Unorm8x4Bgra(v) => F32x4(unorm8x4_bgra(v)),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_strings() {
        type A = WgslAttribute;

        assert_eq!(A::from(777u32).to_string(), "777");
        assert_eq!(A::from(-99i32).to_string(), "-99");
        assert_eq!(A::from(12345u32).to_string(), "12345");
        assert_eq!(A::from(125.875f32).to_string(), "125.875");
        assert_eq!(A::from([1.25, 2.5]).to_string(), "vec2<f32>(1.25, 2.5)");

        assert_eq!(
            A::from([0_u32, 1, 2, 3]).to_string(),
            "vec4<u32>(0, 1, 2, 3)"
        );

        assert_eq!(A::from([-1, 1, 2, 3]).to_string(), "vec4<i32>(-1, 1, 2, 3)");

        assert_eq!(
            A::from([0.25_f32, 1.125, 2.0625, 3.03125]).to_string(),
            "vec4<f32>(0.25, 1.125, 2.0625, 3.03125)"
        );
    }
}
