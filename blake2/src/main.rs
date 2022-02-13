use blake2::{Blake2b512, Blake2s256, Digest};

fn main() {
    let mut hash1 = Blake2s256::new();
    let mut hash2 = Blake2b512::new();

    hash1.update("Hello");
    hash2.update("Hello");

    let res1 = hash1.finalize();
    let res2 = hash2.finalize();

    println!("The Blake2s256 hash for \"Hello\" is:");
    println!("{:x}\n", res1);
    println!("The Blake2b512 hash for \"Hello\" is:");
    println!("{:x}\n", res2);
}
