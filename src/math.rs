static PI: f64 = 3.14159265359;

pub fn deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn fix_ang(a: f64) -> f64 {
    let mut res = a;
    if res > 359.0 { res -= 360.0; }
    if res < 0.0 { res += 360.0; }
    res
}

pub fn sin(a: f64) -> f64 {
    a.sin()
}

pub fn cos(a: f64) -> f64 {
    a.cos()
}

pub fn tan(a: f64) -> f64 {
    a.tan()
}