extern crate transcipher;

use transcipher::{ TransCipher, TransRoute };

fn main() {
    let cipher = TransCipher::new(9, 3, TransRoute::Spiral);
    println!("{:?}", cipher);
}
