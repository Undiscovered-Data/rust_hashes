use sha1::{Sha1, Digest};

fn main() {
    let mut hasher = Sha1::new();
    hasher.update("Hello");
    let res = hasher.finalize();
    println!("\nThe Sha1 hash for \"Hello\" is:");
    println!("{:x}\n", res);
}
