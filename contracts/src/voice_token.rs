// Voice Token Contract - Specialized FT implementation for the Web3Voice platform
use near_sdk::{
    env, log, near, require, AccountId, BorshStorageKey, PanicOnDefault, Promise,
    collections::{LookupMap, UnorderedMap},
    json_types::{U128},
    serde::{Deserialize, Serialize},
    NearToken,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
enum StorageKey {
    Accounts,
    Allowances,
    Metadata,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: U128,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct VoiceToken {
    pub accounts: LookupMap<AccountId, u128>,
    pub allowances: LookupMap<AccountId, UnorderedMap<AccountId, u128>>,
    pub total_supply: u128,
    pub metadata: TokenMetadata,
    pub owner: AccountId,
    pub minters: Vec<AccountId>,
    pub pause_transfers: bool,
}

#[near]
impl VoiceToken {
    #[init]
    pub fn new(owner: AccountId, total_supply: U128) -> Self {
        let total_supply: u128 = total_supply.into();
        let mut token = Self {
            accounts: LookupMap::new(StorageKey::Accounts),
            allowances: LookupMap::new(StorageKey::Allowances),
            total_supply,
            metadata: TokenMetadata {
                name: "Voice Token".to_string(),
                symbol: "VOICE".to_string(),
                decimals: 24,
                total_supply: U128(total_supply),
            },
            owner: owner.clone(),
            minters: vec![owner.clone()],
            pause_transfers: false,
        };
        
        // Assign total supply to owner
        token.accounts.insert(&owner, &total_supply);
        token
    }
    
    // Standard FT methods
    pub fn ft_total_supply(&self) -> U128 {
        U128(self.total_supply)
    }
    
    pub fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        U128(self.accounts.get(&account_id).unwrap_or(0))
    }
    
    #[payable]
    pub fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        require!(!self.pause_transfers, "Transfers are paused");
        
        let sender_id = env::predecessor_account_id();
        let amount: u128 = amount.into();
        
        self.internal_transfer(&sender_id, &receiver_id, amount, memo);
    }
    
    #[payable]
    pub fn ft_transfer_from(&mut self, sender_id: AccountId, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        require!(!self.pause_transfers, "Transfers are paused");
        
        let spender_id = env::predecessor_account_id();
        let amount: u128 = amount.into();
        
        // Check allowance
        let allowance = self.get_allowance(&sender_id, &spender_id);
        require!(allowance >= amount, "Insufficient allowance");
        
        // Update allowance
        self.set_allowance(&sender_id, &spender_id, allowance - amount);
        
        self.internal_transfer(&sender_id, &receiver_id, amount, memo);
    }
    
    pub fn ft_approve(&mut self, spender_id: AccountId, amount: U128) {
        let owner_id = env::predecessor_account_id();
        let amount: u128 = amount.into();
        
        self.set_allowance(&owner_id, &spender_id, amount);
        log!("Approved {} VOICE tokens for {} to spend on behalf of {}", amount, spender_id, owner_id);
    }
    
    pub fn ft_allowance(&self, owner_id: AccountId, spender_id: AccountId) -> U128 {
        U128(self.get_allowance(&owner_id, &spender_id))
    }
    
    // Voice-specific methods
    pub fn mint(&mut self, account_id: AccountId, amount: U128) {
        let caller = env::predecessor_account_id();
        require!(self.minters.contains(&caller), "Not authorized to mint");
        
        let amount: u128 = amount.into();
        let balance = self.accounts.get(&account_id).unwrap_or(0);
        
        self.accounts.insert(&account_id, &(balance + amount));
        self.total_supply += amount;
        self.metadata.total_supply = U128(self.total_supply);
        
        log!("Minted {} VOICE tokens for {}", amount, account_id);
    }
    
    pub fn burn(&mut self, amount: U128) {
        let account_id = env::predecessor_account_id();
        let amount: u128 = amount.into();
        let balance = self.accounts.get(&account_id).unwrap_or(0);
        
        require!(balance >= amount, "Insufficient balance to burn");
        
        self.accounts.insert(&account_id, &(balance - amount));
        self.total_supply -= amount;
        self.metadata.total_supply = U128(self.total_supply);
        
        log!("Burned {} VOICE tokens from {}", amount, account_id);
    }
    
    // Admin methods
    pub fn add_minter(&mut self, minter: AccountId) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can add minters");
        if !self.minters.contains(&minter) {
            self.minters.push(minter.clone());
            log!("Added {} as minter", minter);
        }
    }
    
    pub fn remove_minter(&mut self, minter: AccountId) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can remove minters");
        self.minters.retain(|m| m != &minter);
        log!("Removed {} as minter", minter);
    }
    
    pub fn pause_transfers(&mut self) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can pause transfers");
        self.pause_transfers = true;
        log!("Transfers paused");
    }
    
    pub fn unpause_transfers(&mut self) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can unpause transfers");
        self.pause_transfers = false;
        log!("Transfers unpaused");
    }
    
    pub fn transfer_ownership(&mut self, new_owner: AccountId) {
        require!(env::predecessor_account_id() == self.owner, "Only owner can transfer ownership");
        self.owner = new_owner.clone();
        log!("Ownership transferred to {}", new_owner);
    }
    
    // Internal methods
    fn internal_transfer(&mut self, sender_id: &AccountId, receiver_id: &AccountId, amount: u128, memo: Option<String>) {
        let sender_balance = self.accounts.get(sender_id).unwrap_or(0);
        require!(sender_balance >= amount, "Insufficient balance");
        
        self.accounts.insert(sender_id, &(sender_balance - amount));
        
        let receiver_balance = self.accounts.get(receiver_id).unwrap_or(0);
        self.accounts.insert(receiver_id, &(receiver_balance + amount));
        
        if let Some(memo) = memo {
            log!("Transfer {} VOICE from {} to {} with memo: {}", amount, sender_id, receiver_id, memo);
        } else {
            log!("Transfer {} VOICE from {} to {}", amount, sender_id, receiver_id);
        }
    }
    
    fn get_allowance(&self, owner_id: &AccountId, spender_id: &AccountId) -> u128 {
        self.allowances
            .get(owner_id)
            .and_then(|allowances| allowances.get(spender_id))
            .unwrap_or(0)
    }
    
    fn set_allowance(&mut self, owner_id: &AccountId, spender_id: &AccountId, amount: u128) {
        let mut allowances = self.allowances
            .get(owner_id)
            .unwrap_or_else(|| UnorderedMap::new(format!("allowances-{}", owner_id).as_bytes()));
        
        allowances.insert(spender_id, &amount);
        self.allowances.insert(owner_id, &allowances);
    }
    
    // View methods
    pub fn get_metadata(&self) -> TokenMetadata {
        self.metadata.clone()
    }
    
    pub fn get_minters(&self) -> Vec<AccountId> {
        self.minters.clone()
    }
    
    pub fn is_paused(&self) -> bool {
        self.pause_transfers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn test_token_creation() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);
        
        let total_supply = U128(1_000_000_000_000_000_000_000_000_000);
        let token = VoiceToken::new(accounts(1), total_supply);
        
        assert_eq!(token.ft_total_supply(), total_supply);
        assert_eq!(token.ft_balance_of(accounts(1)), total_supply);
        assert_eq!(token.get_metadata().symbol, "VOICE");
    }
    
    #[test]
    fn test_token_transfer() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);
        
        let mut token = VoiceToken::new(accounts(1), U128(1000));
        token.ft_transfer(accounts(2), U128(100), None);
        
        assert_eq!(token.ft_balance_of(accounts(1)), U128(900));
        assert_eq!(token.ft_balance_of(accounts(2)), U128(100));
    }
    
    #[test]
    fn test_token_mint() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);
        
        let mut token = VoiceToken::new(accounts(1), U128(1000));
        token.mint(accounts(2), U128(500));
        
        assert_eq!(token.ft_total_supply(), U128(1500));
        assert_eq!(token.ft_balance_of(accounts(2)), U128(500));
    }
}
