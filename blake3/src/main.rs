//use blake3::hash;

fn main() {
    let hash = blake3::hash(b"hello");
    println!("The blake3 hash for \"hello\" is:");
    println!("{}", hash);
    
    //You can also do this
    /*
    let mut the_hash = blake3::Hasher::new();
    the_hash.update(b"hello");
    let res = the_hash.finalize();
    println!("The blake3 hash for \"hello\" is:");
    ptinln!("{}", hash);
    */
}
