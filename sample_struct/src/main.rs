#[derive(Debug)]
struct Horse {
    name: String,
    age: u32,
    color: String,
    father: String,
    mother: String,
}



fn main() {
    let horse = Horse {
        name: String::from("Thunder"),
        age: 5,
        color: String::from("Black"),
        father: String::from("Lightning"),
        mother: String::from("Storm"),
    };
    println!("{:?}", horse);
}