use kangarootwelve_xkcp;

fn main() {
    let hash1 = kangarootwelve_xkcp::hash(b"foobarbaz");
    println!("{:?}", &hash1);
    let hash2 = hash1.as_bytes();
    println!("{:?}", &hash2);
    //println!("{:x}", &hash2);
}
