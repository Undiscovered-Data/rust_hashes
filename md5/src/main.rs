use md5::{Md5, Digest};

fn main() {
    let mut hasher = Md5::new();
    hasher.update("Hello");
    let res = hasher.finalize();
    println!("\nThe Md5 hash for \"Hello\" is:");
    println!("{:x}\n", res);
}
