extern crate image;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color {r,g,b}
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
            data: vec![vec![Color::new(0.0, 0.0, 0.0); height as usize]; width as usize],
        };
        frame.clear(&Color::new(0.0, 0.0, 0.0));
        return frame;
    }

    pub fn clear(&mut self, color:&Color) {
        let data = &mut self.data;
        for i in 0..data.len() {
            let row = &mut data[i];
            for j in 0..row.len() {
                data[i][j].r = color.r;
                data[i][j].g = color.g;
                data[i][j].b = color.b;
            }
        }
    }

    #[allow(dead_code)]
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
        println!("{} pixels has write to {}", self.pixel_count(), path);
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