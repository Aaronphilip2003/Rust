fn main() {
    println!("This is a program that demonstrates the concept of shadowing in Rust");
    let mut x=5;
    x=x+1;
    println!("The value of x is :{x}");
    {
        let x=x+2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x in the outer scope is : {x}");
}
