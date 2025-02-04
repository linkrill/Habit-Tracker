use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, env, near_bindgen, serde_json::json, PanicOnDefault, NearSchema};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Contract {
    owner: AccountId,
    notifications_count: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, NearSchema)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct Transaction {
    pub block_timestamp: u64,
    pub block_height: u64,
    pub tx_hash: String,
    pub signer_id: String,
    pub nonce: u64,
    pub receiver_id: String,
    pub receipt_id: String,
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new() -> Self {
        let contract = Self {
            owner: env::predecessor_account_id(),
            notifications_count: 0,
        };
        contract
    }

    /**
     * Get count
     */
    pub fn get_notifications_count(&self) -> u32 {
        self.notifications_count
    }

    /**
     * Notification call function
     */
    pub fn send_notification(&mut self, tx_data: Transaction) -> u32 {
        let log_message = json!(tx_data).to_string();

        self.notifications_count += 1;
        env::log_str(&log_message[..]);

        self.notifications_count.into()
    }
}
