mod __tests__;
mod accountable;
mod signable;

use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

#[derive(Debug)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
    keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng {};
        let keypair: Keypair = Keypair::generate(&mut csprng);

        Wallet {
            private_key: format!(
                "0x{}",
                keypair
                    .secret
                    .as_bytes()
                    .iter()
                    .map(|b| format!("{:02x}", b).to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            public_key: format!(
                "0x{}",
                keypair
                    .public
                    .as_bytes()
                    .iter()
                    .map(|b| format!("{:02x}", b).to_string())
                    .collect::<Vec<String>>()
                    .join(""),
            ),
            keypair,
        }
    }
}
