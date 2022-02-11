use streebog::{Digest, Streebog256, Streebog512};

fn main() {
    let mut hash1 = Streebog256::new();
    let mut hash2 = Streebog512::new();

    hash1.update("Hello");
    hash2.update("Hello");

    let res1 = hash1.finalize();
    let res2 = hash2.finalize();

    println!("{:x}", res1);
    println!("{:x}", res2);
}
