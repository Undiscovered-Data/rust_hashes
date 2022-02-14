use md4::{Md4, Digest};

fn main() {
    let mut hasher = Md4::new();
    hasher.update("Hello");
    let res = hasher.finalize();
    println!("\nThe Md4 hash for \"Hello\" is:");
    println!("{:x}\n", res);
}
