struct Person {
    name: String,
}

impl Person {
    fn new(name: &str) -> Person {
        Person { name: String::from(name) }
    }
}

trait Action {
    fn say(&self) {println!("Some message");}
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let dave: Person = Person::new("Dave");
    dave.say();

    let tom: Person = Person::new("Tom");
    tom.say();
}
