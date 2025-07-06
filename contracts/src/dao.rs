// Voice DAO Contract - Decentralized governance for the Web3Voice platform
use near_sdk::{
    env, log, near, require, AccountId, BorshStorageKey, PanicOnDefault, Promise,
    collections::{LookupMap, UnorderedMap, UnorderedSet},
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
    NearToken,
};
use borsh::{BorshDeserialize, BorshSerialize};

use std::collections::HashMap;

const MIN_PROPOSAL_DEPOSIT: u128 = 100_000_000_000_000_000_000_000_000; // 100 VOICE tokens
const VOTING_PERIOD: u64 = 7 * 24 * 60 * 60 * 1_000_000_000; // 7 days in nanoseconds
const EXECUTION_DELAY: u64 = 24 * 60 * 60 * 1_000_000_000; // 24 hours in nanoseconds
const QUORUM_THRESHOLD: u32 = 1000; // 10% of total supply
const APPROVAL_THRESHOLD: u32 = 5000; // 50% of votes cast

#[derive(BorshSerialize, BorshDeserialize, BorshStorageKey)]
enum StorageKey {
    Proposals,
    Votes,
    Members,
    Roles,
    Treasury,
    Rewards,
    Contributions,
    Delegations,
    Council,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: AccountId,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub votes_for: u128,
    pub votes_against: u128,
    pub votes_abstain: u128,
    pub total_votes: u128,
    pub created_at: u64,
    pub voting_starts_at: u64,
    pub voting_ends_at: u64,
    pub execution_eta: u64,
    pub deposit: u128,
    pub tags: Vec<String>,
    pub attachments: Vec<String>, // IPFS hashes
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalType {
    Treasury {
        recipient: AccountId,
        amount: u128,
        token: String,
        purpose: String,
    },
    Governance {
        parameter: String,
        new_value: String,
    },
    Membership {
        action: String, // "add" or "remove"
        member: AccountId,
        role: String,
    },
    Content {
        action: String, // "feature", "remove", "reward"
        content_id: String,
        details: String,
    },
    Contract {
        action: String, // "upgrade", "pause", "unpause"
        target_contract: AccountId,
        method: String,
        args: String,
    },
    Partnership {
        partner: AccountId,
        terms: String,
        duration: u64,
    },
    Grant {
        recipient: AccountId,
        amount: u128,
        milestones: Vec<String>,
        category: String,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    Draft,
    Active,
    Succeeded,
    Defeated,
    Queued,
    Executed,
    Cancelled,
    Expired,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Vote {
    pub proposal_id: u64,
    pub voter: AccountId,
    pub support: VoteType,
    pub voting_power: u128,
    pub reason: Option<String>,
    pub created_at: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Member {
    pub account_id: AccountId,
    pub roles: Vec<String>,
    pub voting_power: u128,
    pub contributions: u64,
    pub reputation: u32,
    pub joined_at: u64,
    pub last_active: u64,
    pub delegated_to: Option<AccountId>,
    pub delegated_power: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Contribution {
    pub id: String,
    pub contributor: AccountId,
    pub contribution_type: ContributionType,
    pub title: String,
    pub description: String,
    pub ipfs_hash: String,
    pub reward_amount: u128,
    pub status: ContributionStatus,
    pub created_at: u64,
    pub reviewed_at: Option<u64>,
    pub reviewer: Option<AccountId>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ContributionType {
    VoiceContent,
    CodeCommit,
    Documentation,
    Design,
    Marketing,
    Community,
    BugReport,
    FeatureRequest,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ContributionStatus {
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    Rewarded,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Treasury {
    pub voice_balance: u128,
    pub near_balance: u128,
    pub other_tokens: HashMap<String, u128>,
    pub allocated_funds: u128,
    pub available_funds: u128,
    pub total_spent: u128,
    pub spending_categories: HashMap<String, u128>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DAOStats {
    pub total_proposals: u64,
    pub active_proposals: u64,
    pub total_members: u64,
    pub total_voting_power: u128,
    pub treasury_value: u128,
    pub total_contributions: u64,
    pub governance_participation: f64,
    pub last_updated: u64,
}

// Events
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalCreatedEvent {
    pub proposal_id: u64,
    pub proposer: AccountId,
    pub title: String,
    pub proposal_type: ProposalType,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct VoteCastEvent {
    pub proposal_id: u64,
    pub voter: AccountId,
    pub support: VoteType,
    pub voting_power: u128,
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalExecutedEvent {
    pub proposal_id: u64,
    pub executor: AccountId,
    pub success: bool,
    pub result: String,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct VoiceDAO {
    pub proposals: UnorderedMap<u64, Proposal>,
    pub votes: LookupMap<u64, UnorderedMap<AccountId, Vote>>,
    pub members: UnorderedMap<AccountId, Member>,
    pub contributions: UnorderedMap<String, Contribution>,
    pub treasury: Treasury,
    pub voice_token_contract: AccountId,
    pub owner: AccountId,
    pub council: UnorderedSet<AccountId>,
    pub proposal_counter: u64,
    pub contribution_counter: u64,
    pub governance_config: GovernanceConfig,
    pub roles: HashMap<String, RoleConfig>,
    pub delegations: LookupMap<AccountId, AccountId>,
    pub paused: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GovernanceConfig {
    pub min_proposal_deposit: u128,
    pub voting_period: u64,
    pub execution_delay: u64,
    pub quorum_threshold: u32,
    pub approval_threshold: u32,
    pub max_proposals_per_member: u32,
    pub proposal_fee: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RoleConfig {
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub min_voting_power: u128,
    pub max_members: Option<u32>,
}

#[near]
impl VoiceDAO {
    #[init]
    pub fn new(owner: AccountId, voice_token_contract: AccountId) -> Self {
        let mut dao = Self {
            proposals: UnorderedMap::new(StorageKey::Proposals),
            votes: LookupMap::new(StorageKey::Votes),
            members: UnorderedMap::new(StorageKey::Members),
            contributions: UnorderedMap::new(StorageKey::Contributions),
            treasury: Treasury {
                voice_balance: 0,
                near_balance: 0,
                other_tokens: HashMap::new(),
                allocated_funds: 0,
                available_funds: 0,
                total_spent: 0,
                spending_categories: HashMap::new(),
            },
            voice_token_contract,
            owner: owner.clone(),
            council: UnorderedSet::new(StorageKey::Council),
            proposal_counter: 0,
            contribution_counter: 0,
            governance_config: GovernanceConfig {
                min_proposal_deposit: MIN_PROPOSAL_DEPOSIT,
                voting_period: VOTING_PERIOD,
                execution_delay: EXECUTION_DELAY,
                quorum_threshold: QUORUM_THRESHOLD,
                approval_threshold: APPROVAL_THRESHOLD,
                max_proposals_per_member: 5,
                proposal_fee: 0,
            },
            roles: HashMap::new(),
            delegations: LookupMap::new(StorageKey::Delegations),
            paused: false,
        };
        
        // Initialize default roles
        dao.initialize_roles();
        
        // Add owner as initial member and council
        dao.add_member(owner.clone(), vec!["founder".to_string()]);
        dao.council.insert(&owner);
        
        dao
    }
    
    // =================
    // PROPOSAL METHODS
    // =================
    
    #[payable]
    pub fn create_proposal(&mut self, title: String, description: String, proposal_type: ProposalType, tags: Vec<String>) -> u64 {
        require!(!self.paused, "DAO is paused");
        
        let proposer = env::predecessor_account_id();
        let attached_deposit = env::attached_deposit();
        
        // Validate proposer
        require!(self.members.get(&proposer).is_some(), "Not a DAO member");
        
        // Check deposit requirement
        require!(
            attached_deposit >= NearToken::from_yoctonear(self.governance_config.min_proposal_deposit),
            "Insufficient deposit"
        );
        
        // Check proposal limits
        let active_proposals = self.get_active_proposals_by_member(&proposer);
        require!(
            active_proposals.len() < self.governance_config.max_proposals_per_member as usize,
            "Too many active proposals"
        );
        
        // Generate proposal ID
        self.proposal_counter += 1;
        let proposal_id = self.proposal_counter;
        
        // Create proposal
        let now = env::block_timestamp();
        let proposal = Proposal {
            id: proposal_id,
            title: title.clone(),
            description,
            proposer: proposer.clone(),
            proposal_type: proposal_type.clone(),
            status: ProposalStatus::Active,
            votes_for: 0,
            votes_against: 0,
            votes_abstain: 0,
            total_votes: 0,
            created_at: now,
            voting_starts_at: now,
            voting_ends_at: now + self.governance_config.voting_period,
            execution_eta: 0,
            deposit: attached_deposit.as_yoctonear(),
            tags,
            attachments: Vec::new(),
        };
        
        self.proposals.insert(&proposal_id, &proposal);
        
        // Initialize vote tracking
        self.votes.insert(&proposal_id, &UnorderedMap::new(format!("votes-{}", proposal_id).as_bytes()));
        
        // Emit event
        env::log_str(&format!("EVENT_JSON:{}", serde_json::to_string(&ProposalCreatedEvent {
            proposal_id,
            proposer,
            title,
            proposal_type,
        }).unwrap()));
        
        log!("Created proposal {}", proposal_id);
        proposal_id
    }
    
    pub fn vote_on_proposal(&mut self, proposal_id: u64, support: VoteType, reason: Option<String>) {
        require!(!self.paused, "DAO is paused");
        
        let voter = env::predecessor_account_id();
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        
        // Validate voting period
        require!(proposal.status == ProposalStatus::Active, "Proposal not active");
        require!(
            env::block_timestamp() >= proposal.voting_starts_at &&
            env::block_timestamp() <= proposal.voting_ends_at,
            "Not in voting period"
        );
        
        // Get voting power
        let member = self.members.get(&voter).expect("Not a DAO member");
        let voting_power = self.get_voting_power(&voter);
        require!(voting_power > 0, "No voting power");
        
        // Check if already voted
        let mut votes = self.votes.get(&proposal_id).unwrap();
        if let Some(existing_vote) = votes.get(&voter) {
            // Remove previous vote
            match existing_vote.support {
                VoteType::For => proposal.votes_for -= existing_vote.voting_power,
                VoteType::Against => proposal.votes_against -= existing_vote.voting_power,
                VoteType::Abstain => proposal.votes_abstain -= existing_vote.voting_power,
            }
            proposal.total_votes -= existing_vote.voting_power;
        }
        
        // Add new vote
        let vote = Vote {
            proposal_id,
            voter: voter.clone(),
            support: support.clone(),
            voting_power,
            reason: reason.clone(),
            created_at: env::block_timestamp(),
        };
        
        // Update proposal vote counts
        match support {
            VoteType::For => proposal.votes_for += voting_power,
            VoteType::Against => proposal.votes_against += voting_power,
            VoteType::Abstain => proposal.votes_abstain += voting_power,
        }
        proposal.total_votes += voting_power;
        
        // Store vote
        votes.insert(&voter, &vote);
        self.votes.insert(&proposal_id, &votes);
        self.proposals.insert(&proposal_id, &proposal);
        
        // Update member activity
        self.update_member_activity(&voter);
        
        // Emit event
        env::log_str(&format!("EVENT_JSON:{}", serde_json::to_string(&VoteCastEvent {
            proposal_id,
            voter,
            support,
            voting_power,
            reason,
        }).unwrap()));
        
        log!("Vote cast on proposal {}", proposal_id);
    }
    
    pub fn finalize_proposal(&mut self, proposal_id: u64) {
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        
        require!(proposal.status == ProposalStatus::Active, "Proposal not active");
        require!(env::block_timestamp() > proposal.voting_ends_at, "Voting period not ended");
        
        // Calculate results
        let total_supply = self.get_total_voting_power();
        let quorum_met = (proposal.total_votes * 10000) / total_supply >= self.governance_config.quorum_threshold as u128;
        let approval_met = if proposal.total_votes > 0 {
            (proposal.votes_for * 10000) / (proposal.votes_for + proposal.votes_against) >= self.governance_config.approval_threshold as u128
        } else {
            false
        };
        
        if quorum_met && approval_met {
            proposal.status = ProposalStatus::Succeeded;
            proposal.execution_eta = env::block_timestamp() + self.governance_config.execution_delay;
            
            // Return deposit to proposer
            Promise::new(proposal.proposer.clone()).transfer(NearToken::from_yoctonear(proposal.deposit));
        } else {
            proposal.status = ProposalStatus::Defeated;
            
            // Slash deposit (send to treasury)
            self.treasury.near_balance += proposal.deposit;
        }
        
        self.proposals.insert(&proposal_id, &proposal);
        
        log!("Proposal {} finalized with status {:?}", proposal_id, proposal.status);
    }
    
    pub fn queue_proposal(&mut self, proposal_id: u64) {
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        
        require!(proposal.status == ProposalStatus::Succeeded, "Proposal not succeeded");
        require!(env::block_timestamp() >= proposal.execution_eta, "Execution delay not met");
        
        proposal.status = ProposalStatus::Queued;
        self.proposals.insert(&proposal_id, &proposal);
        
        log!("Proposal {} queued for execution", proposal_id);
    }
    
    pub fn execute_proposal(&mut self, proposal_id: u64) {
        let executor = env::predecessor_account_id();
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        
        require!(proposal.status == ProposalStatus::Queued, "Proposal not queued");
        require!(
            self.council.contains(&executor) || executor == self.owner,
            "Not authorized to execute"
        );
        
        // Execute based on proposal type
        let success = match &proposal.proposal_type {
            ProposalType::Treasury { recipient, amount, token, purpose } => {
                self.execute_treasury_proposal(recipient, *amount, token, purpose)
            },
            ProposalType::Governance { parameter, new_value } => {
                self.execute_governance_proposal(&parameter, &new_value)
            },
            ProposalType::Membership { action, member, role } => {
                self.execute_membership_proposal(&action, &member, &role)
            },
            ProposalType::Content { action, content_id, details } => {
                self.execute_content_proposal(&action, &content_id, &details)
            },
            ProposalType::Grant { recipient, amount, milestones, category } => {
                self.execute_grant_proposal(recipient, *amount, milestones, category)
            },
            _ => {
                log!("Proposal type not implemented for execution");
                false
            }
        };
        
        if success {
            proposal.status = ProposalStatus::Executed;
            log!("Proposal {} executed successfully", proposal_id);
        } else {
            log!("Proposal {} execution failed", proposal_id);
        }
        
        self.proposals.insert(&proposal_id, &proposal);
        
        // Emit event
        env::log_str(&format!("EVENT_JSON:{}", serde_json::to_string(&ProposalExecutedEvent {
            proposal_id,
            executor,
            success,
            result: if success { "Success".to_string() } else { "Failed".to_string() },
        }).unwrap()));
    }
    
    // =================
    // CONTRIBUTION METHODS
    // =================
    
    pub fn submit_contribution(&mut self, contribution_type: ContributionType, title: String, description: String, ipfs_hash: String) -> String {
        let contributor = env::predecessor_account_id();
        
        self.contribution_counter += 1;
        let contribution_id = format!("contribution-{}", self.contribution_counter);
        
        let contribution = Contribution {
            id: contribution_id.clone(),
            contributor: contributor.clone(),
            contribution_type,
            title,
            description,
            ipfs_hash,
            reward_amount: 0,
            status: ContributionStatus::Submitted,
            created_at: env::block_timestamp(),
            reviewed_at: None,
            reviewer: None,
        };
        
        self.contributions.insert(&contribution_id, &contribution);
        
        // Update member contribution count
        if let Some(mut member) = self.members.get(&contributor) {
            member.contributions += 1;
            self.members.insert(&contributor, &member);
        }
        
        log!("Contribution {} submitted by {}", contribution_id, contributor);
        contribution_id
    }
    
    pub fn review_contribution(&mut self, contribution_id: String, approved: bool, reward_amount: U128) {
        let reviewer = env::predecessor_account_id();
        require!(self.council.contains(&reviewer), "Not authorized to review");
        
        let mut contribution = self.contributions.get(&contribution_id).expect("Contribution not found");
        
        contribution.status = if approved {
            ContributionStatus::Approved
        } else {
            ContributionStatus::Rejected
        };
        
        contribution.reward_amount = reward_amount.into();
        contribution.reviewed_at = Some(env::block_timestamp());
        contribution.reviewer = Some(reviewer.clone());
        
        self.contributions.insert(&contribution_id, &contribution);
        
        // If approved, add to reputation and potentially reward
        if approved {
            self.update_member_reputation(&contribution.contributor, 10);
            
            if contribution.reward_amount > 0 {
                self.distribute_reward(&contribution.contributor, contribution.reward_amount);
            }
        }
        
        log!("Contribution {} reviewed by {}, approved: {}", contribution_id, reviewer, approved);
    }
    
    // =================
    // MEMBER METHODS
    // =================
    
    pub fn join_dao(&mut self) {
        let member = env::predecessor_account_id();
        require!(self.members.get(&member).is_none(), "Already a member");
        
        self.add_member(member.clone(), vec!["member".to_string()]);
        log!("Member {} joined DAO", member);
    }
    
    pub fn delegate_voting_power(&mut self, delegate: AccountId) {
        let delegator = env::predecessor_account_id();
        require!(self.members.get(&delegator).is_some(), "Not a member");
        require!(self.members.get(&delegate).is_some(), "Delegate not a member");
        require!(delegator != delegate, "Cannot delegate to self");
        
        self.delegations.insert(&delegator, &delegate);
        
        log!("Voting power delegated from {} to {}", delegator, delegate);
    }
    
    pub fn undelegate_voting_power(&mut self) {
        let delegator = env::predecessor_account_id();
        self.delegations.remove(&delegator);
        
        log!("Voting power undelegated by {}", delegator);
    }
    
    // =================
    // TREASURY METHODS
    // =================
    
    #[payable]
    pub fn contribute_to_treasury(&mut self) {
        let contributor = env::predecessor_account_id();
        let amount = env::attached_deposit();
        
        self.treasury.near_balance += amount.as_yoctonear();
        self.treasury.available_funds += amount.as_yoctonear();
        
        log!("Contributor {} added {} NEAR to treasury", contributor, amount.as_yoctonear());
    }
    
    pub fn get_treasury_balance(&self) -> Treasury {
        self.treasury.clone()
    }
    
    // =================
    // ADMIN METHODS
    // =================
    
    pub fn add_council_member(&mut self, member: AccountId) {
        require!(env::predecessor_account_id() == self.owner, "Only owner");
        self.council.insert(&member);
        log!("Added {} to council", member);
    }
    
    pub fn remove_council_member(&mut self, member: AccountId) {
        require!(env::predecessor_account_id() == self.owner, "Only owner");
        self.council.remove(&member);
        log!("Removed {} from council", member);
    }
    
    pub fn update_governance_config(&mut self, config: GovernanceConfig) {
        require!(env::predecessor_account_id() == self.owner, "Only owner");
        self.governance_config = config;
        log!("Updated governance configuration");
    }
    
    pub fn pause_dao(&mut self) {
        require!(env::predecessor_account_id() == self.owner, "Only owner");
        self.paused = true;
        log!("DAO paused");
    }
    
    pub fn unpause_dao(&mut self) {
        require!(env::predecessor_account_id() == self.owner, "Only owner");
        self.paused = false;
        log!("DAO unpaused");
    }
    
    // =================
    // INTERNAL METHODS
    // =================
    
    fn initialize_roles(&mut self) {
        // Founder role
        self.roles.insert("founder".to_string(), RoleConfig {
            name: "Founder".to_string(),
            description: "Founding member with full privileges".to_string(),
            permissions: vec!["all".to_string()],
            min_voting_power: 0,
            max_members: Some(10),
        });
        
        // Council role
        self.roles.insert("council".to_string(), RoleConfig {
            name: "Council".to_string(),
            description: "Council member with executive privileges".to_string(),
            permissions: vec!["execute".to_string(), "review".to_string(), "moderate".to_string()],
            min_voting_power: 1_000_000_000_000_000_000_000_000_000,
            max_members: Some(21),
        });
        
        // Core contributor role
        self.roles.insert("core".to_string(), RoleConfig {
            name: "Core Contributor".to_string(),
            description: "Core contributor with elevated privileges".to_string(),
            permissions: vec!["propose".to_string(), "review".to_string()],
            min_voting_power: 100_000_000_000_000_000_000_000_000,
            max_members: None,
        });
        
        // Member role
        self.roles.insert("member".to_string(), RoleConfig {
            name: "Member".to_string(),
            description: "Regular DAO member".to_string(),
            permissions: vec!["vote".to_string(), "propose".to_string()],
            min_voting_power: 0,
            max_members: None,
        });
    }
    
    fn add_member(&mut self, account_id: AccountId, roles: Vec<String>) {
        let member = Member {
            account_id: account_id.clone(),
            roles,
            voting_power: 0,
            contributions: 0,
            reputation: 0,
            joined_at: env::block_timestamp(),
            last_active: env::block_timestamp(),
            delegated_to: None,
            delegated_power: 0,
        };
        
        self.members.insert(&account_id, &member);
    }
    
    fn get_voting_power(&self, account_id: &AccountId) -> u128 {
        // Base voting power from token balance
        let mut voting_power = 0; // TODO: Get from token contract
        
        // Add delegated power
        if let Some(member) = self.members.get(account_id) {
            voting_power += member.delegated_power;
        }
        
        voting_power
    }
    
    fn get_total_voting_power(&self) -> u128 {
        // TODO: Get total supply from token contract
        1_000_000_000_000_000_000_000_000_000
    }
    
    fn update_member_activity(&mut self, account_id: &AccountId) {
        if let Some(mut member) = self.members.get(account_id) {
            member.last_active = env::block_timestamp();
            self.members.insert(account_id, &member);
        }
    }
    
    fn update_member_reputation(&mut self, account_id: &AccountId, points: u32) {
        if let Some(mut member) = self.members.get(account_id) {
            member.reputation += points;
            self.members.insert(account_id, &member);
        }
    }
    
    fn distribute_reward(&mut self, recipient: &AccountId, amount: u128) {
        if self.treasury.voice_balance >= amount {
            self.treasury.voice_balance -= amount;
            self.treasury.total_spent += amount;
            
            // TODO: Transfer tokens to recipient
            log!("Distributed {} VOICE tokens to {}", amount, recipient);
        }
    }
    
    fn get_active_proposals_by_member(&self, member: &AccountId) -> Vec<Proposal> {
        self.proposals
            .iter()
            .filter(|(_, proposal)| {
                proposal.proposer == *member && proposal.status == ProposalStatus::Active
            })
            .map(|(_, proposal)| proposal)
            .collect()
    }
    
    fn execute_treasury_proposal(&mut self, recipient: &AccountId, amount: u128, token: &str, purpose: &str) -> bool {
        if token == "VOICE" && self.treasury.voice_balance >= amount {
            self.treasury.voice_balance -= amount;
            self.treasury.total_spent += amount;
            
            // Update spending category
            let category_total = self.treasury.spending_categories.get(purpose).unwrap_or(&0) + amount;
            self.treasury.spending_categories.insert(purpose.to_string(), category_total);
            
            log!("Treasury proposal executed: {} {} sent to {} for {}", amount, token, recipient, purpose);
            true
        } else if token == "NEAR" && self.treasury.near_balance >= amount {
            self.treasury.near_balance -= amount;
            self.treasury.total_spent += amount;
            
            Promise::new(recipient.clone()).transfer(NearToken::from_yoctonear(amount));
            
            log!("Treasury proposal executed: {} {} sent to {} for {}", amount, token, recipient, purpose);
            true
        } else {
            false
        }
    }
    
    fn execute_governance_proposal(&mut self, parameter: &str, new_value: &str) -> bool {
        match parameter {
            "voting_period" => {
                if let Ok(value) = new_value.parse::<u64>() {
                    self.governance_config.voting_period = value;
                    true
                } else {
                    false
                }
            },
            "quorum_threshold" => {
                if let Ok(value) = new_value.parse::<u32>() {
                    self.governance_config.quorum_threshold = value;
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }
    
    fn execute_membership_proposal(&mut self, action: &str, member: &AccountId, role: &str) -> bool {
        match action {
            "add" => {
                if self.members.get(member).is_none() {
                    self.add_member(member.clone(), vec![role.to_string()]);
                    true
                } else {
                    false
                }
            },
            "remove" => {
                if self.members.get(member).is_some() {
                    self.members.remove(member);
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }
    
    fn execute_content_proposal(&mut self, action: &str, content_id: &str, details: &str) -> bool {
        // TODO: Implement content-related actions
        log!("Content proposal executed: {} for content {} with details {}", action, content_id, details);
        true
    }
    
    fn execute_grant_proposal(&mut self, recipient: &AccountId, amount: u128, milestones: &[String], category: &String) -> bool {
        if self.treasury.voice_balance >= amount {
            // For now, distribute the full amount
            // In a real implementation, this would be milestone-based
            self.distribute_reward(recipient, amount);
            
            let category_total = self.treasury.spending_categories.get(category).unwrap_or(&0) + amount;
            self.treasury.spending_categories.insert(category.to_string(), category_total);
            
            log!("Grant proposal executed: {} VOICE granted to {} for {}", amount, recipient, category);
            true
        } else {
            false
        }
    }
    
    // =================
    // VIEW METHODS
    // =================
    
    pub fn get_proposal(&self, proposal_id: u64) -> Option<Proposal> {
        self.proposals.get(&proposal_id)
    }
    
    pub fn get_proposals(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Proposal> {
        let start = from_index.map(|v| v.0).unwrap_or(0) as usize;
        let limit = limit.unwrap_or(50) as usize;
        
        self.proposals
            .iter()
            .skip(start)
            .take(limit)
            .map(|(_, proposal)| proposal)
            .collect()
    }
    
    pub fn get_active_proposals(&self) -> Vec<Proposal> {
        self.proposals
            .iter()
            .filter(|(_, proposal)| proposal.status == ProposalStatus::Active)
            .map(|(_, proposal)| proposal)
            .collect()
    }
    
    pub fn get_member(&self, account_id: AccountId) -> Option<Member> {
        self.members.get(&account_id)
    }
    
    pub fn get_members(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Member> {
        let start = from_index.map(|v| v.0).unwrap_or(0) as usize;
        let limit = limit.unwrap_or(50) as usize;
        
        self.members
            .iter()
            .skip(start)
            .take(limit)
            .map(|(_, member)| member)
            .collect()
    }
    
    pub fn get_contribution(&self, contribution_id: String) -> Option<Contribution> {
        self.contributions.get(&contribution_id)
    }
    
    pub fn get_contributions_by_member(&self, member: AccountId) -> Vec<Contribution> {
        self.contributions
            .iter()
            .filter(|(_, contribution)| contribution.contributor == member)
            .map(|(_, contribution)| contribution)
            .collect()
    }
    
    pub fn get_dao_stats(&self) -> DAOStats {
        DAOStats {
            total_proposals: self.proposal_counter,
            active_proposals: self.get_active_proposals().len() as u64,
            total_members: self.members.len() as u64,
            total_voting_power: self.get_total_voting_power(),
            treasury_value: self.treasury.voice_balance + self.treasury.near_balance,
            total_contributions: self.contribution_counter,
            governance_participation: 0.0, // TODO: Calculate
            last_updated: env::block_timestamp(),
        }
    }
    
    pub fn get_governance_config(&self) -> GovernanceConfig {
        self.governance_config.clone()
    }
    
    pub fn get_council_members(&self) -> Vec<AccountId> {
        self.council.iter().collect()
    }
    
    pub fn is_member(&self, account_id: AccountId) -> bool {
        self.members.get(&account_id).is_some()
    }
    
    pub fn is_council_member(&self, account_id: AccountId) -> bool {
        self.council.contains(&account_id)
    }
    
    pub fn is_paused(&self) -> bool {
        self.paused
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn test_dao_initialization() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(1))
            .build();
        testing_env!(context);
        
        let dao = VoiceDAO::new(accounts(1), accounts(2));
        
        assert_eq!(dao.owner, accounts(1));
        assert_eq!(dao.voice_token_contract, accounts(2));
        assert!(dao.is_member(accounts(1)));
        assert!(dao.is_council_member(accounts(1)));
        assert!(!dao.is_paused());
    }
    
    #[test]
    fn test_join_dao() {
        let context = VMContextBuilder::new()
            .current_account_id(accounts(0))
            .predecessor_account_id(accounts(2))
            .build();
        testing_env!(context);
        
        let mut dao = VoiceDAO::new(accounts(1), accounts(2));
        dao.join_dao();
        
        assert!(dao.is_member(accounts(2)));
        let member = dao.get_member(accounts(2)).unwrap();
        assert_eq!(member.roles, vec!["member".to_string()]);
    }
}
