use md2::{Md2, Digest};

fn main() {
    let mut hasher =  Md2::new();
    hasher.update("Hello");
    let res = hasher.finalize();
    println!("\nThe Md2 hash for \"Hello\" is:");
    println!("{:x}\n", res);
}
