use tiger::{Tiger, Tiger2, Digest};

fn main() {
    let mut hash1 = Tiger::new();
    let mut hash2 = Tiger2::new();

    hash1.update("Hello");
    hash2.update("Hello");

    let res1 = hash1.finalize();
    let res2 = hash2.finalize();

    println!("\nThe Tiger hash for \"Hello\" is:");
    println!("{:x}\n", res1);
    println!("The Tiger2 hash for \"Hello\" is:");
    println!("{:x}\n", res2);
}
