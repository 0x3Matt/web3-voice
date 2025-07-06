//! Voice Token Contract - Fungible token for the platform
// (migrated from monolithic contracts/src/voice_token.rs)

use near_sdk::{
    env, log, AccountId, PanicOnDefault,
    collections::{LookupMap, UnorderedSet},
    json_types::U128,
    serde::{Deserialize, Serialize},
};
use borsh::{BorshDeserialize, BorshSerialize};

const TOTAL_SUPPLY: u128 = 1_000_000_000_000_000_000_000_000_000_000; // 1 billion tokens with 18 decimals
const DECIMALS: u8 = 18;

// JSON-compatible types for view methods
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct FungibleTokenMetadataView {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: String,
    pub icon: Option<String>,
}

// Internal contract types
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub struct FungibleTokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: U128,
    pub icon: Option<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub struct TransferEvent {
    pub from: AccountId,
    pub to: AccountId,
    pub amount: U128,
    pub memo: Option<String>,
}

/// The Voice Token contract implementing NEP-141 Fungible Token Standard
#[near_sdk::near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct VoiceToken {
    /// Total supply of tokens
    pub total_supply: U128,
    /// Account ID of the contract owner
    pub owner_id: AccountId,
    /// Account balances
    pub balances: LookupMap<AccountId, U128>,
    /// Allowances for spending on behalf of other accounts
    pub allowances: LookupMap<AccountId, LookupMap<AccountId, U128>>,
    /// Set of accounts that have been registered
    pub accounts: UnorderedSet<AccountId>,
    /// Metadata for the token
    pub metadata: FungibleTokenMetadata,
}

#[near_sdk::near_bindgen]
impl VoiceToken {
    /// Initialize the contract with the given owner
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        
        let metadata = FungibleTokenMetadata {
            name: "Voice Token".to_string(),
            symbol: "VOICE".to_string(),
            decimals: DECIMALS,
            total_supply: U128(TOTAL_SUPPLY),
            icon: None,
        };

        let mut this = Self {
            total_supply: U128(TOTAL_SUPPLY),
            owner_id: owner_id.clone(),
            balances: LookupMap::new(b"b".to_vec()),
            allowances: LookupMap::new(b"a".to_vec()),
            accounts: UnorderedSet::new(b"acc".to_vec()),
            metadata,
        };

        // Give the owner the total supply
        this.balances.insert(&owner_id, &U128(TOTAL_SUPPLY));
        this.accounts.insert(&owner_id);

        log!("Voice Token contract deployed. Owner: {}, Total Supply: {}", owner_id, TOTAL_SUPPLY);

        this
    }

    /// Transfer tokens to another account
    #[payable]
    pub fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        let sender_id = env::predecessor_account_id();
        self.internal_transfer(&sender_id, &receiver_id, amount.0, memo);
    }

    /// Transfer tokens from one account to another (requires allowance)
    #[payable]
    pub fn ft_transfer_from(&mut self, sender_id: AccountId, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        let spender_id = env::predecessor_account_id();
        self.internal_transfer_from(&sender_id, &receiver_id, &spender_id, amount.0, memo);
    }

    /// Get the balance of an account
    pub fn ft_balance_of(&self, account_id: AccountId) -> String {
        self.balances.get(&account_id).unwrap_or(U128(0)).0.to_string()
    }

    /// Get the total supply
    pub fn ft_total_supply(&self) -> String {
        self.total_supply.0.to_string()
    }

    /// Get the token metadata
    pub fn ft_metadata(&self) -> FungibleTokenMetadataView {
        FungibleTokenMetadataView {
            name: self.metadata.name.clone(),
            symbol: self.metadata.symbol.clone(),
            decimals: self.metadata.decimals,
            total_supply: self.metadata.total_supply.0.to_string(),
            icon: self.metadata.icon.clone(),
        }
    }

    /// Register an account (required before transfers)
    #[payable]
    pub fn storage_deposit(&mut self, account_id: Option<AccountId>) -> bool {
        let account = account_id.unwrap_or_else(|| env::predecessor_account_id());
        
        if !self.accounts.contains(&account) {
            self.accounts.insert(&account);
            self.balances.insert(&account, &U128(0));
            true
        } else {
            false
        }
    }

    /// Check if an account is registered
    pub fn storage_balance_of(&self, account_id: AccountId) -> bool {
        self.accounts.contains(&account_id)
    }

    /// Approve another account to spend tokens on your behalf
    pub fn approve(&mut self, spender_id: AccountId, amount: U128) {
        let owner_id = env::predecessor_account_id();
        
        let mut owner_allowances = self.allowances.get(&owner_id).unwrap_or_else(|| LookupMap::new(owner_id.as_bytes().to_vec()));
        owner_allowances.insert(&spender_id, &amount);
        self.allowances.insert(&owner_id, &owner_allowances);

        log!("Approved {} VOICE tokens for {} to spend on behalf of {}", amount.0, spender_id, owner_id);
    }

    /// Get the allowance for a spender
    pub fn allowance(&self, owner_id: AccountId, spender_id: AccountId) -> String {
        self.allowances
            .get(&owner_id)
            .and_then(|allowances| allowances.get(&spender_id))
            .unwrap_or(U128(0)).0.to_string()
    }

    /// Mint new tokens (only owner)
    pub fn mint(&mut self, account_id: AccountId, amount: U128) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can mint tokens");
        
        let current_balance = self.balances.get(&account_id).unwrap_or(U128(0));
        let new_balance = U128(current_balance.0 + amount.0);
        
        self.balances.insert(&account_id, &new_balance);
        self.total_supply.0 += amount.0;
        
        if !self.accounts.contains(&account_id) {
            self.accounts.insert(&account_id);
        }

        log!("Minted {} VOICE tokens to {}", amount.0, account_id);
    }

    /// Burn tokens
    pub fn burn(&mut self, amount: U128) {
        let account_id = env::predecessor_account_id();
        let current_balance = self.balances.get(&account_id).unwrap_or(U128(0));
        
        assert!(current_balance.0 >= amount.0, "Insufficient balance to burn");
        
        let new_balance = U128(current_balance.0 - amount.0);
        self.balances.insert(&account_id, &new_balance);
        self.total_supply.0 -= amount.0;

        log!("Burned {} VOICE tokens from {}", amount.0, account_id);
    }

    // Internal methods
    fn internal_transfer(&mut self, sender_id: &AccountId, receiver_id: &AccountId, amount: u128, _memo: Option<String>) {
        assert_ne!(sender_id, receiver_id, "Cannot transfer to self");
        assert!(amount > 0, "Amount must be positive");
        
        let sender_balance = self.balances.get(sender_id).unwrap_or(U128(0));
        assert!(sender_balance.0 >= amount, "Insufficient balance");
        
        // Ensure receiver is registered
        if !self.accounts.contains(receiver_id) {
            self.accounts.insert(receiver_id);
            self.balances.insert(receiver_id, &U128(0));
        }
        
        let receiver_balance = self.balances.get(receiver_id).unwrap_or(U128(0));
        
        // Update balances
        self.balances.insert(sender_id, &U128(sender_balance.0 - amount));
        self.balances.insert(receiver_id, &U128(receiver_balance.0 + amount));
        
        log!("Transferred {} VOICE tokens from {} to {}", amount, sender_id, receiver_id);
    }

    fn internal_transfer_from(&mut self, sender_id: &AccountId, receiver_id: &AccountId, spender_id: &AccountId, amount: u128, memo: Option<String>) {
        // Check allowance
        let allowance = self.allowances
            .get(sender_id)
            .and_then(|allowances| allowances.get(spender_id))
            .unwrap_or(U128(0));
        
        assert!(allowance.0 >= amount, "Insufficient allowance");
        
        // Perform the transfer
        self.internal_transfer(sender_id, receiver_id, amount, memo);
        
        // Update allowance
        let mut sender_allowances = self.allowances.get(sender_id).unwrap();
        sender_allowances.insert(spender_id, &U128(allowance.0 - amount));
        self.allowances.insert(sender_id, &sender_allowances);
    }
}
