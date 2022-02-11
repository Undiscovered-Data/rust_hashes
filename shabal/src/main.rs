use shabal::{Shabal512, Digest};

fn main() {
    let mut hasher = Shabal512::new();
    hasher.update("Hello");
    let res = hasher.finalize();
    println!("{:x}", res);
}
