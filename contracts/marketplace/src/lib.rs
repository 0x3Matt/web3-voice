//! Marketplace Contract - Decentralized marketplace for voice NFTs
// (migrated from monolithic contracts/src/marketplace.rs)
// include!("../../src/marketplace.rs");

use near_sdk::{
    env, log, AccountId, PanicOnDefault, Promise,
    collections::{LookupMap, UnorderedSet},
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
    NearToken,
};
use borsh::{BorshDeserialize, BorshSerialize};

const MARKETPLACE_FEE: u32 = 250; // 2.5% marketplace fee
const ROYALTY_CAP: u32 = 2000; // 20% max royalty
const MIN_PRICE: u128 = 1_000_000_000_000_000_000_000_000; // 1 VOICE token minimum
const LISTING_DURATION: u64 = 30 * 24 * 60 * 60 * 1_000_000_000; // 30 days in nanoseconds

// JSON-compatible types for view methods
#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct MarketItemView {
    pub id: String,
    pub item_type: String,
    pub seller_id: String,
    pub price: Option<String>,
    pub buyer_id: Option<String>,
    pub start_price: Option<String>,
    pub end_price: Option<String>,
    pub royalty: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct SaleView {
    pub id: String,
    pub seller_id: String,
    pub price: String,
    pub royalty: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct AuctionView {
    pub id: String,
    pub seller_id: String,
    pub start_price: String,
    pub end_price: String,
    pub royalty: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub highest_bidder_id: Option<String>,
    pub highest_bid: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct BidView {
    pub id: String,
    pub bidder_id: String,
    pub auction_id: String,
    pub amount: String,
    pub created_at: String,
}

// Internal contract types (no JsonSchema needed)
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub enum MarketItemType {
    /// An item that is for sale
    Sale {
        /// The price of the item in yoctoNEAR
        price: U128,
        /// The account ID of the seller
        seller_id: AccountId,
        /// The royalty percentage for the item
        royalty: Option<U128>,
    },
    /// An item that is being auctioned
    Auction {
        /// The starting price of the auction in yoctoNEAR
        start_price: U128,
        /// The account ID of the seller
        seller_id: AccountId,
        /// The ending price of the auction in yoctoNEAR
        end_price: U128,
        /// The royalty percentage for the item
        royalty: Option<U128>,
        /// The start time of the auction
        start_time: U64,
        /// The end time of the auction
        end_time: U64,
    },
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub struct MarketItem {
    /// The ID of the item
    pub id: U128,
    /// The type of the item (sale or auction)
    pub item_type: MarketItemType,
    /// The account ID of the buyer
    pub buyer_id: Option<AccountId>,
    /// The timestamp when the item was created
    pub created_at: U64,
    /// The timestamp when the item was last updated
    pub updated_at: U64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub struct Sale {
    /// The ID of the sale
    pub id: U128,
    /// The account ID of the seller
    pub seller_id: AccountId,
    /// The price of the item in yoctoNEAR
    pub price: U128,
    /// The royalty percentage for the item
    pub royalty: Option<U128>,
    /// The timestamp when the sale was created
    pub created_at: U64,
    /// The timestamp when the sale was last updated
    pub updated_at: U64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub struct Auction {
    /// The ID of the auction
    pub id: U128,
    /// The account ID of the seller
    pub seller_id: AccountId,
    /// The starting price of the auction in yoctoNEAR
    pub start_price: U128,
    /// The ending price of the auction in yoctoNEAR
    pub end_price: U128,
    /// The royalty percentage for the item
    pub royalty: Option<U128>,
    /// The start time of the auction
    pub start_time: U64,
    /// The end time of the auction
    pub end_time: U64,
    /// The account ID of the highest bidder
    pub highest_bidder_id: Option<AccountId>,
    /// The highest bid amount in yoctoNEAR
    pub highest_bid: Option<U128>,
    /// The timestamp when the auction was created
    pub created_at: U64,
    /// The timestamp when the auction was last updated
    pub updated_at: U64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq, Eq)]
pub struct Bid {
    /// The ID of the bid
    pub id: U128,
    /// The account ID of the bidder
    pub bidder_id: AccountId,
    /// The auction ID that this bid is for
    pub auction_id: U128,
    /// The bid amount in yoctoNEAR
    pub amount: U128,
    /// The timestamp when the bid was placed
    pub created_at: U64,
}

/// The Marketplace contract.
#[near_sdk::near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Marketplace {
    /// The owner of the contract
    pub owner_id: AccountId,
    /// The total number of items created
    pub total_items: U128,
    /// The total number of sales created
    pub total_sales: U128,
    /// The total number of auctions created
    pub total_auctions: U128,
    /// The total number of bids placed
    pub total_bids: U128,
    /// The mapping of item IDs to items
    pub items: LookupMap<U128, MarketItem>,
    /// The mapping of sale IDs to sales
    pub sales: LookupMap<U128, Sale>,
    /// The mapping of auction IDs to auctions
    pub auctions: LookupMap<U128, Auction>,
    /// The mapping of bid IDs to bids
    pub bids: LookupMap<U128, Bid>,
    /// The mapping of account IDs to their sold items
    pub sales_by_seller: LookupMap<AccountId, UnorderedSet<U128>>,
    /// The mapping of account IDs to their purchased items
    pub sales_by_buyer: LookupMap<AccountId, UnorderedSet<U128>>,
    /// The mapping of account IDs to their created auctions
    pub auctions_by_seller: LookupMap<AccountId, UnorderedSet<U128>>,
    /// The mapping of account IDs to their bids
    pub bids_by_bidder: LookupMap<AccountId, UnorderedSet<U128>>,
}

#[near_sdk::near_bindgen]
impl Marketplace {
    /// Creates a new Marketplace contract.
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        let this = Self {
            owner_id: owner_id.clone(),
            total_items: U128(0),
            total_sales: U128(0),
            total_auctions: U128(0),
            total_bids: U128(0),
            items: LookupMap::new(b"i".to_vec()),
            sales: LookupMap::new(b"s".to_vec()),
            auctions: LookupMap::new(b"a".to_vec()),
            bids: LookupMap::new(b"b".to_vec()),
            sales_by_seller: LookupMap::new(b"ss".to_vec()),
            sales_by_buyer: LookupMap::new(b"sb".to_vec()),
            auctions_by_seller: LookupMap::new(b"as".to_vec()),
            bids_by_bidder: LookupMap::new(b"bb".to_vec()),
        };

        // Log the initial owner
        log!("Marketplace contract deployed. Owner: {}", owner_id);

        this
    }

    /// Creates a new item for sale.
    #[payable]
    pub fn create_sale(
        &mut self,
        price: U128,
        royalty: Option<U128>,
    ) -> U128 {
        let seller_id = env::predecessor_account_id();
        let id = self.total_items.0 + 1;

        // Create the sale
        let sale = Sale {
            id: U128(id),
            seller_id: seller_id.clone(),
            price,
            royalty,
            created_at: U64(env::block_timestamp() / 1_000_000),
            updated_at: U64(env::block_timestamp() / 1_000_000),
        };

        // Insert the sale into the sales map
        self.sales.insert(&U128(id), &sale);

        // Add the sale ID to the seller's list of sales
        let mut sales_by_seller = self.sales_by_seller.get(&seller_id).unwrap_or_else(|| UnorderedSet::new(seller_id.as_bytes().to_vec()));
        sales_by_seller.insert(&U128(id));
        self.sales_by_seller.insert(&seller_id, &sales_by_seller);

        // Increment the total sales counter
        self.total_sales.0 += 1;

        // Return the sale ID
        U128(id)
    }

    /// Buys an item for sale.
    #[payable]
    pub fn buy_item(&mut self, item_id: U128) {
        let buyer_id = env::predecessor_account_id();
        let item = self.items.get(&item_id).expect("Item not found");

        // Ensure the item is for sale
        match item.item_type {

            MarketItemType::Sale { ref seller_id, price, royalty } => {
                // Ensure the attached deposit is at least the price of the item
                assert!(env::attached_deposit().as_yoctonear() >= price.0, "Insufficient deposit");

                // Calculate the marketplace fee
                let fee = (price.0 * MARKETPLACE_FEE as u128) / 10000;
                let proceeds = price.0 - fee;

                // Transfer the marketplace fee to the contract owner
                Promise::new(self.owner_id.clone()).transfer(NearToken::from_yoctonear(fee));

                // Transfer the proceeds to the seller
                Promise::new(seller_id.clone()).transfer(NearToken::from_yoctonear(proceeds));

                // Update the item's buyer ID
                let mut item = item.clone();
                item.buyer_id = Some(buyer_id.clone());
                self.items.insert(&item_id, &item);

                // Add the item ID to the buyer's list of purchased items
                let mut sales_by_buyer = self.sales_by_buyer.get(&buyer_id).unwrap_or_else(|| UnorderedSet::new(buyer_id.as_bytes().to_vec()));
                sales_by_buyer.insert(&item_id);
                self.sales_by_buyer.insert(&buyer_id, &sales_by_buyer);
            },
            _ => panic!("Item is not for sale"),
        }
    }

    /// Creates a new auction.
    #[payable]
    pub fn create_auction(
        &mut self,
        start_price: U128,
        end_price: U128,
        royalty: Option<U128>,
        start_time: U64,
        end_time: U64,
    ) -> U128 {
        let seller_id = env::predecessor_account_id();
        let id = self.total_items.0 + 1;

        // Create the auction
        let auction = Auction {
            id: U128(id),
            seller_id: seller_id.clone(),
            start_price,
            end_price,
            royalty,
            start_time,
            end_time,
            highest_bidder_id: None,
            highest_bid: None,
            created_at: U64(env::block_timestamp() / 1_000_000),
            updated_at: U64(env::block_timestamp() / 1_000_000),
        };

        // Insert the auction into the auctions map
        self.auctions.insert(&U128(id), &auction);

        // Add the auction ID to the seller's list of auctions
        let mut auctions_by_seller = self.auctions_by_seller.get(&seller_id).unwrap_or_else(|| UnorderedSet::new(seller_id.as_bytes().to_vec()));
        auctions_by_seller.insert(&U128(id));
        self.auctions_by_seller.insert(&seller_id, &auctions_by_seller);

        // Increment the total auctions counter
        self.total_auctions.0 += 1;

        // Return the auction ID
        U128(id)
    }

    /// Places a bid on an auction.
    #[payable]
    pub fn place_bid(&mut self, auction_id: U128) {
        let bidder_id = env::predecessor_account_id();
        let auction = self.auctions.get(&auction_id).expect("Auction not found");

        // Ensure the auction is active
        assert!(env::block_timestamp() >= auction.start_time.0, "Auction has not started");
        assert!(env::block_timestamp() <= auction.end_time.0, "Auction has ended");

        // Ensure the bid is higher than the current highest bid
        let highest_bid = auction.highest_bid.unwrap_or(U128(0));

        assert!(env::attached_deposit().as_yoctonear() > highest_bid.0, "Bid is not high enough");

        // Calculate the marketplace fee
        let deposit = env::attached_deposit().as_yoctonear();
        let fee = (deposit * MARKETPLACE_FEE as u128) / 10000;
        let proceeds = deposit - fee;

        // Transfer the marketplace fee to the contract owner
        Promise::new(self.owner_id.clone()).transfer(NearToken::from_yoctonear(fee));

        // Transfer the proceeds to the seller
        Promise::new(auction.seller_id.clone()).transfer(NearToken::from_yoctonear(proceeds));

        // Update the auction with the new highest bid
        let mut auction = auction.clone();
        auction.highest_bidder_id = Some(bidder_id.clone());
        auction.highest_bid = Some(U128(env::attached_deposit().as_yoctonear()));
        self.auctions.insert(&auction_id, &auction);

        // Add the bid ID to the bidder's list of bids
        let mut bids_by_bidder = self.bids_by_bidder.get(&bidder_id).unwrap_or_else(|| UnorderedSet::new(bidder_id.as_bytes().to_vec()));
        bids_by_bidder.insert(&U128(self.total_bids.0 + 1));
        self.bids_by_bidder.insert(&bidder_id, &bids_by_bidder);

        // Increment the total bids counter
        self.total_bids.0 += 1;
    }

    /// Ends an auction and transfers the item to the highest bidder.
    pub fn end_auction(&mut self, auction_id: U128) {
        let auction = self.auctions.get(&auction_id).expect("Auction not found");

        // Ensure the auction has ended
        assert!(env::block_timestamp() > auction.end_time.0, "Auction has not ended yet");

        // Transfer the item to the highest bidder
        if let Some(highest_bidder_id) = auction.highest_bidder_id {
            let item = self.items.get(&auction.id).expect("Item not found");
            let mut item = item.clone();
            item.buyer_id = Some(highest_bidder_id.clone());
            self.items.insert(&auction.id, &item);

            // Add the item ID to the highest bidder's list of purchased items
            let mut sales_by_buyer = self.sales_by_buyer.get(&highest_bidder_id).unwrap_or_else(|| UnorderedSet::new(highest_bidder_id.as_bytes().to_vec()));
            sales_by_buyer.insert(&auction.id);
            self.sales_by_buyer.insert(&highest_bidder_id, &sales_by_buyer);
        }

        // Remove the auction
        self.auctions.remove(&auction_id);
    }

    /// Gets the details of an item.
    pub fn get_item(&self, item_id: U128) -> Option<MarketItemView> {
        self.items.get(&item_id).map(|item| self.item_to_view(item))
    }

    /// Gets the details of a sale.
    pub fn get_sale(&self, sale_id: U128) -> Option<SaleView> {
        self.sales.get(&sale_id).map(|sale| self.sale_to_view(sale))
    }

    /// Gets the details of an auction.
    pub fn get_auction(&self, auction_id: U128) -> Option<AuctionView> {
        self.auctions.get(&auction_id).map(|auction| self.auction_to_view(auction))
    }

    /// Gets the details of a bid.
    pub fn get_bid(&self, bid_id: U128) -> Option<BidView> {
        self.bids.get(&bid_id).map(|bid| self.bid_to_view(bid))
    }

    /// Gets the list of item IDs owned by an account.
    pub fn get_items_by_owner(&self, owner_id: AccountId) -> Vec<String> {
        let mut items = Vec::new();

        // Get the sales owned by the account
        if let Some(sales) = self.sales_by_seller.get(&owner_id) {
            for sale_id in sales.to_vec() {
                if let Some(sale) = self.sales.get(&sale_id) {
                    items.push(sale.id.0.to_string());
                }
            }
        }

        // Get the auctions owned by the account
        if let Some(auctions) = self.auctions_by_seller.get(&owner_id) {
            for auction_id in auctions.to_vec() {
                if let Some(auction) = self.auctions.get(&auction_id) {
                    items.push(auction.id.0.to_string());
                }
            }
        }

        items
    }

    /// Gets the list of sale IDs for an account.
    pub fn get_sales_by_seller(&self, seller_id: AccountId) -> Vec<String> {
        self.sales_by_seller.get(&seller_id).map_or_else(Vec::new, |set| 
            set.to_vec().into_iter().map(|id| id.0.to_string()).collect()
        )
    }

    /// Gets the list of auction IDs for an account.
    pub fn get_auctions_by_seller(&self, seller_id: AccountId) -> Vec<String> {
        self.auctions_by_seller.get(&seller_id).map_or_else(Vec::new, |set| 
            set.to_vec().into_iter().map(|id| id.0.to_string()).collect()
        )
    }

    /// Gets the list of bid IDs for a bidder.
    pub fn get_bids_by_bidder(&self, bidder_id: AccountId) -> Vec<String> {
        self.bids_by_bidder.get(&bidder_id).map_or_else(Vec::new, |set| 
            set.to_vec().into_iter().map(|id| id.0.to_string()).collect()
        )
    }

    /// Gets the total number of items.
    pub fn get_total_items(&self) -> String {
        self.total_items.0.to_string()
    }

    /// Gets the total number of sales.
    pub fn get_total_sales(&self) -> String {
        self.total_sales.0.to_string()
    }

    /// Gets the total number of auctions.
    pub fn get_total_auctions(&self) -> String {
        self.total_auctions.0.to_string()
    }

    /// Gets the total number of bids.
    pub fn get_total_bids(&self) -> String {
        self.total_bids.0.to_string()
    }

    // Helper methods to convert internal types to view types
    fn item_to_view(&self, item: MarketItem) -> MarketItemView {
        match item.item_type {
            MarketItemType::Sale { seller_id, price, royalty } => MarketItemView {
                id: item.id.0.to_string(),
                item_type: "sale".to_string(),
                seller_id: seller_id.to_string(),
                price: Some(price.0.to_string()),
                buyer_id: item.buyer_id.map(|id| id.to_string()),
                start_price: None,
                end_price: None,
                royalty: royalty.map(|r| r.0.to_string()),
                start_time: None,
                end_time: None,
                created_at: item.created_at.0.to_string(),
                updated_at: item.updated_at.0.to_string(),
            },
            MarketItemType::Auction { seller_id, start_price, end_price, royalty, start_time, end_time } => MarketItemView {
                id: item.id.0.to_string(),
                item_type: "auction".to_string(),
                seller_id: seller_id.to_string(),
                price: None,
                buyer_id: item.buyer_id.map(|id| id.to_string()),
                start_price: Some(start_price.0.to_string()),
                end_price: Some(end_price.0.to_string()),
                royalty: royalty.map(|r| r.0.to_string()),
                start_time: Some(start_time.0.to_string()),
                end_time: Some(end_time.0.to_string()),
                created_at: item.created_at.0.to_string(),
                updated_at: item.updated_at.0.to_string(),
            },
        }
    }

    fn sale_to_view(&self, sale: Sale) -> SaleView {
        SaleView {
            id: sale.id.0.to_string(),
            seller_id: sale.seller_id.to_string(),
            price: sale.price.0.to_string(),
            royalty: sale.royalty.map(|r| r.0.to_string()),
            created_at: sale.created_at.0.to_string(),
            updated_at: sale.updated_at.0.to_string(),
        }
    }

    fn auction_to_view(&self, auction: Auction) -> AuctionView {
        AuctionView {
            id: auction.id.0.to_string(),
            seller_id: auction.seller_id.to_string(),
            start_price: auction.start_price.0.to_string(),
            end_price: auction.end_price.0.to_string(),
            royalty: auction.royalty.map(|r| r.0.to_string()),
            start_time: auction.start_time.0.to_string(),
            end_time: auction.end_time.0.to_string(),
            highest_bidder_id: auction.highest_bidder_id.map(|id| id.to_string()),
            highest_bid: auction.highest_bid.map(|b| b.0.to_string()),
            created_at: auction.created_at.0.to_string(),
            updated_at: auction.updated_at.0.to_string(),
        }
    }

    fn bid_to_view(&self, bid: Bid) -> BidView {
        BidView {
            id: bid.id.0.to_string(),
            bidder_id: bid.bidder_id.to_string(),
            auction_id: bid.auction_id.0.to_string(),
            amount: bid.amount.0.to_string(),
            created_at: bid.created_at.0.to_string(),
        }
    }
}
