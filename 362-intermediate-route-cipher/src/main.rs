extern crate transcipher;

use transcipher::{ TransCipher };

fn main() {
    let cipher = TransCipher::new(9, 3);
    println!("{:?}", cipher);
}
