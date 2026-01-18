mod wallet;

use wallet::Identity;

fn main() {
    let public_key: Vec<u8> = vec![1, 2, 3, 4, 5]; // Example public key
    let identity = Identity::from_public_key(&public_key);

    println!("DID: {}", identity.did);
    println!("Public Key: {:?}", identity.public_key);
}
