use sha3::{Digest, Keccak224, Keccak256, Keccak256Full, Keccak384,
        Keccak512, Sha3_224, Sha3_256, Sha3_384, Sha3_512};

fn main() {
    let mut hash1 = Keccak224::new();
    let mut hash2 = Keccak256::new();
    let mut hash3 = Keccak256Full::new();
    let mut hash4 = Keccak384::new();
    let mut hash5 = Keccak512::new();
    let mut hash6 = Sha3_224::new();
    let mut hash7 = Sha3_256::new();
    let mut hash8 = Sha3_384::new();
    let mut hash9 = Sha3_512::new();

    hash1.update("hello");
    hash2.update("hello");
    hash3.update("hello");
    hash4.update("hello");
    hash5.update("hello");
    hash6.update("hello");
    hash7.update("hello");
    hash8.update("hello");
    hash9.update("hello");

    let res1 = hash1.finalize();
    let res2 = hash2.finalize();
    let res3 = hash3.finalize();
    let res4 = hash4.finalize();
    let res5 = hash5.finalize();
    let res6 = hash6.finalize();
    let res7 = hash7.finalize();
    let res8 = hash8.finalize();
    let res9 = hash9.finalize();

    println!("\nThe Keccak224 hash for \"Hello\" is:");
    println!("{:x}\n", res1);
    println!("The Keccak256 hash for \"Hello\" is:");
    println!("{:x}\n", res2);
    println!("The Keccak256Full hash for \"Hello\" is:");
    println!("{:x}\n", res3);
    println!("The Keccak384 hash for \"Hello\" is:");
    println!("{:x}\n", res4);
    println!("The Keccak512 hash for \"Hello\" is:");
    println!("{:x}\n", res5);
    println!("The Sha3_224 hash for \"Hello\" is:");
    println!("{:x}\n", res6);
    println!("The Sha3_256 hash for \"Hello\" is:");
    println!("{:x}\n", res7);
    println!("The Sha3_384 hash for \"Hello\" is:");
    println!("{:x}\n", res8);
    println!("The Sha3_512 hash for \"Hello\" is:");
    println!("{:x}\n", res9);
}
