use ripemd::{Ripemd160, Ripemd320, Digest};

fn main() {
    let mut hasher1 = Ripemd160::new();
    let mut hasher2 = Ripemd320::new();

    hasher1.update("Hello");
    hasher2.update("Hello");

    let res1 = hasher1.finalize();
    let res2 = hasher2.finalize();

    println!("{:x}", res1);
    println!("{:x}", res2);
}
