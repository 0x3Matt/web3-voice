// Web3Voice Smart Contract Suite
// A comprehensive ecosystem for voice content tokenization, governance, and marketplace

pub mod voice_token;
pub mod voice_nft;
pub mod marketplace;
pub mod dao;

// Re-export main contracts
pub use voice_token::VoiceToken;
pub use voice_nft::VoiceNFT;
pub use marketplace::VoiceMarketplace;
pub use dao::VoiceDAO;

use near_sdk::{
    env, log, near, require, AccountId, BorshStorageKey, PanicOnDefault, Promise,
    collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet},
    json_types::{Base64VecU8, U128, U64},
    serde::{Deserialize, Serialize},
    Gas, NearToken,
};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

// Constants
const VOICE_TOKEN_SUPPLY: u128 = 1_000_000_000_000_000_000_000_000_000; // 1 billion VOICE tokens
const MINT_COST: u128 = 1_000_000_000_000_000_000_000_000; // 1 VOICE token
const ROYALTY_PERCENT: u32 = 1000; // 10% royalty
const DAO_VOTING_PERIOD: u64 = 604800000000000; // 7 days in nanoseconds
const MIN_PROPOSAL_DEPOSIT: u128 = 100_000_000_000_000_000_000_000_000; // 100 VOICE tokens

// Storage keys
#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
enum StorageKey {
    VoiceTokens,
    VoiceNFTs,
    VoiceNFTMetadata,
    VoiceNFTOwners,
    VoiceNFTRoyalties,
    MarketListings,
    DAOProposals,
    DAOVotes,
    DAOMembers,
    UserProfiles,
    VoiceAnalytics,
    StakingRewards,
    TokenApprovals,
    NFTApprovals,
    ContractAddresses,
}

// Data structures
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VoiceNFTMetadata {
    pub title: String,
    pub description: String,
    pub creator: AccountId,
    pub category: String,
    pub duration: u64, // in seconds
    pub file_size: u64, // in bytes
    pub ipfs_hash: String,
    pub audio_ipfs_hash: String,
    pub tags: Vec<String>,
    pub ai_analysis: Option<AIAnalysis>,
    pub access_type: AccessType,
    pub created_at: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AIAnalysis {
    pub transcription: String,
    pub sentiment: String, // "positive", "neutral", "negative"
    pub topics: Vec<String>,
    pub insights: Vec<String>,
    pub quality_score: u8, // 1-100
    pub authenticity_score: u8, // 1-100
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum AccessType {
    Public,
    TokenGated { min_tokens: u128 },
    DAOOnly,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UserProfile {
    pub account_id: AccountId,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_ipfs: Option<String>,
    pub total_voice_nfts: u64,
    pub total_earnings: u128,
    pub reputation_score: u32,
    pub verified: bool,
    pub joined_at: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VoiceAnalytics {
    pub token_id: String,
    pub total_plays: u64,
    pub unique_listeners: u64,
    pub earnings: u128,
    pub geographic_data: HashMap<String, u64>, // country -> listener count
    pub sentiment_breakdown: HashMap<String, u64>, // sentiment -> count
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ContractAddresses {
    pub voice_token: Option<AccountId>,
    pub voice_nft: Option<AccountId>,
    pub marketplace: Option<AccountId>,
    pub dao: Option<AccountId>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PlatformStats {
    pub total_voice_nfts: u64,
    pub total_users: u64,
    pub total_volume: u128,
    pub total_plays: u64,
    pub dao_members: u64,
    pub active_proposals: u64,
    pub last_updated: u64,
}

// Main orchestrator contract
#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct VoiceOrchestrator {
    pub owner: AccountId,
    pub contract_addresses: ContractAddresses,
    pub user_profiles: LookupMap<AccountId, UserProfile>,
    pub voice_analytics: LookupMap<String, VoiceAnalytics>,
    pub platform_stats: PlatformStats,
    pub paused: bool,
    pub version: String,
    pub deployed_at: u64,
}

#[near]
impl VoiceOrchestrator {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self {
            owner: owner.clone(),
            contract_addresses: ContractAddresses {
                voice_token: None,
                voice_nft: None,
                marketplace: None,
                dao: None,
            },
            user_profiles: LookupMap::new(StorageKey::UserProfiles),
            voice_analytics: LookupMap::new(StorageKey::VoiceAnalytics),
            platform_stats: PlatformStats {
                total_voice_nfts: 0,
                total_users: 0,
                total_volume: 0,
                total_plays: 0,
                dao_members: 0,
                active_proposals: 0,
                last_updated: env::block_timestamp(),
            },
            paused: false,
            version: "1.0.0".to_string(),
            deployed_at: env::block_timestamp(),
        }
    }

    // ===================
    // CONTRACT MANAGEMENT
    // ===================

    pub fn set_contract_address(&mut self, contract_type: String, address: AccountId) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can set contract addresses");
        
        let address_str = address.to_string();
        match contract_type.as_str() {
            "voice_token" => self.contract_addresses.voice_token = Some(address),
            "voice_nft" => self.contract_addresses.voice_nft = Some(address),
            "marketplace" => self.contract_addresses.marketplace = Some(address),
            "dao" => self.contract_addresses.dao = Some(address),
            _ => env::panic_str("Invalid contract type"),
        }
        
        log!("Set {} contract address to {}", contract_type, address_str);
    }

    pub fn get_contract_addresses(&self) -> ContractAddresses {
        self.contract_addresses.clone()
    }

    // ===================
    // USER PROFILE METHODS
    // ===================

    pub fn create_user_profile(&mut self, display_name: Option<String>, bio: Option<String>, avatar_ipfs: Option<String>) {
        let account_id = env::predecessor_account_id();
        
        let profile = UserProfile {
            account_id: account_id.clone(),
            display_name,
            bio,
            avatar_ipfs,
            total_voice_nfts: 0,
            total_earnings: 0,
            reputation_score: 0,
            verified: false,
            joined_at: env::block_timestamp(),
        };
        
        self.user_profiles.insert(&account_id, &profile);
        self.platform_stats.total_users += 1;
        self.platform_stats.last_updated = env::block_timestamp();
        
        log!("User profile created for {}", account_id);
    }

    pub fn update_user_profile(&mut self, display_name: Option<String>, bio: Option<String>, avatar_ipfs: Option<String>) {
        let account_id = env::predecessor_account_id();
        
        if let Some(mut profile) = self.user_profiles.get(&account_id) {
            if let Some(name) = display_name {
                profile.display_name = Some(name);
            }
            if let Some(bio_text) = bio {
                profile.bio = Some(bio_text);
            }
            if let Some(avatar) = avatar_ipfs {
                profile.avatar_ipfs = Some(avatar);
            }
            
            self.user_profiles.insert(&account_id, &profile);
            log!("User profile updated for {}", account_id);
        } else {
            env::panic_str("User profile not found");
        }
    }

    pub fn get_user_profile(&self, account_id: AccountId) -> Option<UserProfile> {
        self.user_profiles.get(&account_id)
    }

    pub fn verify_user(&mut self, account_id: AccountId) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can verify users");
        
        if let Some(mut profile) = self.user_profiles.get(&account_id) {
            profile.verified = true;
            self.user_profiles.insert(&account_id, &profile);
            log!("User {} verified", account_id);
        }
    }

    // ===================
    // ANALYTICS METHODS
    // ===================

    pub fn record_voice_play(&mut self, token_id: String, listener: AccountId, country: Option<String>, duration: u64) {
        let caller = env::predecessor_account_id();
        
        // Only allow the NFT contract or owner to record plays
        require!(
            caller == self.owner || 
            (self.contract_addresses.voice_nft.is_some() && caller == self.contract_addresses.voice_nft.clone().unwrap()),
            "Not authorized to record plays"
        );
        
        let mut analytics = self.voice_analytics.get(&token_id).unwrap_or_else(|| {
            VoiceAnalytics {
                token_id: token_id.clone(),
                total_plays: 0,
                unique_listeners: 0,
                earnings: 0,
                geographic_data: HashMap::new(),
                sentiment_breakdown: HashMap::new(),
            }
        });
        
        analytics.total_plays += 1;
        
        if let Some(country) = country {
            let count = analytics.geographic_data.get(&country).unwrap_or(&0) + 1;
            analytics.geographic_data.insert(country, count);
        }
        
        self.voice_analytics.insert(&token_id, &analytics);
        self.platform_stats.total_plays += 1;
        self.platform_stats.last_updated = env::block_timestamp();
        
        log!("Recorded play for token {} by {}", token_id, listener);
    }

    pub fn get_voice_analytics(&self, token_id: String) -> Option<VoiceAnalytics> {
        self.voice_analytics.get(&token_id)
    }

    pub fn update_nft_earnings(&mut self, token_id: String, earnings: u128) {
        let caller = env::predecessor_account_id();
        
        // Only allow the marketplace contract or owner to update earnings
        require!(
            caller == self.owner || 
            (self.contract_addresses.marketplace.is_some() && caller == self.contract_addresses.marketplace.clone().unwrap()),
            "Not authorized to update earnings"
        );
        
        if let Some(mut analytics) = self.voice_analytics.get(&token_id) {
            analytics.earnings += earnings;
            self.voice_analytics.insert(&token_id, &analytics);
            self.platform_stats.total_volume += earnings;
            self.platform_stats.last_updated = env::block_timestamp();
        }
    }

    // ===================
    // PLATFORM STATS
    // ===================

    pub fn update_platform_stats(&mut self, nft_count: Option<u64>, dao_members: Option<u64>, active_proposals: Option<u64>) {
        let caller = env::predecessor_account_id();
        require!(caller == self.owner, "Only owner can update platform stats");
        
        if let Some(count) = nft_count {
            self.platform_stats.total_voice_nfts = count;
        }
        if let Some(members) = dao_members {
            self.platform_stats.dao_members = members;
        }
        if let Some(proposals) = active_proposals {
            self.platform_stats.active_proposals = proposals;
        }
        
        self.platform_stats.last_updated = env::block_timestamp();
    }

    pub fn get_platform_stats(&self) -> PlatformStats {
        self.platform_stats.clone()
    }

    // ===================
    // CROSS-CONTRACT CALLS
    // ===================

    pub fn cross_contract_voice_balance(&self, account_id: AccountId) -> Promise {
        require!(self.contract_addresses.voice_token.is_some(), "Voice token contract not set");
        
        let voice_token_contract = self.contract_addresses.voice_token.clone().unwrap();
        
        Promise::new(voice_token_contract)
            .function_call(
                "ft_balance_of".to_string(),
                serde_json::to_vec(&serde_json::json!({
                    "account_id": account_id
                })).unwrap(),
                NearToken::from_yoctonear(0),
                Gas::from_tgas(5),
            )
    }

    pub fn cross_contract_nft_count(&self, owner: AccountId) -> Promise {
        require!(self.contract_addresses.voice_nft.is_some(), "Voice NFT contract not set");
        
        let voice_nft_contract = self.contract_addresses.voice_nft.clone().unwrap();
        
        Promise::new(voice_nft_contract)
            .function_call(
                "nft_supply_for_owner".to_string(),
                serde_json::to_vec(&serde_json::json!({
                    "account_id": owner
                })).unwrap(),
                NearToken::from_yoctonear(0),
                Gas::from_tgas(5),
            )
    }

    // ===================
    // ADMIN METHODS
    // ===================

    pub fn pause_platform(&mut self) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can pause platform");
        self.paused = true;
        log!("Platform paused");
    }

    pub fn unpause_platform(&mut self) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can unpause platform");
        self.paused = false;
        log!("Platform unpaused");
    }

    pub fn transfer_ownership(&mut self, new_owner: AccountId) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can transfer ownership");
        self.owner = new_owner.clone();
        log!("Ownership transferred to {}", new_owner);
    }

    pub fn update_version(&mut self, new_version: String) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can update version");
        self.version = new_version.clone();
        log!("Version updated to {}", new_version);
    }

    // ===================
    // VIEW METHODS
    // ===================

    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub fn get_version(&self) -> String {
        self.version.clone()
    }

    pub fn get_deployed_at(&self) -> u64 {
        self.deployed_at
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn get_user_count(&self) -> u64 {
        self.platform_stats.total_users
    }

    pub fn get_total_plays(&self) -> u64 {
        self.platform_stats.total_plays
    }

    pub fn get_total_volume(&self) -> u128 {
        self.platform_stats.total_volume
    }

    // ===================
    // HELPER METHODS
    // ===================

    pub fn is_contract_deployed(&self, contract_type: String) -> bool {
        match contract_type.as_str() {
            "voice_token" => self.contract_addresses.voice_token.is_some(),
            "voice_nft" => self.contract_addresses.voice_nft.is_some(),
            "marketplace" => self.contract_addresses.marketplace.is_some(),
            "dao" => self.contract_addresses.dao.is_some(),
            _ => false,
        }
    }

    pub fn get_contract_summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        
        summary.insert("owner".to_string(), self.owner.to_string());
        summary.insert("version".to_string(), self.version.clone());
        summary.insert("paused".to_string(), self.paused.to_string());
        summary.insert("total_users".to_string(), self.platform_stats.total_users.to_string());
        summary.insert("total_nfts".to_string(), self.platform_stats.total_voice_nfts.to_string());
        summary.insert("total_plays".to_string(), self.platform_stats.total_plays.to_string());
        summary.insert("total_volume".to_string(), self.platform_stats.total_volume.to_string());
        
        summary
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn test_contract_initialization() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);
        
        let contract = VoiceOrchestrator::new(accounts(1));
        assert_eq!(contract.get_owner(), accounts(1));
        assert_eq!(contract.get_version(), "1.0.0");
        assert!(!contract.is_paused());
        assert_eq!(contract.get_user_count(), 0);
    }

    #[test]
    fn test_user_profile_creation() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);
        
        let mut contract = VoiceOrchestrator::new(accounts(1));
        contract.create_user_profile(
            Some("Test User".to_string()),
            Some("Test bio".to_string()),
            Some("QmTest".to_string()),
        );
        
        assert_eq!(contract.get_user_count(), 1);
        let profile = contract.get_user_profile(accounts(1)).unwrap();
        assert_eq!(profile.display_name, Some("Test User".to_string()));
        assert_eq!(profile.bio, Some("Test bio".to_string()));
        assert!(!profile.verified);
    }

    #[test]
    fn test_contract_address_management() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);
        
        let mut contract = VoiceOrchestrator::new(accounts(1));
        contract.set_contract_address("voice_token".to_string(), accounts(2));
        
        assert!(contract.is_contract_deployed("voice_token".to_string()));
        let addresses = contract.get_contract_addresses();
        assert_eq!(addresses.voice_token, Some(accounts(2)));
    }

    #[test]
    fn test_analytics_recording() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);
        
        let mut contract = VoiceOrchestrator::new(accounts(1));
        contract.record_voice_play(
            "token-1".to_string(),
            accounts(2),
            Some("US".to_string()),
            120,
        );
        
        let analytics = contract.get_voice_analytics("token-1".to_string()).unwrap();
        assert_eq!(analytics.total_plays, 1);
        assert_eq!(analytics.geographic_data.get("US"), Some(&1));
        assert_eq!(contract.get_total_plays(), 1);
    }
}
