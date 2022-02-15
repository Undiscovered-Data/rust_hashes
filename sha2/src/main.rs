use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512_224, Sha512_256, Digest};

fn main() {
    let mut hash1 = Sha224::new();
    let mut hash2 = Sha256::new();
    let mut hash3 = Sha384::new();
    let mut hash4 = Sha512::new();
    let mut hash5 = Sha512_224::new();
    let mut hash6 = Sha512_256::new();

    hash1.update("Hello");
    hash2.update("Hello");
    hash3.update("Hello");
    hash4.update("Hello");
    hash5.update("Hello");
    hash6.update("Hello");

    let res1 = hash1.finalize();
    let res2 = hash2.finalize();
    let res3 = hash3.finalize();
    let res4 = hash4.finalize();
    let res5 = hash5.finalize();
    let res6 = hash6.finalize();

    println!("\nThe Sha224 hash for \"Hello\" is:");
    println!("{:x}\n", res1);
    println!("The Sha256 hash for \"Hello\" is:");
    println!("{:x}\n", res2);
    println!("The Sha384 hash for \"Hello\" is:");
    println!("{:x}\n", res3);
    println!("The Sha512 hash for \"Hello\" is:");
    println!("{:x}\n", res4);
    println!("The Sha512_224 hash for \"Hello\" is:");
    println!("{:x}\n", res5);
    println!("The Sha512_256 hash for \"Hello\" is:");
    println!("{:x}\n", res6);
}
