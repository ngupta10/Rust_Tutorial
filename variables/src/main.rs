fn main() {
    let x : i32 =4;
    println!("{}",x);
    let x : &str = "six";
    println!("{}",x);

    let tup: (&str, i32) = ("NG",12);
    let (channel , count) = tup;
    let coun1 = tup.1;

    let sum = my_function(5,6);
    println!("{}", sum);


}

fn my_function(z: i32, y: i32) -> i32 {
    println!("z : {}",z);
    println!("y : {}",y);
    let sum : i32 = z + y;
    return sum
}
