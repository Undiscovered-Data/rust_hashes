use whirlpool::{Whirlpool, Digest};

fn main() {
    let mut hasher = Whirlpool::new();
    hasher.update("Hello");
    let res = hasher.finalize();
    println!("\nThe Whirlpool hash for \"Hello\" is:");
    println!("{:x}\n", res);
}
