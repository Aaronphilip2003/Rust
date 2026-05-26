use hex::encode;
use sha2::{Digest, Sha256};
use hex_literal::hex;
fn main() {
    let hash = Sha256::digest(b"hello world");
    let eq = hash[..] == hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
    println!("{:?}", encode(hash));
    println!("{:?}",eq)
}
