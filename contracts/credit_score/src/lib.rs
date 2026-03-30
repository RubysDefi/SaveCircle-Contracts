#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol,
};

// ── Storage Keys ──────────────────────────────────────────────────────────────

const SCORE: Symbol = symbol_short!("SCORE");

// ── Constants ─────────────────────────────────────────────────────────────────

const MAX_SCORE: i32 = 1000;
const MIN_SCORE: i32 = 0;
const POINTS_PER_CONTRIBUTION: i32 = 10;
const POINTS_PER_LOAN_REPAID: i32 = 50;
const PENALTY_DEFAULT: i32 = 150;

// ── Data Types ────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug)]
pub struct CreditScore {
    pub owner: Address,
    pub score: i32,
    pub total_contributions: u32,
    pub total_loans_repaid: u32,
    pub total_defaults: u32,
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct CreditScoreContract;

#[contractimpl]
impl CreditScoreContract {
    /// Initialize a credit score record for a new user
    pub fn initialize(env: Env, owner: Address) {
        owner.require_auth();
        let score = CreditScore {
            owner: owner.clone(),
            score: 0,
            total_contributions: 0,
            total_loans_repaid: 0,
            total_defaults: 0,
        };
        env.storage().instance().set(&SCORE, &score);
    }

    /// Called when a member makes a successful contribution
    pub fn on_contribution(env: Env, caller: Address) {
        caller.require_auth();
        let mut score: CreditScore = env.storage().instance().get(&SCORE).unwrap();
        score.score = (score.score + POINTS_PER_CONTRIBUTION).min(MAX_SCORE);
        score.total_contributions += 1;
        env.storage().instance().set(&SCORE, &score);
    }

    /// Called when a member repays a loan
    pub fn on_loan_repaid(env: Env, caller: Address) {
        caller.require_auth();
        let mut score: CreditScore = env.storage().instance().get(&SCORE).unwrap();
        score.score = (score.score + POINTS_PER_LOAN_REPAID).min(MAX_SCORE);
        score.total_loans_repaid += 1;
        env.storage().instance().set(&SCORE, &score);
    }

    /// Called when a member defaults on a loan
    /// TODO: add multi-sig or oracle verification before penalizing
    pub fn on_loan_defaulted(env: Env, caller: Address) {
        caller.require_auth();
        let mut score: CreditScore = env.storage().instance().get(&SCORE).unwrap();
        score.score = (score.score - PENALTY_DEFAULT).max(MIN_SCORE);
        score.total_defaults += 1;
        env.storage().instance().set(&SCORE, &score);
    }

    /// Get the current credit score for a user
    pub fn get_score(env: Env) -> CreditScore {
        env.storage().instance().get(&SCORE).unwrap()
    }

    /// Check if a user meets the minimum score threshold for a loan
    pub fn is_eligible_for_loan(env: Env, min_score: i32) -> bool {
        let score: CreditScore = env.storage().instance().get(&SCORE).unwrap();
        score.score >= min_score
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_score_increases_on_contribution() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, CreditScoreContract);
        let client = CreditScoreContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        client.initialize(&user);
        client.on_contribution(&user);
        client.on_contribution(&user);

        let score = client.get_score();
        assert_eq!(score.score, 20);
        assert_eq!(score.total_contributions, 2);
    }

    #[test]
    fn test_score_increases_on_loan_repaid() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, CreditScoreContract);
        let client = CreditScoreContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        client.initialize(&user);
        client.on_loan_repaid(&user);

        let score = client.get_score();
        assert_eq!(score.score, 50);
    }

    #[test]
    fn test_score_decreases_on_default() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, CreditScoreContract);
        let client = CreditScoreContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        client.initialize(&user);
        client.on_contribution(&user); // +10
        client.on_loan_defaulted(&user); // -150, floor at 0

        let score = client.get_score();
        assert_eq!(score.score, 0);
    }

    #[test]
    fn test_loan_eligibility() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, CreditScoreContract);
        let client = CreditScoreContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);
        client.initialize(&user);

        assert!(!client.is_eligible_for_loan(&300));

        for _ in 0..30 {
            client.on_contribution(&user);
        }

        assert!(client.is_eligible_for_loan(&300));
    }
}
