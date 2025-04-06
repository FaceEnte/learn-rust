fn main() {
    // put your code here to launch it

    let var: i32 = -1;

    println!("{}", test(var));
}

pub fn test(mut x: i32) -> i32 {
    if x > 0 {
        let y: i32 = x + 1;
        y
    } else {
        x
    }
}
