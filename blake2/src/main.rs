use blake2::{Blake2b512, Blake2s256, Digest};

fn main() {
    let mut hash1 = Blake2s256::new();
    let mut hash2 = Blake2b512::new();

    hash1.update("Hello");
    hash2.update("Hello");

    let res1 = hash1.finalize();
    let res2 = hash2.finalize();

    println!("{:x}", res1);
    println!("{:x}", res2);
}
