fn main() {
    //println!("Hello, world!");
    another_function(87);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
    println!("The value of x + 5 is: {}", plus_five(x));
}

fn another_function(x: i32) {
    println!("x is {}", x);
}

fn plus_five(x: i32) -> i32 {
    x + 5
}
