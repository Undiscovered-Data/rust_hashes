use gost94::{Gost94CryptoPro, Digest};

fn main() {
    let mut hasher = Gost94CryptoPro::new();
    hasher.update("Hello");
    let result = hasher.finalize();
    println!("{:x}", result);
}
