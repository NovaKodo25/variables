/* 
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The values of x is: {}", x);
}
*/

/*
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

} 
*/

/*
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
*/

fn main() {

    // integer
    let num: i8 = 50;
    println!("First value: {}", num);

    let num: i8 = 45;
    println!("Second value: {}", num);

    // let num = "hello";
    // println!("Third value: {}", num);

    let num: i8 = num + 10;
    println!("Third value: {}", num);

    // float
    let num: f32 = 10.5;
    println!("Fourth value: {}", num);

    // boolean
    let is_true: bool = true;
    println!("Fifth value: {}", is_true);

    // character
    let letter: char = '#';
    println!("Sixth value: {}", letter);
}