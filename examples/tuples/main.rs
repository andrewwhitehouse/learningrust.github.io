fn main() {
    let point1 = (15, 20);
    let point2 = (80, 92);
    let point1_diff = (point2.0 - point1.0) as f32;
    let point2_diff = (point2.1 - point1.1) as f32;
    let length = (point1_diff*point1_diff + point2_diff*point2_diff).sqrt();
    println!("{}", length);
}
