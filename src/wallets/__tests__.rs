#[cfg(test)]
mod tests {
    use crate::wallets::{signable::Signable, Wallet};

    #[test]
    fn it_should_create_wallet() {
        let wallet = Wallet::new();

        assert_ne!(wallet.public_key, "");
        assert_ne!(wallet.private_key, "");
    }

    #[test]
    fn it_should_sign_and_verify() {
        let wallet = Wallet::new();
        let signature = wallet.sign_message("Signature".to_string());

        assert!(wallet.verify_message("Signature".to_string(), &signature));
    }
}
