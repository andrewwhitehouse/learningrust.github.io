use std::any::type_name;

#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {

  println!("1 {}", type_of(1));

  // println!("5i32 {}", type_of(5i32));

  /*
  let max = std::u128::MAX;
  println!("max {} {}", max, type_of(max));
  */

  /*
  let default_integer = 4;
  println!("default_integer {} {}", default_integer, type_of(default_integer));
  */

  //println!("false {}", type_of(false));
 
  println!("{}", type_of([1,2,3].len()));

  //println!("\u{2705} {}", type_of('\u{2705}'));

  /*
  let default_float = 3.0;
  println!("default_float {} {}", default_float, type_of(default_float));
  */

  /*
  let a_float: f64 = 1.0;
  println!("a_float {}", type_of(a_float));
  */

  let mut inferred_type = -12; // Type i64 is inferred from another line
  println!("inferred_type (mutable) {} {}", inferred_type, type_of(inferred_type));
  inferred_type = 5_000_000_000i64;
}
