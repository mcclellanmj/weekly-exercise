extern crate transcipher;

use transcipher::{ TransCipher, TransRoute };

fn main() {
    let cipher = TransCipher::new(9, 3);
    println!("{:?}", cipher);
    println!("encoded: {}", cipher.encode(TransRoute::Spiral, "WE ARE DISCOVERED. FLEE AT ONCE"));
}
