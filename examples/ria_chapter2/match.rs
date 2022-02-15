fn main() {
   let n = 18;
   let result = match n {
       0         => "zero",
       10 ..= 20 => "ten to twenty inclusive",
       40 | 80   => "forty or eighty",
       _         => "something else"
   };
   println!("{}", result);
}
