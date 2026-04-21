use crate::wgpu;

#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn load<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let img = image::open(path)?;
        let img = img.to_rgba8();
        let (width, height) = img.dimensions();
        Ok(Self {
            width,
            height,
            data: img.into_raw(),
        })
    }

    pub fn load_from_bytes<P: AsRef<[u8]>>(bytes: P) -> anyhow::Result<Self> {
        let img = image::load_from_memory(bytes.as_ref())?;
        let img = img.to_rgba8();
        let (width, height) = img.dimensions();
        Ok(Self {
            width,
            height,
            data: img.into_raw(),
        })
    }

    pub fn extract_sub_image(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
        assert!(x + width <= self.width);
        assert!(y + height <= self.height);
        let mut data = Vec::with_capacity((width * height * 4) as usize);
        for y in y..y + height {
            for x in x..x + width {
                let index = (y * self.width + x) as usize * 4;
                data.extend_from_slice(&self.data[index..index + 4]);
            }
        }
        Self {
            width,
            height,
            data,
        }
    }

    pub fn blit_sub_image(
        &self,
        src_x: u32,
        src_y: u32,
        src_w: u32,
        src_h: u32,
        target: &mut Image,
        dst_x: u32,
        dst_y: u32,
    ) {
        assert!(src_x + src_w <= self.width);
        assert!(src_y + src_h <= self.height);
        assert!(dst_x + src_w <= target.width);
        assert!(dst_y + src_h <= target.height);
        for y in 0..src_h {
            let src_index = ((src_y + y) * self.width + src_x) as usize * 4;
            let dst_index = ((dst_y + y) * target.width + dst_x) as usize * 4;
            target.data[dst_index..dst_index + src_w as usize * 4]
                .copy_from_slice(&self.data[src_index..src_index + src_w as usize * 4]);
        }
    }

    pub fn blit(&self, target: &mut Image, dst_x: u32, dst_y: u32) {
        self.blit_sub_image(0, 0, self.width, self.height, target, dst_x, dst_y);
    }

    pub fn blit_to_self(&mut self, src_x: u32, src_y: u32, w: u32, h: u32, dst_x: u32, dst_y: u32) {
        assert!(src_x + w <= self.width);
        assert!(src_y + h <= self.height);
        assert!(dst_x + w <= self.width);
        assert!(dst_y + h <= self.height);

        if src_y < dst_y {
            for y in (0..h).rev() {
                let src_index = ((src_y + y) * self.width + src_x) as usize * 4;
                let dst_index = ((dst_y + y) * self.width + dst_x) as usize * 4;
                unsafe {
                    let src = self.data.as_ptr().add(src_index);
                    let dst = self.data.as_mut_ptr().add(dst_index);
                    std::ptr::copy_nonoverlapping(src, dst, 4);
                }
            }
        } else if src_y > dst_y {
            for y in 0..h {
                let src_index = ((src_y + y) * self.width + src_x) as usize * 4;
                let dst_index = ((dst_y + y) * self.width + dst_x) as usize * 4;
                unsafe {
                    let src = self.data.as_ptr().add(src_index);
                    let dst = self.data.as_mut_ptr().add(dst_index);
                    std::ptr::copy_nonoverlapping(src, dst, 4);
                }
            }
        } else {
            for y in 0..h {
                let src_index = ((src_y + y) * self.width + src_x) as usize * 4;
                let dst_index = ((dst_y + y) * self.width + dst_x) as usize * 4;
                unsafe {
                    let src = self.data.as_ptr().add(src_index);
                    let dst = self.data.as_mut_ptr().add(dst_index);
                    std::ptr::copy(src, dst, 4);
                }
            }
        }
    }

    pub fn create_texture(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        usage: wgpu::TextureUsages,
    ) -> anyhow::Result<wgpu::Texture> {
        create_texture_from_pixels(device, queue, self.width, self.height, &self.data, usage)
    }
}

pub fn create_texture_from_pixels(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    width: u32,
    height: u32,
    pixels: &[u8],
    usage: wgpu::TextureUsages,
) -> anyhow::Result<wgpu::Texture> {
    assert_eq!(pixels.len(), (width * height * 4) as usize);
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Texture"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage,
        view_formats: &[],
    });
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(width * 4),
            rows_per_image: Some(height),
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    Ok(texture)
}

pub fn load_texture<P: AsRef<std::path::Path>>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    path: P,
    usage: wgpu::TextureUsages,
) -> anyhow::Result<wgpu::Texture> {
    let img = Image::load(path)?;
    img.create_texture(device, queue, usage)
}

pub fn load_texture_from_bytes<P: AsRef<[u8]>>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bytes: P,
    usage: wgpu::TextureUsages,
) -> anyhow::Result<wgpu::Texture> {
    let img = Image::load_from_bytes(bytes)?;
    img.create_texture(device, queue, usage)
}
