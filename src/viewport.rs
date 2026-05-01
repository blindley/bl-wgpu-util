#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl From<[f32; 4]> for Viewport {
    fn from(value: [f32; 4]) -> Self {
        Self {
            x: value[0],
            y: value[1],
            width: value[2],
            height: value[3],
        }
    }
}

impl From<wgpu::Extent3d> for Viewport {
    fn from(value: wgpu::Extent3d) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: value.width as f32,
            height: value.height as f32,
        }
    }
}

impl From<&wgpu::Texture> for Viewport {
    fn from(value: &wgpu::Texture) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: value.size().width as f32,
            height: value.size().height as f32,
        }
    }
}

impl Viewport {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_size(width: f32, height: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width,
            height,
        }
    }

    pub fn apply(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_viewport(self.x, self.y, self.width, self.height, 0.0, 1.0);
    }

    pub fn area_is_positive(&self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }
}
