#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, symbol_short, Vec};

// Structure to hold a single budget entry
#[contracttype]
#[derive(Clone)]
pub struct BudgetEntry {
    pub category: String,
    pub amount: i128,
    pub description: String,
    pub timestamp: u64,
}

// Enum key to map entries
#[contracttype]
pub enum BudgetKey {
    Entry(Address, u64),   // (User, Entry ID)
    Counter(Address),      // Keeps count of entries per user
}

#[contract]
pub struct OpenBudgetTracker;

#[contractimpl]
impl OpenBudgetTracker {
    // Add a new budget entry
    pub fn add_entry(env: Env, user: Address, category: String, amount: i128, description: String) {
        let id_key = BudgetKey::Counter(user.clone());
        let mut entry_id = env.storage().instance().get(&id_key).unwrap_or(0);
        entry_id += 1;

        let entry = BudgetEntry {
            category,
            amount,
            description,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().instance().set(&BudgetKey::Entry(user.clone(), entry_id), &entry);
        env.storage().instance().set(&id_key, &entry_id);
    }

    // Get a specific budget entry
    pub fn get_entry(env: Env, user: Address, entry_id: u64) -> BudgetEntry {
        env.storage().instance().get(&BudgetKey::Entry(user, entry_id)).unwrap()
    }

    // Get total entries for a user
    pub fn get_entry_count(env: Env, user: Address) -> u64 {
        env.storage().instance().get(&BudgetKey::Counter(user)).unwrap_or(0)
    }
}
