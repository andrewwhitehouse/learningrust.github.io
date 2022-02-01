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
