use groestl::{Digest, Groestl224, Groestl256, Groestl384, Groestl512};

fn main() {
    let mut hasher1 = Groestl224::default();
    let mut hasher2 = Groestl256::default();
    let mut hasher3 = Groestl384::default();
    let mut hasher4 = Groestl512::default();

    hasher1.update("Hello");
    hasher2.update("Hello");
    hasher3.update("Hello");
    hasher4.update("Hello");

    let result1 = hasher1.finalize();
    let result2 = hasher2.finalize();
    let result3 = hasher3.finalize();
    let result4 = hasher4.finalize();

    println!("\nThe Groestl224 hash for \"Hello\" is:");
    println!("{:x}\n", result1);
    println!("The Groestl256 hash for \"Hello\" is:");
    println!("{:x}\n", result2);
    println!("The Groestl384 hash for \"Hello\" is:");
    println!("{:x}\n", result3);
    println!("The Groestl512 hash for \"Hello\" is:");
    println!("{:x}\n", result4);
}
