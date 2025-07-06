// Voice NFT Contract - Specialized NFT implementation for voice content
use near_sdk::{
    env, log, near, require, AccountId, BorshStorageKey, PanicOnDefault, Promise,
    collections::{LookupMap, UnorderedMap, UnorderedSet},
    json_types::{Base64VecU8, U128, U64},
    serde::{Deserialize, Serialize},
    NearToken,
};
use borsh::{BorshDeserialize, BorshSerialize};
// use schemars::JsonSchema;
use std::collections::HashMap;

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
enum StorageKey {
    TokensPerOwner,
    TokenById,
    TokenMetadata,
    Enumeration,
    Approval,
    Royalties,
    Analytics,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VoiceNFTMetadata {
    pub title: String,
    pub description: String,
    pub media: String, // IPFS hash for the voice file
    pub media_hash: Option<String>,
    pub copies: Option<u64>,
    pub issued_at: Option<String>,
    pub expires_at: Option<String>,
    pub starts_at: Option<String>,
    pub updated_at: Option<String>,
    pub extra: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
    // Voice-specific metadata
    pub duration: u64,
    pub file_size: u64,
    pub audio_format: String,
    pub sample_rate: u32,
    pub bit_rate: u32,
    pub creator: AccountId,
    pub category: String,
    pub tags: Vec<String>,
    pub ai_analysis: Option<AIAnalysis>,
    pub access_requirements: AccessRequirements,
    pub created_at: u64,
    pub play_count: u64,
    pub like_count: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AIAnalysis {
    pub transcription: String,
    pub language: String,
    pub sentiment: String,
    pub emotion: String,
    pub topics: Vec<String>,
    pub keywords: Vec<String>,
    pub quality_score: u8,
    pub authenticity_score: u8,
    pub content_rating: String,
    pub analysis_version: String,
    pub processed_at: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AccessRequirements {
    pub access_type: String, // "public", "token_gated", "dao_only"
    pub min_tokens: Option<u128>,
    pub required_nfts: Option<Vec<String>>,
    pub whitelist: Option<Vec<AccountId>>,
    pub price: Option<u128>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VoiceAnalytics {
    pub total_plays: u64,
    pub unique_listeners: u64,
    pub total_earnings: u128,
    pub geographic_distribution: HashMap<String, u64>,
    pub device_types: HashMap<String, u64>,
    pub listening_duration: Vec<u64>, // Array of listening durations in seconds
    pub peak_listening_times: Vec<u64>, // Unix timestamps of peak listening
    pub user_ratings: Vec<u8>, // 1-5 star ratings
    pub comments_count: u64,
    pub shares_count: u64,
    pub bookmarks_count: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Royalty {
    pub account_id: AccountId,
    pub value: u32, // Basis points (100 = 1%)
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VoiceNFTMintEvent {
    pub owner_id: AccountId,
    pub token_ids: Vec<String>,
    pub memo: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VoiceNFTTransferEvent {
    pub old_owner_id: AccountId,
    pub new_owner_id: AccountId,
    pub token_ids: Vec<String>,
    pub authorized_id: Option<AccountId>,
    pub memo: Option<String>,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct VoiceNFT {
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<String>>,
    pub token_by_id: LookupMap<String, VoiceNFTMetadata>,
    pub token_metadata_by_id: UnorderedMap<String, VoiceNFTMetadata>,
    pub owner_by_id: LookupMap<String, AccountId>,
    pub next_approval_id_by_id: LookupMap<String, u64>,
    pub approvals_by_id: LookupMap<String, HashMap<AccountId, u64>>,
    pub royalties_by_id: LookupMap<String, HashMap<AccountId, u32>>,
    pub analytics_by_id: LookupMap<String, VoiceAnalytics>,
    pub owner_id: AccountId,
    pub extra_storage_in_bytes_per_token: u64,
    pub next_token_id: u64,
    pub contract_metadata: HashMap<String, String>,
    pub approved_minters: UnorderedSet<AccountId>,
    pub mint_price: u128,
    pub is_minting_enabled: bool,
}

#[near]
impl VoiceNFT {
    #[init]
    pub fn new(owner_id: AccountId, mint_price: U128) -> Self {
        let mut contract = Self {
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner),
            token_by_id: LookupMap::new(StorageKey::TokenById),
            token_metadata_by_id: UnorderedMap::new(StorageKey::TokenMetadata),
            owner_by_id: LookupMap::new(StorageKey::TokenById),
            next_approval_id_by_id: LookupMap::new(StorageKey::Approval),
            approvals_by_id: LookupMap::new(StorageKey::Approval),
            royalties_by_id: LookupMap::new(StorageKey::Royalties),
            analytics_by_id: LookupMap::new(StorageKey::Analytics),
            owner_id: owner_id.clone(),
            extra_storage_in_bytes_per_token: 0,
            next_token_id: 1,
            contract_metadata: HashMap::new(),
            approved_minters: UnorderedSet::new(b"m"),
            mint_price: mint_price.into(),
            is_minting_enabled: true,
        };
        
        // Add owner as approved minter
        contract.approved_minters.insert(&owner_id);
        
        contract
    }
    
    // Core NFT methods
    #[payable]
    pub fn nft_mint(&mut self, receiver_id: AccountId, token_metadata: VoiceNFTMetadata) -> String {
        let initial_storage_usage = env::storage_usage();
        let attached_deposit = env::attached_deposit();
        let predecessor = env::predecessor_account_id();
        
        require!(self.is_minting_enabled, "Minting is disabled");
        require!(
            self.approved_minters.contains(&predecessor) || 
            attached_deposit >= NearToken::from_yoctonear(self.mint_price),
            "Insufficient deposit or not authorized minter"
        );
        
        let token_id = self.next_token_id.to_string();
        self.next_token_id += 1;
        
        // Create token metadata with additional fields
        let mut metadata = token_metadata;
        metadata.created_at = env::block_timestamp();
        metadata.play_count = 0;
        metadata.like_count = 0;
        
        // Store token
        self.token_by_id.insert(&token_id, &metadata);
        self.token_metadata_by_id.insert(&token_id, &metadata);
        self.owner_by_id.insert(&token_id, &receiver_id);
        self.next_approval_id_by_id.insert(&token_id, &1u64);
        
        // Update tokens per owner
        let mut tokens_set = self.tokens_per_owner.get(&receiver_id).unwrap_or_else(|| {
            UnorderedSet::new(format!("tokens-{}", receiver_id).as_bytes())
        });
        tokens_set.insert(&token_id);
        self.tokens_per_owner.insert(&receiver_id, &tokens_set);
        
        // Initialize analytics
        let analytics = VoiceAnalytics {
            total_plays: 0,
            unique_listeners: 0,
            total_earnings: 0,
            geographic_distribution: HashMap::new(),
            device_types: HashMap::new(),
            listening_duration: Vec::new(),
            peak_listening_times: Vec::new(),
            user_ratings: Vec::new(),
            comments_count: 0,
            shares_count: 0,
            bookmarks_count: 0,
        };
        self.analytics_by_id.insert(&token_id, &analytics);
        
        // Set default royalty (10% to creator)
        let mut royalties = HashMap::new();
        royalties.insert(metadata.creator.clone(), 1000u32);
        self.royalties_by_id.insert(&token_id, &royalties);
        
        // Calculate storage cost
        let storage_used = env::storage_usage() - initial_storage_usage;
        let required_cost = env::storage_byte_cost().as_yoctonear() * u128::from(storage_used);
        let required_cost_token = NearToken::from_yoctonear(required_cost);
        
        if attached_deposit < required_cost_token {
            env::panic_str("Insufficient deposit for storage");
        }
        
        // Refund excess deposit
        if attached_deposit > required_cost_token {
            let refund = attached_deposit.as_yoctonear() - required_cost;
            Promise::new(predecessor).transfer(NearToken::from_yoctonear(refund));
        }
        
        // Emit event
        env::log_str(&format!("EVENT_JSON:{}", serde_json::to_string(&VoiceNFTMintEvent {
            owner_id: receiver_id.clone(),
            token_ids: vec![token_id.clone()],
            memo: None,
        }).unwrap()));
        
        log!("Voice NFT {} minted for {}", token_id, receiver_id);
        token_id
    }
    
    #[payable]
    pub fn nft_transfer(&mut self, receiver_id: AccountId, token_id: String, memo: Option<String>) {
        let sender_id = env::predecessor_account_id();
        self.internal_transfer(&sender_id, &receiver_id, &token_id, None, memo);
    }
    
    #[payable]
    pub fn nft_transfer_call(&mut self, receiver_id: AccountId, token_id: String, memo: Option<String>, msg: String) -> bool {
        let sender_id = env::predecessor_account_id();
        self.internal_transfer(&sender_id, &receiver_id, &token_id, None, memo);
        // TODO: Implement cross-contract call
        true
    }
    
    pub fn nft_token(&self, token_id: String) -> Option<VoiceNFTMetadata> {
        self.token_by_id.get(&token_id)
    }
    
    pub fn nft_total_supply(&self) -> U128 {
        U128(self.token_metadata_by_id.len() as u128)
    }
    
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<VoiceNFTMetadata> {
        let start = from_index.map(|v| v.0).unwrap_or(0) as usize;
        let limit = limit.unwrap_or(50) as usize;
        
        self.token_metadata_by_id
            .iter()
            .skip(start)
            .take(limit)
            .map(|(_, metadata)| metadata)
            .collect()
    }
    
    pub fn nft_tokens_for_owner(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<VoiceNFTMetadata> {
        let tokens = self.tokens_per_owner.get(&account_id);
        if let Some(tokens) = tokens {
            let start = from_index.map(|v| v.0).unwrap_or(0) as usize;
            let limit = limit.unwrap_or(50) as usize;
            
            tokens
                .iter()
                .skip(start)
                .take(limit)
                .map(|token_id| self.token_by_id.get(&token_id).unwrap())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        let tokens = self.tokens_per_owner.get(&account_id);
        if let Some(tokens) = tokens {
            U128(tokens.len() as u128)
        } else {
            U128(0)
        }
    }
    
    // Voice-specific methods
    pub fn record_play(&mut self, token_id: String, listener: AccountId, duration: u64, location: Option<String>) {
        let caller = env::predecessor_account_id();
        require!(caller == self.owner_id, "Only contract owner can record plays");
        
        let mut analytics = self.analytics_by_id.get(&token_id).expect("Token not found");
        
        analytics.total_plays += 1;
        analytics.listening_duration.push(duration);
        analytics.peak_listening_times.push(env::block_timestamp());
        
        if let Some(location) = location {
            let count = analytics.geographic_distribution.get(&location).unwrap_or(&0) + 1;
            analytics.geographic_distribution.insert(location, count);
        }
        
        self.analytics_by_id.insert(&token_id, &analytics);
        
        // Update play count in metadata
        let mut metadata = self.token_by_id.get(&token_id).unwrap();
        metadata.play_count += 1;
        self.token_by_id.insert(&token_id, &metadata);
        self.token_metadata_by_id.insert(&token_id, &metadata);
        
        log!("Recorded play for token {} by {}", token_id, listener);
    }
    
    pub fn like_nft(&mut self, token_id: String) {
        let _liker = env::predecessor_account_id();
        
        let mut metadata = self.token_by_id.get(&token_id).expect("Token not found");
        metadata.like_count += 1;
        
        self.token_by_id.insert(&token_id, &metadata);
        self.token_metadata_by_id.insert(&token_id, &metadata);
        
        log!("NFT {} liked, total likes: {}", token_id, metadata.like_count);
    }
    
    pub fn update_ai_analysis(&mut self, token_id: String, ai_analysis: AIAnalysis) {
        let caller = env::predecessor_account_id();
        require!(caller == self.owner_id, "Only contract owner can update AI analysis");
        
        let mut metadata = self.token_by_id.get(&token_id).expect("Token not found");
        metadata.ai_analysis = Some(ai_analysis);
        
        self.token_by_id.insert(&token_id, &metadata);
        self.token_metadata_by_id.insert(&token_id, &metadata);
        
        log!("Updated AI analysis for token {}", token_id);
    }
    
    pub fn get_analytics(&self, token_id: String) -> Option<VoiceAnalytics> {
        self.analytics_by_id.get(&token_id)
    }
    
    pub fn get_royalties(&self, token_id: String) -> Option<HashMap<AccountId, u32>> {
        self.royalties_by_id.get(&token_id)
    }
    
    pub fn set_royalties(&mut self, token_id: String, royalties: HashMap<AccountId, u32>) {
        let caller = env::predecessor_account_id();
        let owner = self.owner_by_id.get(&token_id).expect("Token not found");
        
        require!(caller == owner || caller == self.owner_id, "Not authorized");
        
        self.royalties_by_id.insert(&token_id, &royalties);
        log!("Updated royalties for token {}", token_id);
    }
    
    // Admin methods
    pub fn set_mint_price(&mut self, price: U128) {
        require!(env::predecessor_account_id() == self.owner_id, "Only owner");
        self.mint_price = price.into();
    }
    
    pub fn toggle_minting(&mut self) {
        require!(env::predecessor_account_id() == self.owner_id, "Only owner");
        self.is_minting_enabled = !self.is_minting_enabled;
    }
    
    pub fn add_approved_minter(&mut self, minter: AccountId) {
        require!(env::predecessor_account_id() == self.owner_id, "Only owner");
        self.approved_minters.insert(&minter);
    }
    
    pub fn remove_approved_minter(&mut self, minter: AccountId) {
        require!(env::predecessor_account_id() == self.owner_id, "Only owner");
        self.approved_minters.remove(&minter);
    }
    
    // Internal methods
    fn internal_transfer(&mut self, sender_id: &AccountId, receiver_id: &AccountId, token_id: &String, approval_id: Option<u64>, memo: Option<String>) {
        let owner_id = self.owner_by_id.get(token_id).expect("Token not found");
        
        require!(sender_id == &owner_id, "Sender must be token owner");
        require!(sender_id != receiver_id, "Cannot transfer to self");
        
        // Update owner
        self.owner_by_id.insert(token_id, receiver_id);
        
        // Update tokens per owner
        let mut sender_tokens = self.tokens_per_owner.get(sender_id).unwrap();
        sender_tokens.remove(token_id);
        self.tokens_per_owner.insert(sender_id, &sender_tokens);
        
        let mut receiver_tokens = self.tokens_per_owner.get(receiver_id).unwrap_or_else(|| {
            UnorderedSet::new(format!("tokens-{}", receiver_id).as_bytes())
        });
        receiver_tokens.insert(token_id);
        self.tokens_per_owner.insert(receiver_id, &receiver_tokens);
        
        // Clear approvals
        self.approvals_by_id.remove(token_id);
        
        // Emit event
        env::log_str(&format!("EVENT_JSON:{}", serde_json::to_string(&VoiceNFTTransferEvent {
            old_owner_id: sender_id.clone(),
            new_owner_id: receiver_id.clone(),
            token_ids: vec![token_id.clone()],
            authorized_id: None,
            memo: memo.clone(),
        }).unwrap()));
        
        log!("Voice NFT {} transferred from {} to {}", token_id, sender_id, receiver_id);
    }
    
    // View methods
    pub fn get_contract_metadata(&self) -> HashMap<String, String> {
        self.contract_metadata.clone()
    }
    
    pub fn get_approved_minters(&self) -> Vec<AccountId> {
        self.approved_minters.iter().collect()
    }
    
    pub fn get_mint_price(&self) -> U128 {
        U128(self.mint_price)
    }
    
    pub fn is_minting_enabled(&self) -> bool {
        self.is_minting_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn test_nft_mint() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .attached_deposit(NearToken::from_yoctonear(1_000_000_000_000_000_000_000_000))
            .build();
        testing_env!(context);
        
        let mut contract = VoiceNFT::new(accounts(1), U128(1_000_000_000_000_000_000_000_000));
        
        let metadata = VoiceNFTMetadata {
            title: "Test Voice".to_string(),
            description: "A test voice NFT".to_string(),
            media: "QmTest".to_string(),
            media_hash: None,
            copies: Some(1),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
            duration: 120,
            file_size: 1024,
            audio_format: "mp3".to_string(),
            sample_rate: 44100,
            bit_rate: 128,
            creator: accounts(1),
            category: "music".to_string(),
            tags: vec!["test".to_string()],
            ai_analysis: None,
            access_requirements: AccessRequirements {
                access_type: "public".to_string(),
                min_tokens: None,
                required_nfts: None,
                whitelist: None,
                price: None,
            },
            created_at: 0,
            play_count: 0,
            like_count: 0,
        };
        
        let token_id = contract.nft_mint(accounts(2), metadata);
        
        assert_eq!(token_id, "1");
        assert_eq!(contract.nft_supply_for_owner(accounts(2)), U128(1));
        
        let token = contract.nft_token(token_id).unwrap();
        assert_eq!(token.title, "Test Voice");
    }
}
