//! Voice NFT Contract - Specialized NFT implementation for voice content
// (migrated from monolithic contracts/src/voice_nft.rs)

use near_sdk::{
    env, log, AccountId, PanicOnDefault,
    collections::{LookupMap, UnorderedMap, UnorderedSet},
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

// JSON-compatible types for view methods
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct VoiceNFTMetadataView {
    pub title: String,
    pub description: Option<String>,
    pub media: Option<String>,
    pub media_hash: Option<String>,
    pub copies: Option<u64>,
    pub issued_at: Option<String>,
    pub expires_at: Option<String>,
    pub starts_at: Option<String>,
    pub updated_at: Option<String>,
    pub extra: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
    pub duration: Option<u64>,
    pub voice_type: Option<String>,
    pub language: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct VoiceNFTView {
    pub token_id: String,
    pub owner_id: String,
    pub metadata: VoiceNFTMetadataView,
    pub approved_account_ids: HashMap<String, u64>,
    pub royalty: Option<HashMap<String, u32>>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct ContractMetadataView {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub base_uri: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
}

// Internal contract types (no JsonSchema needed)
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub struct VoiceNFTMetadata {
    pub title: String,
    pub description: Option<String>,
    pub media: Option<String>, // URL to the voice file
    pub media_hash: Option<String>, // Hash of the media file
    pub copies: Option<u64>,
    pub issued_at: Option<String>,
    pub expires_at: Option<String>,
    pub starts_at: Option<String>,
    pub updated_at: Option<String>,
    pub extra: Option<String>, // JSON string for additional metadata
    pub reference: Option<String>, // URL to off-chain JSON metadata
    pub reference_hash: Option<String>, // Hash of the reference
    pub duration: Option<u64>, // Duration in seconds
    pub voice_type: Option<String>, // Type of voice content
    pub language: Option<String>, // Language of the voice
    pub tags: Option<Vec<String>>, // Tags for categorization
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub struct VoiceNFT {
    pub token_id: String,
    pub owner_id: AccountId,
    pub metadata: VoiceNFTMetadata,
    pub approved_account_ids: HashMap<AccountId, u64>,
    pub royalty: Option<HashMap<AccountId, u32>>, // Account -> royalty percentage (basis points)
    pub created_at: U64,
    pub updated_at: U64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub struct ContractMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub base_uri: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<String>,
}

/// The Voice NFT contract implementing NEP-171 Non-Fungible Token Standard
#[near_sdk::near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct VoiceNFTContract {
    /// Contract owner
    pub owner_id: AccountId,
    /// Total number of tokens
    pub total_supply: U128,
    /// Mapping from token ID to token
    pub tokens: UnorderedMap<String, VoiceNFT>,
    /// Mapping from owner to list of token IDs
    pub tokens_by_owner: LookupMap<AccountId, UnorderedSet<String>>,
    /// Mapping from token ID to approved account
    pub token_approvals: LookupMap<String, AccountId>,
    /// Mapping from owner to operator approvals
    pub operator_approvals: LookupMap<AccountId, UnorderedSet<AccountId>>,
    /// Contract metadata
    pub metadata: ContractMetadata,
    /// Next token ID counter
    pub next_token_id: U128,
}

#[near_sdk::near_bindgen]
impl VoiceNFTContract {
    /// Initialize the contract
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        let metadata = ContractMetadata {
            spec: "nft-1.0.0".to_string(),
            name: "Voice NFT".to_string(),
            symbol: "VNFT".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        };

        let this = Self {
            owner_id: owner_id.clone(),
            total_supply: U128(0),
            tokens: UnorderedMap::new(b"t".to_vec()),
            tokens_by_owner: LookupMap::new(b"tbo".to_vec()),
            token_approvals: LookupMap::new(b"ta".to_vec()),
            operator_approvals: LookupMap::new(b"oa".to_vec()),
            metadata,
            next_token_id: U128(1),
        };

        log!("Voice NFT contract deployed. Owner: {}", owner_id);

        this
    }

    /// Mint a new voice NFT
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: String,
        receiver_id: AccountId,
        metadata: VoiceNFTMetadataView,
        royalty: Option<HashMap<String, u32>>,
    ) -> VoiceNFTView {
        // Only owner can mint for now
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can mint");
        
        // Ensure token doesn't already exist
        assert!(!self.tokens.get(&token_id).is_some(), "Token already exists");

        // Convert view metadata to internal metadata
        let internal_metadata = VoiceNFTMetadata {
            title: metadata.title,
            description: metadata.description,
            media: metadata.media,
            media_hash: metadata.media_hash,
            copies: metadata.copies,
            issued_at: metadata.issued_at,
            expires_at: metadata.expires_at,
            starts_at: metadata.starts_at,
            updated_at: metadata.updated_at,
            extra: metadata.extra,
            reference: metadata.reference,
            reference_hash: metadata.reference_hash,
            duration: metadata.duration,
            voice_type: metadata.voice_type,
            language: metadata.language,
            tags: metadata.tags,
        };

        // Convert royalty from String keys to AccountId keys
        let internal_royalty = royalty.map(|r| 
            r.into_iter()
                .map(|(k, v)| (k.parse().expect("Invalid account ID"), v))
                .collect()
        );

        let token = VoiceNFT {
            token_id: token_id.clone(),
            owner_id: receiver_id.clone(),
            metadata: internal_metadata,
            approved_account_ids: HashMap::new(),
            royalty: internal_royalty,
            created_at: U64(env::block_timestamp()),
            updated_at: U64(env::block_timestamp()),
        };

        // Insert token
        self.tokens.insert(&token_id, &token);

        // Add to owner's tokens
        let mut owner_tokens = self.tokens_by_owner.get(&receiver_id).unwrap_or_else(|| UnorderedSet::new(receiver_id.as_bytes().to_vec()));
        owner_tokens.insert(&token_id);
        self.tokens_by_owner.insert(&receiver_id, &owner_tokens);

        // Increment counters
        self.total_supply.0 += 1;
        self.next_token_id.0 += 1;

        log!("Minted voice NFT {} to {}", token_id, receiver_id);

        self.token_to_view(token)
    }

    /// Transfer a token
    #[payable]
    pub fn nft_transfer(&mut self, receiver_id: AccountId, token_id: String, memo: Option<String>) {
        let sender_id = env::predecessor_account_id();
        self.internal_transfer(&sender_id, &receiver_id, &token_id, memo);
    }

    /// Transfer a token from one account to another (requires approval)
    #[payable]
    pub fn nft_transfer_from(&mut self, sender_id: AccountId, receiver_id: AccountId, token_id: String, memo: Option<String>) {
        let predecessor_id = env::predecessor_account_id();
        self.internal_transfer_from(&sender_id, &receiver_id, &token_id, &predecessor_id, memo);
    }

    /// Approve an account to transfer a specific token
    #[payable]
    pub fn nft_approve(&mut self, token_id: String, account_id: AccountId) {
        let owner_id = env::predecessor_account_id();
        let token = self.tokens.get(&token_id).expect("Token not found");
        
        assert_eq!(token.owner_id, owner_id, "Only owner can approve");
        
        self.token_approvals.insert(&token_id, &account_id);
        
        log!("Approved {} to transfer token {}", account_id, token_id);
    }

    /// Revoke approval for a specific token
    #[payable]
    pub fn nft_revoke(&mut self, token_id: String) {
        let owner_id = env::predecessor_account_id();
        let token = self.tokens.get(&token_id).expect("Token not found");
        
        assert_eq!(token.owner_id, owner_id, "Only owner can revoke");
        
        self.token_approvals.remove(&token_id);
        
        log!("Revoked approval for token {}", token_id);
    }

    /// Get token information
    pub fn nft_token(&self, token_id: String) -> Option<VoiceNFTView> {
        self.tokens.get(&token_id).map(|token| self.token_to_view(token))
    }

    /// Get tokens owned by an account
    pub fn nft_tokens_for_owner(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<VoiceNFTView> {
        let tokens_set = self.tokens_by_owner.get(&account_id);
        
        if let Some(tokens_set) = tokens_set {
            let start = from_index.map(|i| i.0 as usize).unwrap_or(0);
            let limit = limit.unwrap_or(50) as usize;
            
            tokens_set
                .to_vec()
                .into_iter()
                .skip(start)
                .take(limit)
                .filter_map(|token_id| self.tokens.get(&token_id))
                .map(|token| self.token_to_view(token))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get total supply
    pub fn nft_total_supply(&self) -> String {
        self.total_supply.0.to_string()
    }

    /// Get contract metadata
    pub fn nft_metadata(&self) -> ContractMetadataView {
        ContractMetadataView {
            spec: self.metadata.spec.clone(),
            name: self.metadata.name.clone(),
            symbol: self.metadata.symbol.clone(),
            icon: self.metadata.icon.clone(),
            base_uri: self.metadata.base_uri.clone(),
            reference: self.metadata.reference.clone(),
            reference_hash: self.metadata.reference_hash.clone(),
        }
    }

    /// Get tokens by page
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<VoiceNFTView> {
        let start = from_index.map(|i| i.0 as usize).unwrap_or(0);
        let limit = limit.unwrap_or(50) as usize;
        
        self.tokens
            .values()
            .skip(start)
            .take(limit)
            .map(|token| self.token_to_view(token))
            .collect()
    }

    /// Get supply for owner
    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> String {
        self.tokens_by_owner
            .get(&account_id)
            .map(|tokens| tokens.len().to_string())
            .unwrap_or("0".to_string())
    }

    // Internal methods
    fn internal_transfer(&mut self, sender_id: &AccountId, receiver_id: &AccountId, token_id: &String, _memo: Option<String>) {
        let mut token = self.tokens.get(token_id).expect("Token not found");
        
        assert_eq!(&token.owner_id, sender_id, "Only owner can transfer");
        assert_ne!(sender_id, receiver_id, "Cannot transfer to self");
        
        // Remove from sender's tokens
        let mut sender_tokens = self.tokens_by_owner.get(sender_id).unwrap();
        sender_tokens.remove(token_id);
        if sender_tokens.is_empty() {
            self.tokens_by_owner.remove(sender_id);
        } else {
            self.tokens_by_owner.insert(sender_id, &sender_tokens);
        }
        
        // Add to receiver's tokens
        let mut receiver_tokens = self.tokens_by_owner.get(receiver_id).unwrap_or_else(|| UnorderedSet::new(receiver_id.as_bytes().to_vec()));
        receiver_tokens.insert(token_id);
        self.tokens_by_owner.insert(receiver_id, &receiver_tokens);
        
        // Update token owner
        token.owner_id = receiver_id.clone();
        token.updated_at = U64(env::block_timestamp());
        token.approved_account_ids.clear();
        self.tokens.insert(token_id, &token);
        
        // Clear approvals
        self.token_approvals.remove(token_id);
        
        log!("Transferred token {} from {} to {}", token_id, sender_id, receiver_id);
    }

    fn internal_transfer_from(&mut self, sender_id: &AccountId, receiver_id: &AccountId, token_id: &String, predecessor_id: &AccountId, _memo: Option<String>) {
        let token = self.tokens.get(token_id).expect("Token not found");
        
        assert_eq!(&token.owner_id, sender_id, "Sender is not the owner");
        
        // Check if predecessor is approved
        let approved = self.token_approvals.get(token_id).map(|approved_id| &approved_id == predecessor_id).unwrap_or(false);
        let operator_approved = self.operator_approvals.get(sender_id).map(|operators| operators.contains(predecessor_id)).unwrap_or(false);
        
        assert!(approved || operator_approved || predecessor_id == sender_id, "Not approved to transfer");
        
        self.internal_transfer(sender_id, receiver_id, token_id, _memo);
    }

    // Helper methods to convert internal types to view types
    fn token_to_view(&self, token: VoiceNFT) -> VoiceNFTView {
        VoiceNFTView {
            token_id: token.token_id,
            owner_id: token.owner_id.to_string(),
            metadata: VoiceNFTMetadataView {
                title: token.metadata.title,
                description: token.metadata.description,
                media: token.metadata.media,
                media_hash: token.metadata.media_hash,
                copies: token.metadata.copies,
                issued_at: token.metadata.issued_at,
                expires_at: token.metadata.expires_at,
                starts_at: token.metadata.starts_at,
                updated_at: token.metadata.updated_at,
                extra: token.metadata.extra,
                reference: token.metadata.reference,
                reference_hash: token.metadata.reference_hash,
                duration: token.metadata.duration,
                voice_type: token.metadata.voice_type,
                language: token.metadata.language,
                tags: token.metadata.tags,
            },
            approved_account_ids: token.approved_account_ids.into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
            royalty: token.royalty.map(|royalty| 
                royalty.into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect()
            ),
            created_at: token.created_at.0.to_string(),
            updated_at: token.updated_at.0.to_string(),
        }
    }
}
