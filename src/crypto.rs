use anyhow::Result;
use bech32::{ToBase32, Variant};
use bs58;
use ripemd::Ripemd160;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

use crate::BitcoinClientType;

pub struct BitcoinCrypto;

impl BitcoinCrypto {
    // Calculating the SHA256 hash
    pub fn sha256(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    // Calculate double SHA256 hash
    pub fn double_sha256(data: &[u8]) -> [u8; 32] {
        let first = Self::sha256(data);
        Self::sha256(&first)
    }

    // Calculating the RIPEMD160 hash
    pub fn ripemd160(data: &[u8]) -> [u8; 20] {
        let mut hasher = Ripemd160::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    // Calculate hash 160 (RIPEMD160(SHA256(data)))
    pub fn hash160(data: &[u8]) -> [u8; 20] {
        let sha256_hash = Self::sha256(data);
        Self::ripemd160(&sha256_hash)
    }

    // Generate a public key from a private key
    pub fn private_to_public(private_key: &[u8; 32], compressed: bool) -> Result<Vec<u8>> {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(private_key)?;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        if compressed {
            Ok(public_key.serialize().to_vec())
        } else {
            Ok(public_key.serialize_uncompressed().to_vec())
        }
    }

    // Creating a P2PKH Address
    pub fn public_key_to_p2pkh_address(
        public_key: &[u8],
        bitcoin_client_type: BitcoinClientType,
    ) -> Result<String> {
        let hash160 = Self::hash160(public_key);
        Self::hash160_to_p2pkh_address(&hash160, bitcoin_client_type)
    }

    // Create a P2PKH address from hash 160
    pub fn hash160_to_p2pkh_address(
        hash160: &[u8; 20],
        bitcoin_client_type: BitcoinClientType,
    ) -> Result<String> {
        let prefix = match bitcoin_client_type {
            BitcoinClientType::Mainnet => 0x00,
            BitcoinClientType::Testnet | BitcoinClientType::Signet | BitcoinClientType::Regtest => {
                0x6f
            }
        };
        let mut data = vec![prefix];
        data.extend_from_slice(hash160);
        let checksum = &Self::double_sha256(&data)[..4];
        data.extend_from_slice(checksum);
        Ok(bs58::encode(data).into_string())
    }

    // Create a P2SH address from hash 160
    pub fn hash160_to_p2sh_address(
        hash160: &[u8; 20],
        bitcoin_client_type: BitcoinClientType,
    ) -> Result<String> {
        let prefix = match bitcoin_client_type {
            BitcoinClientType::Mainnet => 0x05,
            BitcoinClientType::Testnet | BitcoinClientType::Signet | BitcoinClientType::Regtest => {
                0xc4
            }
        };
        let mut data = vec![prefix];
        data.extend_from_slice(hash160);
        let checksum = &Self::double_sha256(&data)[..4];
        data.extend_from_slice(checksum);
        Ok(bs58::encode(data).into_string())
    }

    // Verify address format
    pub fn validate_address(address: &str) -> bool {
        if bech32::decode(address).is_ok() {
            return true;
        }
        Self::decode_base58check(address).is_ok()
    }

    // Decoding a Base58Check address
    pub fn decode_base58check(encoded: &str) -> Result<Vec<u8>> {
        let decoded = bs58::decode(encoded)
            .into_vec()
            .map_err(|e| anyhow::anyhow!("Base58 decode error: {}", e))?;

        if decoded.len() < 4 {
            return Err(anyhow::anyhow!("Data too short for checksum"));
        }
        let payload_len = decoded.len() - 4;
        let data = &decoded[..payload_len];
        let checksum = &decoded[payload_len..];
        let calculated_checksum = &Self::double_sha256(data)[..4];
        if checksum != calculated_checksum {
            return Err(anyhow::anyhow!("Checksum mismatch"));
        }
        Ok(data.to_vec())
    }

    // Get address type
    pub fn get_address_type(address: &str) -> Result<AddressType> {
        if let Ok(decoded) = bech32::decode(address) {
            return match decoded.0.as_str() {
                "bc" => Ok(AddressType::Bech32Mainnet),
                "tb" => Ok(AddressType::Bech32Testnet),
                "bcrt" => Ok(AddressType::Bech32Regtest),
                _ => Err(anyhow::anyhow!("Unknown Bech32 HRP: {}", decoded.0)),
            };
        }
        let decoded = Self::decode_base58check(address)?;
        if decoded.is_empty() {
            return Err(anyhow::anyhow!("Empty address"));
        }
        match decoded[0] {
            0x00 => Ok(AddressType::P2PKHMainnet),
            0x6f => Ok(AddressType::P2PKHTestnet),
            0x05 => Ok(AddressType::P2SHMainnet),
            0xc4 => Ok(AddressType::P2SHTestnet),
            _ => Err(anyhow::anyhow!(
                "Unknown address prefix: 0x{:02x}",
                decoded[0]
            )),
        }
    }

    // Creating a Bech32 Address (P2WPKH)
    pub fn public_key_to_bech32_address(
        public_key: &[u8],
        bitcoin_client_type: BitcoinClientType,
    ) -> Result<String> {
        let hash160 = Self::hash160(public_key);
        Self::hash160_to_bech32_address(&hash160, bitcoin_client_type)
    }

    // Create a Bech32 address from hash 160
    pub fn hash160_to_bech32_address(
        hash160: &[u8; 20],
        bitcoin_client_type: BitcoinClientType,
    ) -> Result<String> {
        let hrp = match bitcoin_client_type {
            BitcoinClientType::Mainnet => "bc",
            BitcoinClientType::Testnet | BitcoinClientType::Signet => "tb",
            BitcoinClientType::Regtest => "bcrt",
        };

        let data = hash160.to_base32();
        bech32::encode(hrp, data, Variant::Bech32)
            .map_err(|e| anyhow::anyhow!("Bech32 encode error: {}", e))
    }

    // Decoding Bech32 addresses
    pub fn decode_bech32_address(address: &str) -> Result<(String, Vec<u8>)> {
        let decoded =
            bech32::decode(address).map_err(|e| anyhow::anyhow!("Bech32 decode error: {}", e))?;
        // to bytes
        let bytes = bech32::FromBase32::from_base32(&decoded.1)
            .map_err(|e| anyhow::anyhow!("Bech32 from_base32 error: {}", e))?;
        Ok((decoded.0, bytes))
    }

    // Decodes a Base58 address (returns version bytes and a hash)
    pub fn decode_address(address: &str) -> Result<(u8, Vec<u8>)> {
        let decoded = Self::decode_base58check(address)?;
        if decoded.len() < 2 {
            return Err(anyhow::anyhow!("Address too short"));
        }
        let version = decoded[0];
        let hash = decoded[1..].to_vec();
        Ok((version, hash))
    }

    // Create a private key in WIF format
    pub fn private_key_to_wif(
        private_key: &[u8; 32],
        compressed: bool,
        bitcoin_client_type: BitcoinClientType,
    ) -> Result<String> {
        let prefix = match bitcoin_client_type {
            BitcoinClientType::Mainnet => 0x80,
            BitcoinClientType::Testnet | BitcoinClientType::Signet | BitcoinClientType::Regtest => {
                0xef
            }
        };
        let mut data = vec![prefix];
        data.extend_from_slice(private_key);

        if compressed {
            data.push(0x01);
        }
        let checksum = &Self::double_sha256(&data)[..4];
        data.extend_from_slice(checksum);
        Ok(bs58::encode(data).into_string())
    }

    // wif to private key
    pub fn wif_to_private_key(wif: &str) -> Result<(Vec<u8>, bool, BitcoinClientType)> {
        let decoded = Self::decode_base58check(wif)?;
        if decoded.len() < 2 {
            return Err(anyhow::anyhow!("WIF too short"));
        }
        let version = decoded[0];
        let (network, compressed) = match version {
            0x80 => (
                BitcoinClientType::Mainnet,
                decoded.len() == 34 && decoded[33] == 0x01,
            ),
            0xef => (
                BitcoinClientType::Testnet,
                decoded.len() == 34 && decoded[33] == 0x01,
            ),
            _ => return Err(anyhow::anyhow!("Unknown WIF version: 0x{:02x}", version)),
        };
        let private_key = if compressed {
            decoded[1..33].to_vec() // Skip version byte and compression flag
        } else {
            decoded[1..].to_vec() // Skip version bytes
        };

        Ok((private_key, compressed, network))
    }

    // Base58 encoding (without checksum)
    pub fn base58_encode(data: &[u8]) -> String {
        bs58::encode(data).into_string()
    }

    // Base58 decoding (without checksum)
    pub fn base58_decode(encoded: &str) -> Result<Vec<u8>> {
        bs58::decode(encoded)
            .into_vec()
            .map_err(|e| anyhow::anyhow!("Base58 decode error: {}", e))
    }

    // Base58Check encoding
    pub fn base58check_encode(data: &[u8]) -> String {
        let checksum = &Self::double_sha256(data)[..4];
        let mut payload = data.to_vec();
        payload.extend_from_slice(checksum);
        bs58::encode(payload).into_string()
    }

    // Base58Check decode
    pub fn base58check_decode(encoded: &str) -> Result<Vec<u8>> {
        Self::decode_base58check(encoded)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AddressType {
    P2PKHMainnet,
    P2PKHTestnet,
    P2SHMainnet,
    P2SHTestnet,
    Bech32Mainnet,
    Bech32Testnet,
    Bech32Regtest,
}
