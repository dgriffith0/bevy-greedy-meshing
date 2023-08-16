use bevy::prelude::Color;

pub type Voxels = Vec<Vec<Vec<Voxel>>>;

#[derive(Clone, Default)]
pub struct Voxel {
    pub kind: u8,
    pub size: Option<(usize, usize, usize)>,
}

impl Voxel {
    pub fn new(kind: u8) -> Self {
        Self {
            kind,
            size: Some((1, 1, 1)),
        }
    }

    pub fn kind_map(&self) -> u8 {
        match self.kind {
            0 => 0,
            1 => 0,
            _ => 1,
        }
    }

    pub fn color(&self) -> Color {
        match self.kind_map() {
            0 => Color::YELLOW,
            1 => Color::GREEN,
            2 => Color::BLUE,
            _ => Color::WHITE,
        }
    }
}
