/*
fn simplify(s: &mut String) {
    let mut changed;
    loop {
       changed = false;
       let backtrack = ["ns", "sn", "we"];
       for removal in backtrack {
           if s.contains(removal) {
               changed = true;
               *s = s.replace(removal, "");
           }
       }
       if !changed {
           break;
       }
    }
}

fn main() {
    let mut route = String::from("essswnnsssneesweee");
    simplify(&mut route);
    println!("{}", route);
}
*/

/*
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
}
*/

fn main() {
    let s = String::from("hello");

    let r1 = &mut s;
    println!("{} {}", r1, s);
}
