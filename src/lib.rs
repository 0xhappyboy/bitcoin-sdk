mod crypto;
mod serialization;
mod types;

use base64::{Engine, prelude::BASE64_STANDARD};
pub use crypto::*;
pub use types::*;

use anyhow::{Result, anyhow};
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BitcoinClient {
    client: Client,
    url: String,
    auth: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BitcoinNetWorkRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BitcoinNetWorkResponse<T> {
    jsonrpc: String,
    id: u64,
    result: Option<T>,
    error: Option<RpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RpcError {
    code: i32,
    message: String,
}

impl BitcoinClient {
    pub fn new(url: &str, username: &str, password: &str) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        let auth = format!("{}:{}", username, password);
        let auth = format!("Basic {}", BASE64_STANDARD.encode(&auth));
        BitcoinClient {
            client,
            url: url.to_string(),
            auth,
        }
    }

    pub fn new_local(network: BitcoinClientType) -> Self {
        let (url, port) = match network {
            BitcoinClientType::Mainnet => ("http://127.0.0.1", 8332),
            BitcoinClientType::Testnet => ("http://127.0.0.1", 18332),
            BitcoinClientType::Signet => ("http://127.0.0.1", 38332),
            BitcoinClientType::Regtest => ("http://127.0.0.1", 18443),
        };
        Self::new(&format!("{}:{}", url, port), "bitcoin", "password")
    }

    async fn call<T: for<'de> Deserialize<'de>>(&self, method: &str, params: Value) -> Result<T> {
        let request = BitcoinNetWorkRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: method.to_string(),
            params,
        };
        let response: Response = self
            .client
            .post(&self.url)
            .header("Authorization", &self.auth)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow!("HTTP error {}: {}", status, text));
        }
        let rpc_response: BitcoinNetWorkResponse<T> = response.json().await?;
        if let Some(error) = rpc_response.error {
            return Err(anyhow!("RPC error {}: {}", error.code, error.message));
        }
        rpc_response
            .result
            .ok_or_else(|| anyhow!("No result in response"))
    }

    pub async fn get_blockchain_info(&self) -> Result<BlockchainInfo> {
        self.call("getblockchaininfo", Value::Null).await
    }

    pub async fn get_block_count(&self) -> Result<u64> {
        self.call("getblockcount", Value::Null).await
    }

    pub async fn get_best_block_hash(&self) -> Result<String> {
        self.call("getbestblockhash", Value::Null).await
    }

    pub async fn get_block(&self, block_hash: &str, verbosity: u8) -> Result<Block> {
        self.call("getblock", json!([block_hash, verbosity])).await
    }

    pub async fn get_block_hash(&self, height: u64) -> Result<String> {
        self.call("getblockhash", json!([height])).await
    }

    pub async fn get_block_header(&self, block_hash: &str, verbose: bool) -> Result<BlockHeader> {
        self.call("getblockheader", json!([block_hash, verbose]))
            .await
    }

    pub async fn get_chain_tips(&self) -> Result<Vec<ChainTip>> {
        self.call("getchaintips", Value::Null).await
    }

    pub async fn get_difficulty(&self) -> Result<f64> {
        self.call("getdifficulty", Value::Null).await
    }

    pub async fn get_raw_transaction(&self, txid: &str, verbose: bool) -> Result<Transaction> {
        self.call("getrawtransaction", json!([txid, verbose])).await
    }

    pub async fn decode_raw_transaction(&self, tx_hex: &str) -> Result<DecodedTransaction> {
        self.call("decoderawtransaction", json!([tx_hex])).await
    }

    pub async fn send_raw_transaction(&self, tx_hex: &str) -> Result<String> {
        self.call("sendrawtransaction", json!([tx_hex])).await
    }

    pub async fn get_tx_out(
        &self,
        txid: &str,
        vout: u32,
        include_mempool: bool,
    ) -> Result<Option<TxOut>> {
        self.call("gettxout", json!([txid, vout, include_mempool]))
            .await
    }

    pub async fn get_tx_out_set_info(&self) -> Result<TxOutSetInfo> {
        self.call("gettxoutsetinfo", Value::Null).await
    }

    pub async fn get_wallet_info(&self) -> Result<WalletInfo> {
        self.call("getwalletinfo", Value::Null).await
    }

    pub async fn get_balance(
        &self,
        dummy: &str,
        min_conf: i32,
        include_watchonly: bool,
    ) -> Result<f64> {
        self.call("getbalance", json!([dummy, min_conf, include_watchonly]))
            .await
    }

    pub async fn get_new_address(
        &self,
        label: Option<&str>,
        address_type: Option<&str>,
    ) -> Result<String> {
        let params = match (label, address_type) {
            (Some(l), Some(t)) => json!([l, t]),
            (Some(l), None) => json!([l]),
            _ => Value::Null,
        };
        self.call("getnewaddress", params).await
    }

    pub async fn get_addresses_by_label(
        &self,
        label: &str,
    ) -> Result<HashMap<String, AddressInfo>> {
        self.call("getaddressesbylabel", json!([label])).await
    }

    pub async fn validate_address(&self, address: &str) -> Result<ValidateAddress> {
        self.call("validateaddress", json!([address])).await
    }

    pub async fn send_to_address(&self, address: &str, amount: f64) -> Result<String> {
        self.call("sendtoaddress", json!([address, amount])).await
    }

    pub async fn list_unspent(
        &self,
        min_conf: i32,
        max_conf: i32,
        addresses: Option<Vec<&str>>,
    ) -> Result<Vec<Utxo>> {
        let params = match addresses {
            Some(addrs) => json!([min_conf, max_conf, addrs]),
            None => json!([min_conf, max_conf]),
        };
        self.call("listunspent", params).await
    }

    pub async fn get_network_info(&self) -> Result<NetworkInfo> {
        self.call("getnetworkinfo", Value::Null).await
    }

    pub async fn get_peer_info(&self) -> Result<Vec<PeerInfo>> {
        self.call("getpeerinfo", Value::Null).await
    }

    pub async fn get_network_hash_ps(
        &self,
        nblocks: Option<i32>,
        height: Option<i32>,
    ) -> Result<f64> {
        let params = match (nblocks, height) {
            (Some(n), Some(h)) => json!([n, h]),
            (Some(n), None) => json!([n]),
            _ => Value::Null,
        };
        self.call("getnetworkhashps", params).await
    }

    pub async fn get_mempool_info(&self) -> Result<MempoolInfo> {
        self.call("getmempoolinfo", Value::Null).await
    }

    pub async fn get_raw_mempool(&self, verbose: bool) -> Result<Vec<String>> {
        self.call("getrawmempool", json!([verbose])).await
    }

    pub async fn estimate_smart_fee(&self, conf_target: i32) -> Result<FeeEstimate> {
        self.call("estimatesmartfee", json!([conf_target])).await
    }

    pub async fn get_mining_info(&self) -> Result<MiningInfo> {
        self.call("getmininginfo", Value::Null).await
    }

    pub async fn generate_to_address(&self, nblocks: u32, address: &str) -> Result<Vec<String>> {
        self.call("generatetoaddress", json!([nblocks, address]))
            .await
    }

    pub async fn create_raw_transaction(
        &self,
        inputs: Vec<CreateTxInput>,
        outputs: HashMap<String, f64>,
    ) -> Result<String> {
        self.call("createrawtransaction", json!([inputs, outputs]))
            .await
    }

    pub async fn sign_raw_transaction_with_wallet(
        &self,
        tx_hex: &str,
    ) -> Result<SignedTransaction> {
        self.call("signrawtransactionwithwallet", json!([tx_hex]))
            .await
    }

    pub async fn get_block_stats(&self, height: u64) -> Result<BlockStats> {
        self.call("getblockstats", json!([height])).await
    }

    pub async fn batch_call(&self, requests: Vec<(String, Value)>) -> Result<Vec<Value>> {
        let batch_requests: Vec<BitcoinNetWorkRequest> = requests
            .into_iter()
            .enumerate()
            .map(|(i, (method, params))| BitcoinNetWorkRequest {
                jsonrpc: "2.0".to_string(),
                id: i as u64,
                method,
                params,
            })
            .collect();
        let response: Response = self
            .client
            .post(&self.url)
            .header("Authorization", &self.auth)
            .header("Content-Type", "application/json")
            .json(&batch_requests)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }
        let responses: Vec<BitcoinNetWorkResponse<Value>> = response.json().await?;
        let mut results = Vec::new();
        for response in responses {
            if let Some(error) = response.error {
                return Err(anyhow!("RPC error {}: {}", error.code, error.message));
            }
            results.push(response.result.unwrap_or(Value::Null));
        }
        Ok(results)
    }
}
