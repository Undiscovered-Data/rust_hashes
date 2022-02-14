use gost94::{Gost94CryptoPro, Gost94Test, Gost94s2015, Digest};

fn main() {
    let mut hasher = Gost94CryptoPro::new();
    hasher.update("Hello");
    let result = hasher.finalize();
    println!("\nThe Gost94CryptoPro hash for \"Hello\" is:");
    println!("{:x}\n", result);

    let mut hasher2 = Gost94Test::new();
    hasher2.update("Hello");
    let result2 = hasher2.finalize();
    println!("The Gost94Test hash for \"Hello\" is:");
    println!("{:x}\n", result2);

    let mut hasher3 = Gost94s2015::new();
    hasher3.update("Hello");
    let result3 = hasher3.finalize();
    println!("The Gost94s2015 hash for \"Hello\" is:");
    println!("{:x}\n", result3);
}
