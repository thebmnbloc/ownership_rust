fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // this will print `hello, world!`

    bior_name();
}





fn bior_name() -> String {
   let mut name = String::from("Bior"); // creates a String from a string literal
   name.push_str(" is awesome!"); // appends a string literal to the String
   println!("{}", name); // prints the string
   return name // returns the string
}
