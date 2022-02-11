use groestl::{Digest, Groestl256, Groestl512};

fn main() {
    let mut hasher1 = Groestl256::default();
    let mut hasher2 = Groestl512::default();

    hasher1.update("Hello");
    hasher2.update("Hello");

    let result1 = hasher1.finalize();
    let result2 = hasher2.finalize();

    println!("{:x}", result1);
    println!("{:x}", result2);
}
