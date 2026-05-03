#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Viewport {
    pub offset: glam::Vec2,
    pub size: glam::Vec2,
}

impl From<[f32; 4]> for Viewport {
    fn from(value: [f32; 4]) -> Self {
        Self {
            offset: glam::vec2(value[0], value[1]),
            size: glam::vec2(value[2], value[3]),
        }
    }
}

impl From<wgpu::Extent3d> for Viewport {
    fn from(value: wgpu::Extent3d) -> Self {
        Self {
            offset: glam::vec2(0.0, 0.0),
            size: glam::vec2(value.width as f32, value.height as f32),
        }
    }
}

impl From<&wgpu::Texture> for Viewport {
    fn from(value: &wgpu::Texture) -> Self {
        Self {
            offset: glam::vec2(0.0, 0.0),
            size: glam::vec2(value.size().width as f32, value.size().height as f32),
        }
    }
}

impl Viewport {
    pub fn new(offset: glam::Vec2, size: glam::Vec2) -> Self {
        Self { offset, size }
    }

    pub fn from_size(size: glam::Vec2) -> Self {
        Self {
            offset: glam::vec2(0.0, 0.0),
            size,
        }
    }

    pub fn apply(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_viewport(
            self.offset.x,
            self.offset.y,
            self.size.x,
            self.size.y,
            0.0,
            1.0,
        );
    }

    pub fn area_is_positive(&self) -> bool {
        self.size.x > 0.0 && self.size.y > 0.0
    }

    pub fn contains(&self, point: glam::Vec2) -> bool {
        point.x >= self.offset.x
            && point.x < self.offset.x + self.size.x
            && point.y >= self.offset.y
            && point.y < self.offset.y + self.size.y
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.size.x / self.size.y
    }
}
