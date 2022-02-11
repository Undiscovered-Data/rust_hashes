//use blake3::hash;

fn main() {
    let hash = blake3::hash(b"hello");
    println!("{}", hash);
}
