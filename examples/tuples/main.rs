fn main() {
    let point1 = (15, 20);
    let point2 = (80, 92);
    let point1_delta = (point2.0 - point1.0) as f32;
    let point2_delta = (point2.1 - point1.1) as f32;
    let length = (point1_delta*point1_delta + point2_delta*point2_delta).sqrt();
    println!("{}", length);
}
