fn main() {
// let mut const
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let mut spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces);

// learn types

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let tup_first = tup.0;
    println!("{}", tup_first)

    // list
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //これは整数型のサイズ以外は、上と同義。

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let first = a[0];
    let second = a[1];
}

