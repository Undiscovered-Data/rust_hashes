use fsb::{Digest, Fsb256, Fsb512};

fn main() {
    let mut hash1 = Fsb256::new();
    let mut hash2 = Fsb512::new();

    hash1.update("Hello");
    hash2.update("Hello");

    let result1 = hash1.finalize();
    let result2 = hash2.finalize();

    println!("{:x}", result1);
    println!("{:x}", result2);
}
