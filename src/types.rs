/// This module contains definitions for all data types.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BitcoinClientType {
    #[serde(rename = "main")]
    Mainnet,
    #[serde(rename = "test")]
    Testnet,
    #[serde(rename = "signet")]
    Signet,
    #[serde(rename = "regtest")]
    Regtest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub mediantime: u64,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub chainwork: String,
    pub size_on_disk: u64,
    pub pruned: bool,
    pub pruneheight: Option<u64>,
    pub softforks: HashMap<String, SoftFork>,
    pub warnings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftFork {
    pub r#type: String,
    pub active: bool,
    pub height: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub confirmations: i32,
    pub strippedsize: Option<u32>,
    pub size: u32,
    pub weight: u32,
    pub height: u64,
    pub version: i32,
    pub version_hex: String,
    pub merkleroot: String,
    pub tx: Vec<String>,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub n_tx: u32,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub hash: String,
    pub confirmations: i32,
    pub height: u64,
    pub version: i32,
    pub version_hex: String,
    pub merkleroot: String,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u64,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub n_tx: u32,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainTip {
    pub height: u64,
    pub hash: String,
    pub branchlen: i32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub txid: String,
    pub hash: String,
    pub version: i32,
    pub size: u32,
    pub vsize: u32,
    pub weight: u32,
    pub locktime: u32,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub hex: String,
    pub blockhash: Option<String>,
    pub confirmations: Option<u32>,
    pub time: Option<u64>,
    pub blocktime: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vin {
    pub txid: Option<String>,
    pub vout: Option<u32>,
    #[serde(alias = "scriptSig")]
    pub script_sig: Option<ScriptSig>,
    pub txinwitness: Option<Vec<String>>,
    pub sequence: u64,
    pub coinbase: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vout {
    pub value: f64,
    pub n: u32,
    #[serde(alias = "scriptPubKey")]
    pub script_pub_key: ScriptPubKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSig {
    pub asm: String,
    pub hex: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptPubKey {
    pub asm: String,
    pub hex: String,
    pub req_sigs: Option<u32>,
    pub r#type: String,
    pub addresses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecodedTransaction {
    pub txid: String,
    pub hash: String,
    pub version: i32,
    pub size: u32,
    pub vsize: u32,
    pub weight: u32,
    pub locktime: u32,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOut {
    pub bestblock: String,
    pub confirmations: u32,
    pub value: f64,
    #[serde(alias = "scriptPubKey")]
    pub script_pub_key: ScriptPubKey,
    pub coinbase: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutSetInfo {
    pub height: u64,
    pub bestblock: String,
    pub transactions: u64,
    pub txouts: u64,
    pub bogosize: u64,
    pub hash_serialized_2: String,
    pub disk_size: u64,
    pub total_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub walletname: String,
    pub walletversion: u32,
    pub balance: f64,
    pub unconfirmed_balance: f64,
    pub immature_balance: f64,
    pub txcount: u32,
    pub keypoololdest: u64,
    pub keypoolsize: u32,
    pub keypoolsize_hd_internal: u32,
    pub unlocked_until: Option<u64>,
    pub paytxfee: f64,
    pub hdseedid: Option<String>,
    pub private_keys_enabled: bool,
    pub avoid_reuse: bool,
    pub scanning: Option<Scanning>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scanning {
    pub duration: u32,
    pub progress: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInfo {
    pub purpose: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateAddress {
    pub isvalid: bool,
    pub address: String,
    #[serde(alias = "scriptPubKey")]
    pub script_pub_key: String,
    pub isscript: bool,
    pub iswitness: bool,
    pub witness_version: Option<u32>,
    pub witness_program: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub address: Option<String>,
    pub label: Option<String>,
    #[serde(alias = "scriptPubKey")]
    pub script_pub_key: String,
    pub amount: f64,
    pub confirmations: u32,
    #[serde(alias = "redeemScript")]
    pub redeem_script: Option<String>,
    #[serde(alias = "witnessScript")]
    pub witness_script: Option<String>,
    pub spendable: bool,
    pub solvable: bool,
    pub desc: Option<String>,
    pub safe: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub version: u32,
    pub subversion: String,
    pub protocolversion: u32,
    pub localservices: String,
    pub localrelay: bool,
    pub timeoffset: i32,
    pub connections: u32,
    pub networkactive: bool,
    pub networks: Vec<NetworkData>,
    pub relayfee: f64,
    pub incrementalfee: f64,
    pub localaddresses: Vec<LocalAddress>,
    pub warnings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkData {
    pub name: String,
    pub limited: bool,
    pub reachable: bool,
    pub proxy: String,
    pub proxy_randomize_credentials: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalAddress {
    pub address: String,
    pub port: u16,
    pub score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub id: u32,
    pub addr: String,
    pub addrbind: Option<String>,
    pub addrlocal: Option<String>,
    pub services: String,
    pub relaytxes: bool,
    pub lastsend: u64,
    pub lastrecv: u64,
    pub bytessent: u64,
    pub bytesrecv: u64,
    pub conntime: u64,
    pub timeoffset: i32,
    pub pingtime: f64,
    pub minping: Option<f64>,
    pub version: u32,
    pub subver: String,
    pub inbound: bool,
    pub addnode: bool,
    pub startingheight: u32,
    pub banscore: u32,
    pub synced_headers: i32,
    pub synced_blocks: i32,
    pub inflight: Vec<u32>,
    pub whitelisted: bool,
    pub permissions: Vec<String>,
    pub minfeefilter: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MempoolInfo {
    pub loaded: bool,
    pub size: u32,
    pub bytes: u64,
    pub usage: u64,
    pub maxmempool: u64,
    pub mempoolminfee: f64,
    pub minrelaytxfee: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeEstimate {
    pub feerate: Option<f64>,
    pub errors: Option<Vec<String>>,
    pub blocks: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningInfo {
    pub blocks: u64,
    pub currentblockweight: Option<u64>,
    pub currentblocktx: Option<u64>,
    pub difficulty: f64,
    pub networkhashps: f64,
    pub pooledtx: u64,
    pub chain: String,
    pub warnings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTxInput {
    pub txid: String,
    pub vout: u32,
    pub sequence: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub hex: String,
    pub complete: bool,
    pub errors: Option<Vec<SigningError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningError {
    pub txid: String,
    pub vout: u32,
    #[serde(alias = "scriptSig")]
    pub script_sig: String,
    pub sequence: u64,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStats {
    pub avgfee: u64,
    pub avgfeerate: u64,
    pub avgtxsize: u64,
    pub blockhash: String,
    pub feerate_percentiles: Vec<u64>,
    pub height: u64,
    pub ins: u64,
    pub maxfee: u64,
    pub maxfeerate: u64,
    pub maxtxsize: u64,
    pub medianfee: u64,
    pub mediantime: u64,
    pub mediantxsize: u64,
    pub minfee: u64,
    pub minfeerate: u64,
    pub mintxsize: u64,
    pub outs: u64,
    pub subsidy: u64,
    pub swtotal_size: u64,
    pub swtotal_weight: u64,
    pub swtxs: u64,
    pub time: u64,
    pub total_out: u64,
    pub total_size: u64,
    pub total_weight: u64,
    pub totalfee: u64,
    pub txs: u64,
    pub utxo_increase: i64,
    pub utxo_size_inc: i64,
}
