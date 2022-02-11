use sha2::{Sha256, Sha512, Digest};

fn main() {
    let mut hash1 = Sha256::new();
    let mut hash2 = Sha512::new();

    hash1.update("Hello");
    hash2.update("Hello");

    let res1 = hash1.finalize();
    let res2 = hash2.finalize();

    println!("{:x}", res1);
    println!("{:x}", res2);
}
