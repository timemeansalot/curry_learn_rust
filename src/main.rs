use ed25519_dalek::{Signature, Signer, SigningKey};
use rand::rngs::OsRng;

/// generate the keypair for sign and verify signature
/// the keypair is composed of `secret key` and `verifying key`
/// - the `secret key` is 32 bytes random number for ed25519
/// - the `verifying key` is calculated using the `secret key`
fn generate_key() -> SigningKey {
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    signing_key
}

/// use the keypair to sign the message, return the signature
fn sign(sk: &SigningKey, message: &[u8]) -> Signature {
    sk.sign(message)
}

/// use the keypair and signature to verify the message
/// I don't understand the detailed verify algorithm for now as it need
/// many crypto knowledge
fn verify(sk: &SigningKey, message: &[u8], signature: &Signature) -> bool {
    sk.verify(message, signature).is_ok()
}

fn main() {
    let message: &[u8] = b"This is a test of the tsunami alert system.";
    let signing_key = generate_key();
    let signature = sign(&signing_key, message);
    if verify(&signing_key, message, &signature) {
        println!("verify passed");
    } else {
        println!("verify failed");
    }
}
