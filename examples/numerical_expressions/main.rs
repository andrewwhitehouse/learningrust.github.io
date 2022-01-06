use std::f64::consts::PI;

fn haversine_distance(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    let average_earth_radius = 6_371_000f64; // metres
    let φ1 = lat1 * PI / 180.0;
    let φ2 = lat2 * PI / 180.0;
    let δφ = (lat2 - lat1) * PI / 180.0;
    let δλ = (lng2 - lng1) * PI / 180.0;
    let a = (δφ / 2.0).sin().powi(2) +
        φ1.cos() * φ2.cos() * (δλ / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0-a).sqrt());
    average_earth_radius * c
}

fn main() {
    let uluru = (-25.344490, 131.035431);
    let everest = (27.986065, 86.922623);
    let distance = haversine_distance(uluru.0, uluru.1,
                                      everest.0, everest.1);
    println!("{}", distance);
}
