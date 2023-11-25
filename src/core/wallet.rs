use secp256k1::rand::rngs::OsRng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

pub struct Wallet {
    keys: Vec<WalletKeyPair>,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet { keys: Vec::new() }
    }

    pub fn add_key_pair(&mut self, key_pair: WalletKeyPair) -> () {
        self.keys.push(key_pair)
    }

    pub fn get_public_keys(&self) -> Vec<&PublicKey> {
        self.keys
            .iter()
            .map(|key_pair| &key_pair.public_key)
            .collect()
    }
}

pub struct WalletKeyPair {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
}

impl WalletKeyPair {
    pub fn new() -> WalletKeyPair {
        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

        WalletKeyPair {
            secret_key,
            public_key,
        }
    }
}

#[cfg(test)]
mod wallet_test {
    use crate::core::WalletKeyPair;

    use super::Wallet;

    #[test]
    fn wallet_can_have_multiple_key_pairs() {
        let mut wallet = Wallet::new();

        assert_eq!(0, wallet.get_public_keys().len());

        let key_pair = WalletKeyPair::new();
        let public_key = key_pair.public_key.clone();
        wallet.add_key_pair(key_pair);

        let retrived_key_pair = wallet.get_public_keys();
        assert_eq!(1, retrived_key_pair.len());
        assert_eq!(&public_key, retrived_key_pair[0]);

        for _ in 1..=5 {
            wallet.add_key_pair(WalletKeyPair::new());
        }

        assert_eq!(6, wallet.get_public_keys().len());
    }
}
