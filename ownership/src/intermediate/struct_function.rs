#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
    married: bool,
}

fn create_person(name: String, age: i32, married: bool) -> Person {
    Person { name, age, married } // Ownership of 'name' is moved to the struct 'Person'
}

fn print_person(person: Person) {
    println!("Name      : {}", person.name);
    println!("Age       : {}", person.age);
    println!("Married   : {}", if person.married { "Yes" } else { "No" });
    // Ownership of 'person' is dropped here
}

pub fn main() {
    let name: String = String::from("Verdi");
    let person: Person = create_person(name, 21, false);

    // Already moved ownership
    // println!("{}", name); // borrow of moved value: `name`

    println!("{:?}", person);

    print_person(person);

    // println!("{:?}", person) // borrow of moved value: `person`
}