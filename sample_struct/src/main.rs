trait Animal {
    fn make_sound(&self);
}

struct Horse;
struct Cat;

impl Animal for Horse {
    fn make_sound(&self) {
        println!("ヒヒーン");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("ニャー");
    }
}

fn main() {
    let horse = Horse;
    let cat = Cat;
    horse.make_sound();
    cat.make_sound();
}

