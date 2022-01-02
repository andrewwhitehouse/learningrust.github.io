fn main() {
    let centigrade = 3.0;
    let result = format!("{0:.1} centigrade = {1} fahrenheit", centigrade, centigrade * 1.8 + 32.0);
    println!("{}", result.to_uppercase());
}
