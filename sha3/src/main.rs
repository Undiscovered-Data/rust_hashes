use sha3::{Digest, Sha3_256, Sha3_512};

fn main() {
    let mut hash1 = Sha3_256::new();
    let mut hash2 = Sha3_512::new();

    hash1.update("hello");
    hash2.update("hello");

    let res1 = hash1.finalize();
    let res2 = hash2.finalize();

    println!("{:x}", res1);
    println!("{:x}", res2);
}
