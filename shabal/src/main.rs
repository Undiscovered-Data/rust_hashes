use shabal::{Shabal192, Shabal224, Shabal256, Shabal384, Shabal512, Digest};

fn main() {
    let mut hash1 = Shabal192::new();
    let mut hash2 = Shabal224::new();
    let mut hash3 = Shabal256::new();
    let mut hash4 = Shabal384::new();
    let mut hash5 = Shabal512::new();

    hash1.update("Hello");
    hash2.update("Hello");
    hash3.update("Hello");
    hash4.update("Hello");
    hash5.update("Hello");

    let res1 = hash1.finalize();
    let res2 = hash2.finalize();
    let res3 = hash3.finalize();
    let res4 = hash4.finalize();
    let res5 = hash5.finalize();

    println!("\nThe Shabal192 hash for \"Hello\" is:");
    println!("{:x}\n", res1);
    println!("The Shabal224 hash for \"Hello\" is:");
    println!("{:x}\n", res2);
    println!("The Shabal256 hash for \"Hello\" is:");
    println!("{:x}\n", res3);
    println!("The Shabal384 hash for \"Hello\" is:");
    println!("{:x}\n", res4);
    println!("The Shabal512 hash for \"Hello\" is:");
    println!("{:x}\n", res5);
}
