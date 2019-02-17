extern crate image;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Vec<Color>>,
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Frame {
        let mut frame = Frame {
            width,
            height,
            data: vec![vec![Color::new(); height as usize]; width as usize],
        };
        frame.clear();
        return frame;
    }

    pub fn clear(&mut self) {
        let data = &mut self.data;
        for i in 0..data.len() {
            let row = &mut data[i];
            for j in 0..row.len() {
                data[i][j] = Color::new()
            }
        }
    }

    pub fn set_pixel(&mut self, color: &Color, x: u32, y: u32) {
        let data = &mut self.data;
        data[x as usize][y as usize] = color.clone();
    }

    pub fn pixel_count(&self) -> u32 {
        self.width * self.height
    }

    pub fn write_to_file(&self, path: &str) {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let pix = self.data[x as usize][y as usize];
            *pixel = image::Rgb([
            (pix.r.min(1.0).max(0.0) * 255.0) as u8,
            (pix.g.min(1.0).max(0.0) * 255.0) as u8,
            (pix.b.min(1.0).max(0.0) * 255.0) as u8,
        ])
        }

        imgbuf.save(path).unwrap();
    }

    // pub fn iter_pixels(){

    // }
}

// impl Iterator for MyFunkyIterator {
//     type Item = (f64, Position);

//     fn next(&mut self) -> Option<(f64, Position)> {
//         // @target_san's example has the inner iterator at self.0
//         // so maybe call self.0.next(), tweak the result, and return it.
//     }
// }