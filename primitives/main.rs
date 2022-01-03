use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
  println!("1 {}", type_of(1));
  println!("false {}", type_of(false));

  let a_float: f64 = 1.0;
  println!("a_float {}", type_of(a_float));

  println!("5i32 {}", type_of(5i32));

  let max = std::u128::MAX;
  println!("max {} {}", max, type_of(max));

  let default_float = 3.0;
  println!("default_float {} {}", default_float, type_of(default_float));

  let default_integer = 4;
  println!("default_integer {} {}", default_integer, type_of(default_integer));

  let mut inferred_type = 12; // Type i64 is inferred from another line
  inferred_type = 4294967296i64;
  println!("inferred_type (mutable) {} {}", inferred_type, type_of(inferred_type));
}
