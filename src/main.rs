
mod vec;
mod ray;
mod object;
mod sphere;
mod material;
mod frame;

fn main() {
    let mut point = vec::Vec3{
        x: 1.,
        y: 2.,
        z: 3.
    };

    point.x = 2.;
    // println!("{}", point);
}

pub fn another_function(x: i32, y: i32) ->i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    return x;
}