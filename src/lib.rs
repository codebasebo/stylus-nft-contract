#![cfg_attr(not(feature = "export-abi"), no_std)]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use stylus_sdk::{msg, alloy_primitives::{Address, U256}, prelude::*};

// Define the Occasion struct
#[derive(Debug, Clone)]
pub struct Occasion {
    pub id: U256,
    pub name: String,
    pub cost: U256,
    pub tickets: U256,
    pub max_tickets: U256,
    pub date: String,
    pub time: String,
    pub location: String,
}

// Define the contract
sol_storage! {
    #[entrypoint]
    pub struct TokenMaster {
        #[borrow]
        address owner;
        uint256 total_occasions,
        uint256 total_supply,
        mapping(uint256 => Occasion) occasions,
        mapping(uint256 => mapping(address => bool)) has_bought,
        mapping(uint256 => mapping(uint256 => address)) seat_taken,
        mapping(uint256 => uint256[])seats_taken,
    }
}

// Implement the contract
impl TokenMaster {
    pub fn new(&mut self, name: String, symbol: String) {
        // Initialize the owner
        self.owner.set(msg::sender());

        // Initialize the total occasions and total supply
        self.total_occasions.set(U256::from(0));
        self.total_supply.set(U256::from(0));
    }

    pub fn list(&mut self, name: String, cost: U256, max_tickets: U256, date: String, time: String, location: String) {
        // Require that only the owner can list a new occasion
        assert!(msg::sender() == self.owner.get());

        // Increment the total occasions
        self.total_occasions.set(self.total_occasions.get() + U256::from(1));

        // Create a new occasion
        let occasion = Occasion {
            id: self.total_occasions.get(),
            name,
            cost,
            tickets: max_tickets,
            max_tickets,
            date,
            time,
            location,
        };

        // Store the occasion
        self.occasions.insert(self.total_occasions.get(), occasion);
    }

    pub fn mint(&mut self, id: U256, seat: U256) {
        // Require that the id is not 0 or less than the total occasions
        assert!(id != U256::from(0));
        assert!(id <= self.total_occasions.get());

        // Require that the ETH sent is greater than the cost
        assert!(msg::value() >= self.occasions.get(id).cost);

        // Require that the seat is not taken and the seat exists
        assert!(self.seat_taken.get(id).get(seat) == Address::default());
        assert!(seat <= self.occasions.get(id).max_tickets);

        // Update the ticket count
        self.occasions.get_mut(id).tickets -= U256::from(1);

        // Update the buying status
        self.has_bought.get_mut(id).insert(msg::sender(), true);

        // Assign the seat
        self.seat_taken.get_mut(id).insert(seat, msg::sender());

        // Update the seats taken
        self.seats_taken.get_mut(id).push(seat);

        // Increment the total supply
        self.total_supply.set(self.total_supply.get() + U256::from(1));

        // Mint the token
        self.mint_token(msg::sender(), self.total_supply.get());
    }

    pub fn get_occasion(&self, id: U256) -> Occasion {
        self.occasions.get(id).clone()
    }

    pub fn get_seats_taken(&self, id: U256) -> Vec<U256> {
        self.seats_taken.get(id).clone()
    }

    pub fn withdraw(&mut self) {
        // Require that only the owner can withdraw ETH
        assert!(msg::sender() == self.owner.get());

        // Withdraw ETH
        self.owner.get().transfer(self.balance());
    }

    fn mint_token(&mut self, to: Address, id: U256) {
        // Implement token minting logic here
    }
}