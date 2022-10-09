struct Person {
    name: String,
    age: u8
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {} and I am {}.", self.name, self.age);
    }
    
}

fn main() {
    let mona = Person { name: String::from("Mona"), age: 23};

    println!("{}", mona.to_string());
    
}
