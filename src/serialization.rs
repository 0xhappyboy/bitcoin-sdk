use crate::crypto::BitcoinCrypto;
use anyhow::Result;

pub struct Serialization;

impl Serialization {
    // Serializing variable-length integers
    pub fn serialize_varint(value: u64) -> Vec<u8> {
        if value < 0xFD {
            vec![value as u8]
        } else if value <= 0xFFFF {
            let mut bytes = vec![0xFD];
            bytes.extend_from_slice(&(value as u16).to_le_bytes());
            bytes
        } else if value <= 0xFFFFFFFF {
            let mut bytes = vec![0xFE];
            bytes.extend_from_slice(&(value as u32).to_le_bytes());
            bytes
        } else {
            let mut bytes = vec![0xFF];
            bytes.extend_from_slice(&value.to_le_bytes());
            bytes
        }
    }

    // Deserializing variable-length integers
    pub fn deserialize_varint(data: &[u8]) -> Result<(u64, usize)> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("Empty data for varint"));
        }
        match data[0] {
            n if n < 0xFD => Ok((n as u64, 1)),
            0xFD => {
                if data.len() < 3 {
                    return Err(anyhow::anyhow!("Insufficient data for varint(16)"));
                }
                let value = u16::from_le_bytes([data[1], data[2]]) as u64;
                Ok((value, 3))
            }
            0xFE => {
                if data.len() < 5 {
                    return Err(anyhow::anyhow!("Insufficient data for varint(32)"));
                }
                let value = u32::from_le_bytes([data[1], data[2], data[3], data[4]]) as u64;
                Ok((value, 5))
            }
            0xFF => {
                if data.len() < 9 {
                    return Err(anyhow::anyhow!("Insufficient data for varint(64)"));
                }
                let value = u64::from_le_bytes([
                    data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
                ]);
                Ok((value, 9))
            }
            _ => Err(anyhow::anyhow!("Invalid varint prefix")),
        }
    }

    // serialized string
    pub fn serialize_string(s: &str) -> Vec<u8> {
        let bytes = s.as_bytes();
        let mut result = Self::serialize_varint(bytes.len() as u64);
        result.extend_from_slice(bytes);
        result
    }

    // Calculate transaction ID (reversed double SHA256)
    pub fn calculate_txid(tx_hex: &str) -> Result<String> {
        let tx_bytes = hex::decode(tx_hex)?;
        let hash = BitcoinCrypto::double_sha256(&tx_bytes);
        let mut reversed = hash;
        reversed.reverse();
        Ok(hex::encode(reversed))
    }

    // Verifying the Merkle root
    pub fn verify_merkle_root(tx_hashes: &[String], merkle_root: &str) -> bool {
        if tx_hashes.is_empty() {
            return merkle_root
                == "0000000000000000000000000000000000000000000000000000000000000000";
        }
        let mut hashes: Vec<[u8; 32]> = tx_hashes
            .iter()
            .map(|h| {
                let mut bytes = hex::decode(h).unwrap();
                bytes.reverse();
                let mut array = [0u8; 32];
                array.copy_from_slice(&bytes);
                array
            })
            .collect();
        while hashes.len() > 1 {
            let mut new_hashes = Vec::new();
            for chunk in hashes.chunks(2) {
                let left = chunk[0];
                let right = if chunk.len() > 1 { chunk[1] } else { chunk[0] };

                let combined = [&left[..], &right[..]].concat();
                let hash = BitcoinCrypto::double_sha256(&combined);
                new_hashes.push(hash);
            }
            hashes = new_hashes;
        }
        let calculated_root = hashes[0];
        let mut reversed = calculated_root;
        reversed.reverse();
        hex::encode(reversed) == merkle_root
    }
}
