struct Person<'a> {
    name: &'a str,
}

fn longest_name<'a>(p1: Person<'a>, p2: Person<'a>) -> &'a str {
    if p1.name.len() > p2.name.len() {
        p1.name
    } else {
        p2.name
    }
}


fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

fn change_value(value: &mut i32) {
    *value += 10;
}


fn main() {
    let person1 = Person { name: "Alice" };
    let person2 = Person { name: "Bob" };

    let longest = longest_name(person1, person2);
    println!("The longest name is: {}", longest);


    let a: i32 = 10;
    let b: i32 = 20;
    let sum = a + b;
    println!("The sum of {} and {} is {}", a, b, sum);

    let mut value = 5;
    println!("Value before change: {}", value);
    change_value(&mut value);
    println!("Value after change: {}", value);
}
