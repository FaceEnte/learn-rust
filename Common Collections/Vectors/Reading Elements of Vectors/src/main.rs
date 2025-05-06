fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    match v.get(3) {
        Some(fourth) => println!("The fourth elements is {}", fourth),
        None => println!("There is no fourth elements"),
    }

    match v.get(4) {
        Some(fifth) => println!("The fifth elements is {}", fifth),
        None => println!("There is no fifth elements"),
    }

    let does_not_exist1 = &v[100]; // PANIC!!!
    let does_not_exist2 = v.get(100); // No panic
}
