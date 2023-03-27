use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockResponse {
    pub jsonrpc: String,
    pub result: Result,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub block_height: i64,
    pub block_time: Value,
    pub blockhash: String,
    pub parent_slot: i64,
    pub previous_blockhash: String,
    pub transactions: Vec<Transaction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub meta: Meta,
    pub transaction: Transaction2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub err: Value,
    pub fee: i64,
    pub inner_instructions: Vec<Value>,
    pub log_messages: Vec<Value>,
    pub post_balances: Vec<i64>,
    pub post_token_balances: Vec<Value>,
    pub pre_balances: Vec<i64>,
    pub pre_token_balances: Vec<Value>,
    pub rewards: Value,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "Ok")]
    pub ok: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction2 {
    pub message: Message,
    pub signatures: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub account_keys: Vec<String>,
    pub header: Header,
    pub instructions: Vec<Instruction>,
    pub recent_blockhash: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub num_readonly_signed_accounts: i64,
    pub num_readonly_unsigned_accounts: i64,
    pub num_required_signatures: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub accounts: Vec<i64>,
    pub data: String,
    pub program_id_index: i64,
}
