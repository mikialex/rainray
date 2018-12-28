
mod vec;
mod ray;
mod object;
mod sphere;
mod material;
mod frame;

fn main() {
    let mut color = vec::Vec3{
        x: 1.,
        y: 2.,
        z: 3.
    };

    let mut frame = frame::Frame::new(100, 100);
    frame.set_pixel(&color, 2, 2);
    println!("frame pixel count: {}", frame.pixel_count());

    color.x = 2.;
    // println!("{}", point);
}

pub fn another_function(x: i32, y: i32) ->i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    return x;
}