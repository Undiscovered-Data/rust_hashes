use sm3::{Digest, Sm3};

fn main() {
    let mut hash = Sm3::new();
    hash.update("Hello");
    let res = hash.finalize();
    println!("{:x}", res);
}
