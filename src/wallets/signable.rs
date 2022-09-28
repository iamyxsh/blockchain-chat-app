use ed25519_dalek::Signer;

use super::Wallet;
use ed25519_dalek::Signature;

pub trait Signable {
    fn verify_message(&self, message: String, signature: &Signature) -> bool;
    fn sign_message(&self, message: String) -> Signature;
}

impl Signable for Wallet {
    fn verify_message(&self, message: String, signature: &Signature) -> bool {
        self.keypair.verify(message.as_bytes(), signature).is_ok()
    }

    fn sign_message(&self, message: String) -> Signature {
        self.keypair.sign(message.as_bytes())
    }
}
