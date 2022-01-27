**Numerical Expressions**

Rust support the common set of [operators](https://doc.rust-lang.org/reference/expressions.html#expression-precedence) used in expressions, which follow common precedence rules.

So `*` takes precedence over `+`, for example, meaning that `1 + 3 * 5 = 16`.

Some languages support `++` and `--` to increment or decrement a variabe. In Rust use `+= 1` or `-= 1`.

Let's calculate the distance between two points using the [Haversine](https://en.wikipedia.org/wiki/Haversine_formula) (also described [here](https://www.movable-type.co.uk/scripts/latlong.html)) formula.

Here's some sample code:

```
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
```

The approach for calling the trigonometry functions may be different to other languages that you've used. Here it is applied as a method to the type.


Exercises:

1. If the latitude and logitude of Ayres Rock in Australia are -25.344490 and 131.035431 respectively.

   And the summit of Mount Everest is latitude 27.986065 longitude 86.922623, what is the distance as the crow flies, to the nearest metre, between these two points?

   Hint: write a main function to pass these parameters to the supplied function.

[details="Answer"]
1. 7578338
[/details]
Copyright (c) 2022 Andrew Whitehouse. All rights reserved.
