use sm3::{Digest, Sm3};

fn main() {
    let mut hash = Sm3::new();
    hash.update("Hello");
    let res = hash.finalize();
    println!("\nThe Sm3 hash for \"Hello\" is:");
    println!("{:x}", res);
}
