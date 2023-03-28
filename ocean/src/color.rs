#[derive(Eq, PartialEq, Debug)]

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn new_red() -> Color {
        Color::new(255, 0, 0)
    }

    pub fn new_green() -> Color {
        Color::new(0, 255, 0)
    }

    pub fn new_blue() -> Color {
        Color::new(0, 0, 255)
    }

    /**
     * Returns a new `Color` whose components are the sum of `c1` and `c2`'s components, modulo 256.
     *
     * First, try writing this function the "obvious" way with arithmetic operations. The test for
     * this method (which you can run with `cargo test part1_color` will fail) with a panic.
     *
     * Note which line of the test is causing the panic: why not the other?
     *
     * Then, look through the documentation for `u8` and see if there is a method that will help you.
     * https://doc.rust-lang.org/std/primitive.u8.html
     */
    pub fn cross(c1: &Color, c2: &Color) -> Color {
        let mut cross_r = c1.r + c2.r;
        if cross_r.checked_div(255) == Some(1){
            if cross_r % 255 > 0{
                cross_r = cross_r % 255;
            }
        } else if cross_r.checked_div(255) > Some(1){
            cross_r = 255 - 1;
        }
        println!("{}", cross_r);
        let mut cross_b = c1.b + c2.b;
        if cross_b.checked_div(255) == Some(1){
            if cross_b % 255 > 0{
                cross_b = cross_b % 255;
            }
        } else if cross_b.checked_div(255) > Some(1) {
            cross_b = 255 - 1;
        }
        let mut cross_g = c1.g + c2.g;
        if cross_g.checked_div(255) == Some(1){
            if cross_g % 255 > 0{
                cross_g = cross_g % 255;
            }
        } else if cross_g.checked_div(255) > Some(1){
            cross_g = 255 - 1;
        }
        Color::new(cross_r, cross_g, cross_b)
    }
}
