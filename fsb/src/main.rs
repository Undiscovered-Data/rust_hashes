use fsb::{Digest, Fsb160, Fsb224, Fsb256, Fsb384, Fsb512};

fn main() {
    let mut hash1 = Fsb160::new();
    let mut hash2 = Fsb224::new();
    let mut hash3 = Fsb256::new();
    let mut hash4 = Fsb384::new();
    let mut hash5 = Fsb512::new();

    hash1.update("Hello");
    hash2.update("Hello");
    hash3.update("Hello");
    hash4.update("Hello");
    hash5.update("Hello");

    let result1 = hash1.finalize();
    let result2 = hash2.finalize();
    let result3 = hash3.finalize();
    let result4 = hash4.finalize();
    let result5 = hash5.finalize();

    println!("The result for Fsb160 hash \"Hello\" is:");
    println!("{:x}", result1);
    println!("The result for Fsb224 hash \"Hello\" is:");
    println!("{:x}", result2);
    println!("The result for Fsb256 hash \"Hello\" is:");
    println!("{:x}", result3);
    println!("The result for Fsb348 hash \"Hello\" is:");
    println!("{:x}", result4);
    println!("The result for Fsb512 hash \"Hello\" is:");
    println!("{:x}", result5);
}
