//fn main() {
//  println!("Hello world");
//}

// 1
/*
fn foo(n: i32) -> i32 {
  if n > 0 {
    return n * 3;
  } else {
    return n * 2;
  }
} 

fn main() {
  println!("{} {}", foo(-23), foo(5));
}
*/

// 2
/*
fn foo(n: i32) -> i32 {
  if n > 0 {
    n * 3;
    println!("Tripled");
  } else {
    n * 2;
    let x = n * 2;
  }
} 

fn main() {
  println!("{} {}", foo(-23), foo(5));
}
*/

// 3
/*
fn foo(n: i32) -> i32 {
  let ret = if n > 0 { n * 3 } else { n * 2 };
  println!("Foo returns {}", ret);
  ret
} 

fn main() {
  println!("{} {}", foo(-23), foo(5));
}
*/

// 4
fn hello(who: Option<&str>) -> String {
  format!("Hello {}", if who.is_none() { "world" } else { who.unwrap() } );
} 

fn main() {
  println!("{}", hello(None));
  println!("{}", hello(Some("again"));
}
