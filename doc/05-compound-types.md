**Compound Types: Tuples**

Rust supports _tuples_, which provide a way to relate items, which could be of different types. (But they don't have to be.)

A geometric point is an example where the types could be the same.

Let's rewrite our previous example to make use of tuples.

And in doing so, let's come up with better names than those symbol which are quite difficult to distinguish, and it isn't clear what the symbols represent. Picking good names is important, if you assume that somebody else is going to read your code (and understand it).

In fact, it's been said more than once that there are three hard problems in Computer Science: caching, naming and off-by-one errors!

If we're writing code to be around for a long time (or even for our selves, coming back to a piece of code after a month) by picking good names we can indicate its intent.

```
fn haversine_distance(from_lat_lng: (f64, f64), to_lat_lng: (f64, f64)) -> f64 {
    let average_earth_radius_metres = 6_371_000f64;
    let lat1_radians = from_lat_lng.0 * PI / 180.0;
    let lat2_radians = to_lat_lng.0 * PI / 180.0;
    let latitude_delta = (from_lat_lng.1 - from_lat_lng.0) * PI / 180.0;
    let longitude_delta = (to_lat_lng.1 - from_lat_lng.0) * PI / 180.0;
    let a = (latitude_delta / 2.0).sin().powi(2) +
        lat1_radians.cos() * lat2_radians.cos() * (longitude_delta / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0-a).sqrt());
    average_earth_radius_metres * c
}
```


Exercises:

1. There is a problem with this programme

  ```
  fn main() {
      let point1 = (15, 20);
      let point2 = (80, 92);
      let point1_diff = point2.0 - point1.0;
      let point2_diff = point2.1 - point1.1;
      let length = (point1_diff*point1_diff + point2_diff*point2_diff).sqrt();
      println!("{}", length);
}
  ``` 

  Can you fix this calculation of the Pythagorean distance by converting the sum of squares to a floating point value? Hint: *integer_value as f32* is one option. 

  What is the length of the line between the two points?

[details="Answer"]
1. 97
[/details]    
Copyright (c) 2022 Andrew Whitehouse. All rights reserved.
