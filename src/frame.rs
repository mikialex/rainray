
pub struct Frame{
    width: u32,
    height: u32,
    data: Vec<f32>
}

impl Frame {
    pub fn new(width:u32, height:u32)-> Frame {
        
    }

    fn pixel_count(&self) -> u32 {
        self.width * self.height
    }
}
