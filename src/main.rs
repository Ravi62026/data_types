fn main() {
    // let mut a = 10;
    // println!("Before Modify: ");
    // println!("{a}");
    // a = 34;
    // println!("After Modify: {a}");

    // // a = "10";

    // let b = 10.5;
    // println!("Value of b is: {b} ");

    // let is_active: bool = true;
    // println!("{is_active}");

    // let emoji: char = '😊';
    // let letter: char = 'A';

    // println!("{emoji}");
    // println!("{letter}");

    // // Basic Operations: 

    // // addition

    // let a = 10;
    // let b = 20;

    // let res = a + b;
    // println!("Result of addition are : {res} ");


    // // let a: f64 = 10.5; // error as data types mismatch
    // // let b = 20;

    // // let res = a + b;

    // let a = 10.5;
    // let b = 20.6;

    // let res = a + b;

    // println!("Result of addition are : {res} ");

    // // subscription

    // let x = 34.4;
    // let y: f32 = 54.3;

    // let res = x - y;

    // println!("{res}");

    // // multiplication

    // let res = 4 * 34;
    // println!("{res}");

    // // let res = 4 * 34.6; // error of mismatch data types  cannot multiply `{integer}` by `{float}`
    // // println!("{res}");

    // // division

    // let x1 = 43;
    // let y1 = 22;

    // // quotient

    // let quotient = x1 / y1 ;
    // println!("Quotient are : {quotient} ");

    // let quotient = -x1 / y1 ;
    // println!("Quotient are : {quotient} ");

    // let quotient = 56.7 / 32.2;
    // println!("Quotient are : {quotient} ");


    // // trncate

    // let truncated = -5 / 3; // Results in -1
    // println!("Truncated : {truncated} ");

    // // remainder

    // let remainder = 43 % 5;

    // println!("Remainder are : {remainder} ");

    // let x = 32;
    // let y = -32;

    // let x: u8 = 8;
    // let y: i8 = -8;

    // let h = 7;

    // TUPLE

    // let ravi = ("Ravi", 21, true, 1000);  // immutable

    // let tup = (); // empty tuple

    // let (x, y, z, p) = ravi;

    // println!("x: {x}");
    // println!("y: {y}");
    // println!("z: {z}");
    // println!("p: {p}");

    // println!("0th value is {} ", ravi.0);


    // ARRAY

    let a: [i32; 5] = [1,2,3,4,5]; // fixed size stack rather than heap

    println!("{}", a[0]);

    let b = [10; 5];  // array of length 5 with all value of 10 

    println!("{}", b[3]);

    let idx = random();

    println!("Value is {}", a[idx]);


}

fn random() -> usize {
    7
}
