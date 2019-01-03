use crate::vec::Vec3;

pub struct Frame {
    width: u32,
    height: u32,
    data: Vec<f32>,
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Frame {
        let data_length = width * height * 3;
        let mut frame = Frame {
            width,
            height,
            data: vec![0.; data_length as usize],
        };
        frame.clear();
        return frame;
    }

    pub fn clear(&mut self) {
        let data = &mut self.data;
        let end = data.len();
        for i in 0..end {
            data[i] = 0 as f32;
        }
    }

    pub fn set_pixel(&mut self, color: &Vec3, x: u32, y: u32) {
        let data = &mut self.data;
        let index = (y as usize - 1) * self.width as usize + x as usize;
        data[index] = color.x;
        data[index + 1] = color.y;
        data[index + 2] = color.z;
    }

    pub fn pixel_count(&self) -> u32 {
        self.width * self.height
    }

    pub fn write_to_file(&self, path: &str) {
        
    }
}
