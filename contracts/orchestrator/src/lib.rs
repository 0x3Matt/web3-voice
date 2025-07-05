//! Orchestrator Contract - Platform orchestration logic
// (migrated from monolithic contracts/src/orchestrator.rs)

use near_sdk::{
    env, log, AccountId, PanicOnDefault, Promise,
    collections::{LookupMap, UnorderedMap, UnorderedSet},
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
    NearToken, Gas,
};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

// JSON-compatible view types for ABI generation
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct ContractRegistryView {
    pub voice_token_contract: String,
    pub voice_nft_contract: String,
    pub marketplace_contract: String,
    pub dao_contract: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct PlatformStatsView {
    pub total_voice_tokens_minted: String,
    pub total_voice_nfts_minted: String,
    pub total_marketplace_transactions: String,
    pub total_dao_proposals: String,
    pub total_active_users: String,
    pub platform_revenue: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct UserActivityView {
    pub account_id: String,
    pub voice_tokens_balance: String,
    pub voice_nfts_owned: String,
    pub marketplace_transactions: String,
    pub dao_participation: String,
    pub last_activity: String,
    pub reputation_score: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct PlatformOperationView {
    pub id: String,
    pub operation_type: String,
    pub initiator: String,
    pub parameters: HashMap<String, String>,
    pub status: String,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub result: Option<String>,
}

// Internal contract types (no JsonSchema needed)
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractRegistry {
    pub voice_token_contract: AccountId,
    pub voice_nft_contract: AccountId,
    pub marketplace_contract: AccountId,
    pub dao_contract: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformStats {
    pub total_voice_tokens_minted: U128,
    pub total_voice_nfts_minted: U128,
    pub total_marketplace_transactions: U128,
    pub total_dao_proposals: U128,
    pub total_active_users: U128,
    pub platform_revenue: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserActivity {
    pub account_id: AccountId,
    pub voice_tokens_balance: U128,
    pub voice_nfts_owned: U128,
    pub marketplace_transactions: U128,
    pub dao_participation: U128,
    pub last_activity: U64,
    pub reputation_score: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformOperation {
    pub operation_id: U128,
    pub operation_type: String,
    pub initiated_by: AccountId,
    pub target_contract: AccountId,
    pub parameters: String, // JSON string
    pub status: String, // "pending", "completed", "failed"
    pub created_at: U64,
    pub completed_at: Option<U64>,
    pub result: Option<String>,
}


/// The Orchestrator contract for managing platform-wide operations
#[near_sdk::near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Orchestrator {
    /// Contract owner
    pub owner_id: AccountId,
    /// Registry of platform contracts
    pub contract_registry: ContractRegistry,
    /// Platform statistics
    pub platform_stats: PlatformStats,
    /// User activity tracking
    pub user_activities: LookupMap<AccountId, UserActivity>,
    /// Active users set
    pub active_users: UnorderedSet<AccountId>,
    /// Platform operations log
    pub operations: UnorderedMap<U128, PlatformOperation>,
    /// Total operations counter
    pub total_operations: U128,
    /// Platform configuration
    pub platform_config: LookupMap<String, String>,
    /// Revenue tracking
    pub revenue_by_contract: LookupMap<AccountId, U128>,
}

#[near_sdk::near_bindgen]
impl Orchestrator {
    /// Initialize the orchestrator
    #[init]
    pub fn new(
        owner_id: AccountId,
        voice_token_contract: AccountId,
        voice_nft_contract: AccountId,
        marketplace_contract: AccountId,
        dao_contract: AccountId,
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        let contract_registry = ContractRegistry {
            voice_token_contract,
            voice_nft_contract,
            marketplace_contract,
            dao_contract,
        };

        let platform_stats = PlatformStats {
            total_voice_tokens_minted: U128(0),
            total_voice_nfts_minted: U128(0),
            total_marketplace_transactions: U128(0),
            total_dao_proposals: U128(0),
            total_active_users: U128(0),
            platform_revenue: U128(0),
        };

        let this = Self {
            owner_id: owner_id.clone(),
            contract_registry,
            platform_stats,
            user_activities: LookupMap::new(b"ua".to_vec()),
            active_users: UnorderedSet::new(b"au".to_vec()),
            operations: UnorderedMap::new(b"op".to_vec()),
            total_operations: U128(0),
            platform_config: LookupMap::new(b"pc".to_vec()),
            revenue_by_contract: LookupMap::new(b"rbc".to_vec()),
        };

        log!("Orchestrator contract deployed. Owner: {}", owner_id);

        this
    }

    /// Update contract registry
    pub fn update_contract_registry(&mut self, contract_registry: ContractRegistryView) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can update registry");
        
        // Convert view type to internal type
        let internal_registry = ContractRegistry {
            voice_token_contract: contract_registry.voice_token_contract.parse().expect("Invalid voice token contract ID"),
            voice_nft_contract: contract_registry.voice_nft_contract.parse().expect("Invalid voice NFT contract ID"),
            marketplace_contract: contract_registry.marketplace_contract.parse().expect("Invalid marketplace contract ID"),
            dao_contract: contract_registry.dao_contract.parse().expect("Invalid DAO contract ID"),
        };
        
        self.contract_registry = internal_registry;
        log!("Contract registry updated");
    }

    /// Record user activity
    pub fn record_user_activity(&mut self, account_id: AccountId, activity_type: String, value: U128) {
        // Only registered contracts can record activity
        let predecessor = env::predecessor_account_id();
        assert!(
            predecessor == self.contract_registry.voice_token_contract ||
            predecessor == self.contract_registry.voice_nft_contract ||
            predecessor == self.contract_registry.marketplace_contract ||
            predecessor == self.contract_registry.dao_contract ||
            predecessor == self.owner_id,
            "Unauthorized to record activity"
        );

        let mut user_activity = self.user_activities.get(&account_id).unwrap_or_else(|| UserActivity {
            account_id: account_id.clone(),
            voice_tokens_balance: U128(0),
            voice_nfts_owned: U128(0),
            marketplace_transactions: U128(0),
            dao_participation: U128(0),
            last_activity: U64(env::block_timestamp()),
            reputation_score: U128(0),
        });

        // Update activity based on type
        match activity_type.as_str() {
            "voice_token_transaction" => {
                user_activity.voice_tokens_balance = value;
            },
            "voice_nft_mint" => {
                user_activity.voice_nfts_owned.0 += 1;
                user_activity.reputation_score.0 += 10;
            },
            "marketplace_transaction" => {
                user_activity.marketplace_transactions.0 += 1;
                user_activity.reputation_score.0 += 5;
            },
            "dao_vote" => {
                user_activity.dao_participation.0 += 1;
                user_activity.reputation_score.0 += 15;
            },
            _ => {}
        }

        user_activity.last_activity = U64(env::block_timestamp());
        self.user_activities.insert(&account_id, &user_activity);

        // Add to active users if not already present
        if !self.active_users.contains(&account_id) {
            self.active_users.insert(&account_id);
            self.platform_stats.total_active_users.0 += 1;
        }

        log!("Recorded activity: {} for user {}", activity_type, account_id);
    }

    /// Update platform statistics
    pub fn update_platform_stats(&mut self, stat_type: String, value: U128) {
        // Only registered contracts can update stats
        let predecessor = env::predecessor_account_id();
        assert!(
            predecessor == self.contract_registry.voice_token_contract ||
            predecessor == self.contract_registry.voice_nft_contract ||
            predecessor == self.contract_registry.marketplace_contract ||
            predecessor == self.contract_registry.dao_contract ||
            predecessor == self.owner_id,
            "Unauthorized to update stats"
        );

        match stat_type.as_str() {
            "voice_tokens_minted" => {
                self.platform_stats.total_voice_tokens_minted.0 += value.0;
            },
            "voice_nfts_minted" => {
                self.platform_stats.total_voice_nfts_minted.0 += value.0;
            },
            "marketplace_transactions" => {
                self.platform_stats.total_marketplace_transactions.0 += value.0;
            },
            "dao_proposals" => {
                self.platform_stats.total_dao_proposals.0 += value.0;
            },
            "platform_revenue" => {
                self.platform_stats.platform_revenue.0 += value.0;
                // Track revenue by contract
                let current_revenue = self.revenue_by_contract.get(&predecessor).unwrap_or(U128(0));
                self.revenue_by_contract.insert(&predecessor, &U128(current_revenue.0 + value.0));
            },
            _ => {}
        }

        log!("Updated platform stat: {} by {}", stat_type, value.0);
    }

    /// Execute a cross-contract call
    pub fn execute_cross_contract_call(
        &mut self,
        target_contract: AccountId,
        method_name: String,
        args: String,
        deposit: U128,
        gas: U64,
    ) -> Promise {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can execute cross-contract calls");

        let operation_id = self.total_operations.0 + 1;
        let operation = PlatformOperation {
            operation_id: U128(operation_id),
            operation_type: format!("cross_contract_call:{}", method_name),
            initiated_by: env::predecessor_account_id(),
            target_contract: target_contract.clone(),
            parameters: args.clone(),
            status: "pending".to_string(),
            created_at: U64(env::block_timestamp()),
            completed_at: None,
            result: None,
        };

        self.operations.insert(&U128(operation_id), &operation);
        self.total_operations.0 += 1;

        log!("Executing cross-contract call: {} on {}", method_name, target_contract);

        Promise::new(target_contract)
            .function_call(
                method_name,
                args.into_bytes(),
                NearToken::from_yoctonear(deposit.0),
                Gas::from_gas(gas.0),
            )
    }

    /// Batch mint voice NFTs
    pub fn batch_mint_voice_nfts(
        &mut self,
        recipients: Vec<AccountId>,
        token_ids: Vec<String>,
        metadatas: Vec<String>, // JSON strings
    ) -> Promise {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can batch mint");
        assert_eq!(recipients.len(), token_ids.len(), "Recipients and token IDs length mismatch");
        assert_eq!(recipients.len(), metadatas.len(), "Recipients and metadatas length mismatch");

        let operation_id = self.total_operations.0 + 1;
        let operation = PlatformOperation {
            operation_id: U128(operation_id),
            operation_type: "batch_mint_nfts".to_string(),
            initiated_by: env::predecessor_account_id(),
            target_contract: self.contract_registry.voice_nft_contract.clone(),
            parameters: format!("recipients: {:?}, token_ids: {:?}", recipients, token_ids),
            status: "pending".to_string(),
            created_at: U64(env::block_timestamp()),
            completed_at: None,
            result: None,
        };

        self.operations.insert(&U128(operation_id), &operation);
        self.total_operations.0 += 1;

        // For now, just log the batch operation
        // In a real implementation, this would make multiple cross-contract calls
        log!("Batch minting {} voice NFTs", recipients.len());

        Promise::new(env::current_account_id())
    }

    /// Get platform statistics
    pub fn get_platform_stats(&self) -> PlatformStatsView {
        self.platform_stats_to_view(self.platform_stats.clone())
    }

    /// Get user activity
    pub fn get_user_activity(&self, account_id: AccountId) -> Option<UserActivityView> {
        self.user_activities.get(&account_id).map(|activity| self.user_activity_to_view(activity))
    }

    /// Get contract registry
    pub fn get_contract_registry(&self) -> ContractRegistryView {
        self.contract_registry_to_view(self.contract_registry.clone())
    }

    /// Get platform operations
    pub fn get_operations(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<PlatformOperationView> {
        let start = from_index.map(|i| i.0 as usize).unwrap_or(0);
        let limit = limit.unwrap_or(10) as usize;

        self.operations
            .values()
            .skip(start)
            .take(limit)
            .map(|op| self.platform_operation_to_view(op))
            .collect()
    }

    /// Get active users
    pub fn get_active_users(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<String> {
        let start = from_index.map(|i| i.0 as usize).unwrap_or(0);
        let limit = limit.unwrap_or(50) as usize;

        self.active_users
            .to_vec()
            .into_iter()
            .skip(start)
            .take(limit)
            .map(|id| id.to_string())
            .collect()
    }

    /// Get revenue by contract
    pub fn get_revenue_by_contract(&self, contract_id: AccountId) -> String {
        self.revenue_by_contract.get(&contract_id).unwrap_or(U128(0)).0.to_string()
    }

    /// Set platform configuration
    pub fn set_platform_config(&mut self, key: String, value: String) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can set config");
        self.platform_config.insert(&key, &value);
        log!("Set platform config: {} = {}", key, value);
    }

    /// Get platform configuration
    pub fn get_platform_config(&self, key: String) -> Option<String> {
        self.platform_config.get(&key)
    }

    /// Get total operations
    pub fn get_total_operations(&self) -> String {
        self.total_operations.0.to_string()
    }

    /// Emergency pause (owner only)
    pub fn emergency_pause(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can pause");
        self.platform_config.insert(&"emergency_paused".to_string(), &"true".to_string());
        log!("Emergency pause activated");
    }

    /// Emergency unpause (owner only)
    pub fn emergency_unpause(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can unpause");
        self.platform_config.insert(&"emergency_paused".to_string(), &"false".to_string());
        log!("Emergency pause deactivated");
    }

    /// Check if platform is paused
    pub fn is_paused(&self) -> bool {
        self.platform_config.get(&"emergency_paused".to_string())
            .map(|v| v == "true")
            .unwrap_or(false)
    }

    // Helper methods to convert internal types to view types
    fn platform_stats_to_view(&self, stats: PlatformStats) -> PlatformStatsView {
        PlatformStatsView {
            total_voice_tokens_minted: stats.total_voice_tokens_minted.0.to_string(),
            total_voice_nfts_minted: stats.total_voice_nfts_minted.0.to_string(),
            total_marketplace_transactions: stats.total_marketplace_transactions.0.to_string(),
            total_dao_proposals: stats.total_dao_proposals.0.to_string(),
            total_active_users: stats.total_active_users.0.to_string(),
            platform_revenue: stats.platform_revenue.0.to_string(),
        }
    }

    fn user_activity_to_view(&self, activity: UserActivity) -> UserActivityView {
        UserActivityView {
            account_id: activity.account_id.to_string(),
            voice_tokens_balance: activity.voice_tokens_balance.0.to_string(),
            voice_nfts_owned: activity.voice_nfts_owned.0.to_string(),
            marketplace_transactions: activity.marketplace_transactions.0.to_string(),
            dao_participation: activity.dao_participation.0.to_string(),
            last_activity: activity.last_activity.0.to_string(),
            reputation_score: activity.reputation_score.0.to_string(),
        }
    }

    fn contract_registry_to_view(&self, registry: ContractRegistry) -> ContractRegistryView {
        ContractRegistryView {
            voice_token_contract: registry.voice_token_contract.to_string(),
            voice_nft_contract: registry.voice_nft_contract.to_string(),
            marketplace_contract: registry.marketplace_contract.to_string(),
            dao_contract: registry.dao_contract.to_string(),
        }
    }

    fn platform_operation_to_view(&self, operation: PlatformOperation) -> PlatformOperationView {
        PlatformOperationView {
            id: operation.operation_id.0.to_string(),
            operation_type: operation.operation_type,
            initiator: operation.initiated_by.to_string(),
            parameters: HashMap::new(), // Could parse JSON parameters if needed
            status: operation.status,
            created_at: operation.created_at.0.to_string(),
            completed_at: operation.completed_at.map(|t| t.0.to_string()),
            result: operation.result,
        }
    }
}
