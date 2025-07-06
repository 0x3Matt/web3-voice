//! DAO Contract - Decentralized Autonomous Organization logic
// (migrated from monolithic contracts/src/dao.rs)

use near_sdk::{
    env, log, AccountId, PanicOnDefault, Promise,
    collections::{LookupMap, UnorderedMap, UnorderedSet},
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
    NearToken,
};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

// JSON-compatible view types for ABI generation
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub enum ProposalStatusView {
    InProgress,
    Approved,
    Rejected,
    Expired,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub enum ProposalTypeView {
    /// Change contract parameters
    ParameterChange {
        parameter: String,
        value: String,
    },
    /// Treasury spending proposal
    TreasurySpend {
        recipient: String,
        amount: String,
        description: String,
    },
    /// Member management proposal
    MembershipChange {
        account_id: String,
        action: String, // "add" or "remove"
    },
    /// General governance proposal
    General {
        description: String,
        actions: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct ProposalView {
    pub id: String,
    pub proposer: String,
    pub proposal_type: ProposalTypeView,
    pub description: String,
    pub votes_for: String,
    pub votes_against: String,
    pub status: ProposalStatusView,
    pub created_at: String,
    pub voting_period_end: String,
    pub execution_period_end: String,
    pub voters: HashMap<String, bool>, // true = for, false = against
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct DAOMemberView {
    pub account_id: String,
    pub voting_power: String,
    pub joined_at: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct DAOConfigView {
    pub voting_period: String, // Duration in nanoseconds
    pub execution_period: String, // Duration in nanoseconds
    pub quorum_threshold: u32, // Percentage (basis points)
    pub approval_threshold: u32, // Percentage (basis points)
    pub proposal_deposit: String, // Amount required to create proposal
    pub min_voting_power: String, // Minimum voting power to participate
}

// Internal contract types (no JsonSchema needed)
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    InProgress,
    Approved,
    Rejected,
    Expired,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalType {
    /// Change contract parameters
    ParameterChange {
        parameter: String,
        value: String,
    },
    /// Treasury spending proposal
    TreasurySpend {
        recipient: AccountId,
        amount: U128,
        description: String,
    },
    /// Member management proposal
    MembershipChange {
        account_id: AccountId,
        action: String, // "add" or "remove"
    },
    /// General governance proposal
    General {
        description: String,
        actions: Vec<String>,
    },
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proposal {
    pub id: U128,
    pub proposer: AccountId,
    pub proposal_type: ProposalType,
    pub description: String,
    pub votes_for: U128,
    pub votes_against: U128,
    pub status: ProposalStatus,
    pub created_at: U64,
    pub voting_period_end: U64,
    pub execution_period_end: U64,
    pub voters: HashMap<AccountId, bool>, // true = for, false = against
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DAOMember {
    pub account_id: AccountId,
    pub voting_power: U128,
    pub joined_at: U64,
    pub is_active: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DAOConfig {
    pub voting_period: U64, // Duration in nanoseconds
    pub execution_period: U64, // Duration in nanoseconds
    pub quorum_threshold: u32, // Percentage (basis points)
    pub approval_threshold: u32, // Percentage (basis points)
    pub proposal_deposit: U128, // Amount required to create proposal
    pub min_voting_power: U128, // Minimum voting power to participate
}

/// The DAO contract for decentralized governance
#[near_sdk::near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DAO {
    /// Contract owner/admin
    pub owner_id: AccountId,
    /// DAO configuration
    pub config: DAOConfig,
    /// Treasury balance
    pub treasury_balance: U128,
    /// Total proposals created
    pub total_proposals: U128,
    /// All proposals
    pub proposals: UnorderedMap<U128, Proposal>,
    /// DAO members
    pub members: LookupMap<AccountId, DAOMember>,
    /// Active members set
    pub active_members: UnorderedSet<AccountId>,
    /// Total voting power
    pub total_voting_power: U128,
    /// Proposal deposits
    pub proposal_deposits: LookupMap<U128, U128>,
}

#[near_sdk::near_bindgen]
impl DAO {
    /// Initialize the DAO
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        let config = DAOConfig {
            voting_period: U64(7 * 24 * 60 * 60 * 1_000_000_000), // 7 days
            execution_period: U64(3 * 24 * 60 * 60 * 1_000_000_000), // 3 days
            quorum_threshold: 1000, // 10%
            approval_threshold: 5000, // 50%
            proposal_deposit: U128(100_000_000_000_000_000_000_000), // 100 VOICE tokens
            min_voting_power: U128(1_000_000_000_000_000_000_000), // 1 VOICE token
        };

        let mut this = Self {
            owner_id: owner_id.clone(),
            config,
            treasury_balance: U128(0),
            total_proposals: U128(0),
            proposals: UnorderedMap::new(b"p".to_vec()),
            members: LookupMap::new(b"m".to_vec()),
            active_members: UnorderedSet::new(b"am".to_vec()),
            total_voting_power: U128(0),
            proposal_deposits: LookupMap::new(b"pd".to_vec()),
        };

        // Add owner as initial member
        let owner_member = DAOMember {
            account_id: owner_id.clone(),
            voting_power: U128(1_000_000_000_000_000_000_000_000), // 1M VOICE tokens
            joined_at: U64(env::block_timestamp()),
            is_active: true,
        };

        this.members.insert(&owner_id, &owner_member);
        this.active_members.insert(&owner_id);
        this.total_voting_power.0 += owner_member.voting_power.0;

        log!("DAO contract deployed. Owner: {}", owner_id);

        this
    }

    /// Add funds to the treasury
    #[payable]
    pub fn add_to_treasury(&mut self) {
        let amount = env::attached_deposit();
        self.treasury_balance.0 += amount.as_yoctonear();
        
        log!("Added {} yoctoNEAR to treasury", amount.as_yoctonear());
    }

    /// Join the DAO as a member
    #[payable]
    pub fn join_dao(&mut self, voting_power: U128) {
        let account_id = env::predecessor_account_id();
        
        assert!(!self.members.contains_key(&account_id), "Already a member");
        assert!(voting_power.0 >= self.config.min_voting_power.0, "Insufficient voting power");
        
        let member = DAOMember {
            account_id: account_id.clone(),
            voting_power,
            joined_at: U64(env::block_timestamp()),
            is_active: true,
        };
        
        self.members.insert(&account_id, &member);
        self.active_members.insert(&account_id);
        self.total_voting_power.0 += voting_power.0;
        
        log!("New member joined: {} with voting power {}", account_id, voting_power.0);
    }

    /// Create a new proposal
    #[payable]
    pub fn create_proposal(&mut self, proposal_type: ProposalTypeView, description: String) -> String {
        let proposer = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        
        // Check if proposer is a member
        let member = self.members.get(&proposer).expect("Only members can create proposals");
        assert!(member.is_active, "Member is not active");
        
        // Check deposit
        assert!(deposit.as_yoctonear() >= self.config.proposal_deposit.0, "Insufficient deposit");
        
        // Convert view type to internal type
        let internal_proposal_type = match proposal_type {
            ProposalTypeView::ParameterChange { parameter, value } => {
                ProposalType::ParameterChange { parameter, value }
            },
            ProposalTypeView::TreasurySpend { recipient, amount, description } => {
                ProposalType::TreasurySpend {
                    recipient: recipient.parse().expect("Invalid account ID"),
                    amount: U128(amount.parse().expect("Invalid amount")),
                    description,
                }
            },
            ProposalTypeView::MembershipChange { account_id, action } => {
                ProposalType::MembershipChange {
                    account_id: account_id.parse().expect("Invalid account ID"),
                    action,
                }
            },
            ProposalTypeView::General { description, actions } => {
                ProposalType::General { description, actions }
            },
        };
        
        let proposal_id = self.total_proposals.0 + 1;
        let voting_period_end = env::block_timestamp() + self.config.voting_period.0;
        let execution_period_end = voting_period_end + self.config.execution_period.0;
        
        let proposal = Proposal {
            id: U128(proposal_id),
            proposer: proposer.clone(),
            proposal_type: internal_proposal_type,
            description,
            votes_for: U128(0),
            votes_against: U128(0),
            status: ProposalStatus::InProgress,
            created_at: U64(env::block_timestamp()),
            voting_period_end: U64(voting_period_end),
            execution_period_end: U64(execution_period_end),
            voters: HashMap::new(),
        };
        
        self.proposals.insert(&U128(proposal_id), &proposal);
        self.proposal_deposits.insert(&U128(proposal_id), &U128(deposit.as_yoctonear()));
        self.total_proposals.0 += 1;
        
        log!("Created proposal {} by {}", proposal_id, proposer);
        
        proposal_id.to_string()
    }

    /// Vote on a proposal
    pub fn vote(&mut self, proposal_id: U128, support: bool) {
        let voter = env::predecessor_account_id();
        let member = self.members.get(&voter).expect("Only members can vote");
        
        assert!(member.is_active, "Member is not active");
        
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        
        // Check if voting is still open
        assert!(env::block_timestamp() < proposal.voting_period_end.0, "Voting period has ended");
        assert!(proposal.status == ProposalStatus::InProgress, "Proposal is not in progress");
        
        // Check if already voted
        assert!(!proposal.voters.contains_key(&voter), "Already voted");
        
        // Record vote
        proposal.voters.insert(voter.clone(), support);
        
        if support {
            proposal.votes_for.0 += member.voting_power.0;
        } else {
            proposal.votes_against.0 += member.voting_power.0;
        }
        
        self.proposals.insert(&proposal_id, &proposal);
        
        log!("Vote recorded: {} voted {} on proposal {}", voter, support, proposal_id.0);
    }

    /// Finalize a proposal after voting period
    pub fn finalize_proposal(&mut self, proposal_id: U128) {
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        
        assert!(env::block_timestamp() >= proposal.voting_period_end.0, "Voting period not ended");
        assert!(proposal.status == ProposalStatus::InProgress, "Proposal already finalized");
        
        let total_votes = proposal.votes_for.0 + proposal.votes_against.0;
        let quorum = (self.total_voting_power.0 * self.config.quorum_threshold as u128) / 10000;
        
        if total_votes < quorum {
            proposal.status = ProposalStatus::Rejected;
            log!("Proposal {} rejected: insufficient quorum", proposal_id.0);
        } else {
            let approval_threshold = (total_votes * self.config.approval_threshold as u128) / 10000;
            if proposal.votes_for.0 >= approval_threshold {
                proposal.status = ProposalStatus::Approved;
                log!("Proposal {} approved", proposal_id.0);
            } else {
                proposal.status = ProposalStatus::Rejected;
                log!("Proposal {} rejected: insufficient approval", proposal_id.0);
            }
        }
        
        self.proposals.insert(&proposal_id, &proposal);
    }

    /// Execute an approved proposal
    pub fn execute_proposal(&mut self, proposal_id: U128) {
        let proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        
        assert!(proposal.status == ProposalStatus::Approved, "Proposal not approved");
        assert!(env::block_timestamp() <= proposal.execution_period_end.0, "Execution period expired");
        
        match &proposal.proposal_type {
            ProposalType::TreasurySpend { recipient, amount, description: _ } => {
                assert!(self.treasury_balance.0 >= amount.0, "Insufficient treasury balance");
                
                Promise::new(recipient.clone()).transfer(NearToken::from_yoctonear(amount.0));
                self.treasury_balance.0 -= amount.0;
                
                log!("Executed treasury spend: {} NEAR to {}", amount.0, recipient);
            },
            ProposalType::MembershipChange { account_id, action } => {
                if action == "remove" {
                    if let Some(member) = self.members.get(account_id) {
                        let mut updated_member = member.clone();
                        updated_member.is_active = false;
                        self.members.insert(account_id, &updated_member);
                        self.active_members.remove(account_id);
                        self.total_voting_power.0 -= member.voting_power.0;
                        
                        log!("Removed member: {}", account_id);
                    }
                }
            },
            _ => {
                log!("Executed proposal {} (type not implemented)", proposal_id.0);
            }
        }
    }

    /// Get proposal details
    pub fn get_proposal(&self, proposal_id: U128) -> Option<ProposalView> {
        self.proposals.get(&proposal_id).map(|p| self.proposal_to_view(p))
    }

    /// Get member details
    pub fn get_member(&self, account_id: AccountId) -> Option<DAOMemberView> {
        self.members.get(&account_id).map(|m| self.member_to_view(m))
    }

    /// Get DAO configuration
    pub fn get_config(&self) -> DAOConfigView {
        self.config_to_view(self.config.clone())
    }

    /// Get treasury balance
    pub fn get_treasury_balance(&self) -> String {
        self.treasury_balance.0.to_string()
    }

    /// Get total proposals
    pub fn get_total_proposals(&self) -> String {
        self.total_proposals.0.to_string()
    }

    /// Get all proposals (paginated)
    pub fn get_proposals(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<ProposalView> {
        let start = from_index.map(|i| i.0 as usize).unwrap_or(0);
        let limit = limit.unwrap_or(10) as usize;
        
        self.proposals
            .values()
            .skip(start)
            .take(limit)
            .map(|p| self.proposal_to_view(p))
            .collect()
    }

    /// Get active members
    pub fn get_active_members(&self) -> Vec<String> {
        self.active_members.to_vec().iter().map(|id| id.to_string()).collect()
    }

    /// Get total voting power
    pub fn get_total_voting_power(&self) -> String {
        self.total_voting_power.0.to_string()
    }

    // Helper methods to convert internal types to view types
    fn proposal_to_view(&self, proposal: Proposal) -> ProposalView {
        ProposalView {
            id: proposal.id.0.to_string(),
            proposer: proposal.proposer.to_string(),
            proposal_type: self.proposal_type_to_view(proposal.proposal_type),
            description: proposal.description,
            votes_for: proposal.votes_for.0.to_string(),
            votes_against: proposal.votes_against.0.to_string(),
            status: match proposal.status {
                ProposalStatus::InProgress => ProposalStatusView::InProgress,
                ProposalStatus::Approved => ProposalStatusView::Approved,
                ProposalStatus::Rejected => ProposalStatusView::Rejected,
                ProposalStatus::Expired => ProposalStatusView::Expired,
            },
            created_at: proposal.created_at.0.to_string(),
            voting_period_end: proposal.voting_period_end.0.to_string(),
            execution_period_end: proposal.execution_period_end.0.to_string(),
            voters: proposal.voters.into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        }
    }

    fn proposal_type_to_view(&self, proposal_type: ProposalType) -> ProposalTypeView {
        match proposal_type {
            ProposalType::ParameterChange { parameter, value } => {
                ProposalTypeView::ParameterChange { parameter, value }
            },
            ProposalType::TreasurySpend { recipient, amount, description } => {
                ProposalTypeView::TreasurySpend {
                    recipient: recipient.to_string(),
                    amount: amount.0.to_string(),
                    description,
                }
            },
            ProposalType::MembershipChange { account_id, action } => {
                ProposalTypeView::MembershipChange {
                    account_id: account_id.to_string(),
                    action,
                }
            },
            ProposalType::General { description, actions } => {
                ProposalTypeView::General { description, actions }
            },
        }
    }

    fn member_to_view(&self, member: DAOMember) -> DAOMemberView {
        DAOMemberView {
            account_id: member.account_id.to_string(),
            voting_power: member.voting_power.0.to_string(),
            joined_at: member.joined_at.0.to_string(),
            is_active: member.is_active,
        }
    }

    fn config_to_view(&self, config: DAOConfig) -> DAOConfigView {
        DAOConfigView {
            voting_period: config.voting_period.0.to_string(),
            execution_period: config.execution_period.0.to_string(),
            quorum_threshold: config.quorum_threshold,
            approval_threshold: config.approval_threshold,
            proposal_deposit: config.proposal_deposit.0.to_string(),
            min_voting_power: config.min_voting_power.0.to_string(),
        }
    }
}
