<h1 align="center">
    Bitcoin NetWork SDK
</h1>
<h4 align="center">
基于最基本密码库实现的bitcoin网络SDK,包含了常用的API功能.
</h4>
<p align="center">
  <a href="https://github.com/0xhappyboy/aptos-network-sdk/LICENSE"><img src="https://img.shields.io/badge/License-GPL3.0-d1d1f6.svg?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=googledocs&label=license&logoColor=BEC5C9" alt="License"></a>
</p>
<p align="center">
<a href="./README_zh-CN.md">简体中文</a> | <a href="./README.md">English</a>
</p>

# 案例

## 创建比特币客户端和获取区块链信息

```rust
use bitcoin_lib::{BitcoinClient, BitcoinClientType};

pub async fn example_blockchain_info() -> anyhow::Result<()> {
    let client = BitcoinClient::new_local(BitcoinClientType::Testnet);
    let blockchain_info = client.get_blockchain_info().await?;
    println!("Chain: {}", blockchain_info.chain);
    println!("Blocks: {}", blockchain_info.blocks);
    println!("Best block hash: {}", blockchain_info.bestblockhash);
    println!("Difficulty: {}", blockchain_info.difficulty);

    Ok(())
}
```

## 生成新地址和验证地址

```rust
use bitcoin_lib::{BitcoinClient, BitcoinClientType, BitcoinCrypto};

pub async fn example_address_operations() -> anyhow::Result<()> {
    let client = BitcoinClient::new_local(BitcoinClientType::Testnet);
    let legacy_address = client.get_new_address(Some("test"), Some("legacy")).await?;
    println!("Legacy address: {}", legacy_address);
    let bech32_address = client.get_new_address(Some("test"), Some("bech32")).await?;
    println!("Bech32 address: {}", bech32_address);
    let is_valid_legacy = BitcoinCrypto::validate_address(&legacy_address);
    let is_valid_bech32 = BitcoinCrypto::validate_address(&bech32_address);
    println!("Legacy address valid: {}", is_valid_legacy);
    println!("Bech32 address valid: {}", is_valid_bech32);
    if let Ok(address_type) = BitcoinCrypto::get_address_type(&legacy_address) {
        println!("Address type: {:?}", address_type);
    }
    Ok(())
}
```

## 加密操作 - 从私钥生成地址

```rust
use bitcoin_lib::{BitcoinCrypto, BitcoinClientType};

pub fn example_crypto_operations() -> anyhow::Result<()> {
    let private_key_hex = "private key hex";
    let private_key_bytes = hex::decode(private_key_hex)?;
    let private_key: [u8; 32] = private_key_bytes.try_into().unwrap();
    let compressed_public_key = BitcoinCrypto::private_to_public(&private_key, true)?;
    println!("Compressed public key: {}", hex::encode(&compressed_public_key));
    let p2pkh_address = BitcoinCrypto::public_key_to_p2pkh_address(
        &compressed_public_key,
        BitcoinClientType::Testnet
    )?;
    println!("P2PKH address: {}", p2pkh_address);
    let bech32_address = BitcoinCrypto::public_key_to_bech32_address(
        &compressed_public_key,
        BitcoinClientType::Testnet
    )?;
    println!("Bech32 address: {}", bech32_address);
    let wif = BitcoinCrypto::private_key_to_wif(
        &private_key,
        true,
        BitcoinClientType::Testnet
    )?;
    println!("WIF: {}", wif);
    Ok(())
}
```

## 交易操作 - 查询 UTXO 和创建交易

```rust
use bitcoin_lib::{BitcoinClient, BitcoinClientType};
use std::collections::HashMap;

pub async fn example_transaction_operations() -> anyhow::Result<()> {
    let client = BitcoinClient::new_local(BitcoinClientType::Testnet);
    let unspent = client.list_unspent(1, 9999999, None).await?;
    println!("Found {} unspent outputs:", unspent.len());
    for utxo in &unspent {
        println!("  TXID: {}, Vout: {}, Amount: {}", utxo.txid, utxo.vout, utxo.amount);
    }
    if !unspent.is_empty() {
        let inputs = vec![
            crate::CreateTxInput {
                txid: unspent[0].txid.clone(),
                vout: unspent[0].vout,
                sequence: None,
            }
        ];
        let mut outputs = HashMap::new();
        outputs.insert("tb1qexample...".to_string(), 0.001);
        let raw_tx = client.create_raw_transaction(inputs, outputs).await?;
        println!("Raw transaction: {}", raw_tx);
    }
    Ok(())
}
```

## 序列化和哈希操作

```rust
use bitcoin_lib::{Serialization, BitcoinCrypto};

pub fn example_serialization_operations() -> anyhow::Result<()> {
    let test_data = b"Hello, Bitcoin!";
    let sha256_hash = BitcoinCrypto::sha256(test_data);
    println!("SHA256: {}", hex::encode(sha256_hash));
    let double_sha256_hash = BitcoinCrypto::double_sha256(test_data);
    println!("Double SHA256: {}", hex::encode(double_sha256_hash));
    let hash160 = BitcoinCrypto::hash160(test_data);
    println!("Hash160: {}", hex::encode(hash160));
    let small_int = 150u64;
    let large_int = 100000u64;
    let serialized_small = Serialization::serialize_varint(small_int);
    let serialized_large = Serialization::serialize_varint(large_int);
    println!("Serialized {}: {:?}", small_int, serialized_small);
    println!("Serialized {}: {:?}", large_int, serialized_large);
    let (deserialized_small, bytes_used_small) = Serialization::deserialize_varint(&serialized_small)?;
    let (deserialized_large, bytes_used_large) = Serialization::deserialize_varint(&serialized_large)?;
    println!("Deserialized small: {}, bytes used: {}", deserialized_small, bytes_used_small);
    println!("Deserialized large: {}, bytes used: {}", deserialized_large, bytes_used_large);

    Ok(())
}
```

## 批量 RPC 调用

```rust
use bitcoin_lib::{BitcoinClient, BitcoinClientType};
use serde_json::Value;

pub async fn example_batch_calls() -> anyhow::Result<()> {
    let client = BitcoinClient::new_local(BitcoinClientType::Testnet);
    // 准备多个 RPC 请求
    let requests = vec![
        ("getblockcount".to_string(), Value::Null),
        ("getbestblockhash".to_string(), Value::Null),
        ("getnetworkinfo".to_string(), Value::Null),
        ("getmempoolinfo".to_string(), Value::Null),
    ];
    // 执行批量调用
    let results = client.batch_call(requests).await?;
    for (i, result) in results.iter().enumerate() {
        println!("Result {}: {}", i, result);
    }

    Ok(())
}
```
