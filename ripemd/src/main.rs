use ripemd::{Ripemd160, Ripemd256, Ripemd320, Digest};

fn main() {
    let mut hasher1 = Ripemd160::new();
    let mut hasher2 = Ripemd256::new();
    let mut hasher3 = Ripemd320::new();

    hasher1.update("Hello");
    hasher2.update("Hello");
    hasher3.update("Hello");

    let res1 = hasher1.finalize();
    let res2 = hasher2.finalize();
    let res3 = hasher3.finalize();

    println!("\nThe Ripemd160 has for \"Hello\" is:");
    println!("{:x}\n", res1);
    println!("The Ripemd256 has for \"Hello\" is:");
    println!("{:x}\n", res2);
    println!("The Ripemd320 has for \"Hello\" is:");
    println!("{:x}\n", res3);
}
