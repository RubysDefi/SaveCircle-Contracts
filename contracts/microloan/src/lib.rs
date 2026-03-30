#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol,
};

// ── Storage Keys ──────────────────────────────────────────────────────────────

const LOAN: Symbol = symbol_short!("LOAN");

// ── Data Types ────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum LoanStatus {
    Active,
    Repaid,
    Defaulted,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Loan {
    pub borrower: Address,
    pub amount: i128,
    pub interest_rate_bps: u32, // basis points e.g. 250 = 2.5%
    pub due_ledger: u32,
    pub status: LoanStatus,
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct MicroloanContract;

#[contractimpl]
impl MicroloanContract {
    /// Disburse a microloan to a borrower
    /// TODO: verify credit score contract before disbursing
    /// TODO: integrate with Stellar token contract to transfer funds
    pub fn disburse(
        env: Env,
        admin: Address,
        borrower: Address,
        amount: i128,
        interest_rate_bps: u32,
        repayment_ledgers: u32,
    ) {
        admin.require_auth();

        let loan = Loan {
            borrower: borrower.clone(),
            amount,
            interest_rate_bps,
            due_ledger: env.ledger().sequence() + repayment_ledgers,
            status: LoanStatus::Active,
        };

        env.storage().instance().set(&LOAN, &loan);

        // TODO: call token contract to transfer `amount` to borrower
    }

    /// Record a loan repayment
    /// TODO: verify on-chain token transfer before marking repaid
    pub fn repay(env: Env, borrower: Address) {
        borrower.require_auth();
        let mut loan: Loan = env.storage().instance().get(&LOAN).unwrap();
        assert!(loan.borrower == borrower, "Not the loan borrower");
        assert!(loan.status == LoanStatus::Active, "Loan is not active");

        loan.status = LoanStatus::Repaid;
        env.storage().instance().set(&LOAN, &loan);

        // TODO: call credit score contract on_loan_repaid
    }

    /// Mark a loan as defaulted if past due ledger
    /// TODO: add oracle or admin verification before marking default
    pub fn mark_defaulted(env: Env, admin: Address) {
        admin.require_auth();
        let mut loan: Loan = env.storage().instance().get(&LOAN).unwrap();
        assert!(loan.status == LoanStatus::Active, "Loan is not active");
        assert!(
            env.ledger().sequence() > loan.due_ledger,
            "Loan is not yet past due"
        );

        loan.status = LoanStatus::Defaulted;
        env.storage().instance().set(&LOAN, &loan);

        // TODO: call credit score contract on_loan_defaulted
    }

    /// Calculate total repayment amount including interest
    pub fn repayment_amount(env: Env) -> i128 {
        let loan: Loan = env.storage().instance().get(&LOAN).unwrap();
        let interest = (loan.amount * loan.interest_rate_bps as i128) / 10_000;
        loan.amount + interest
    }

    /// Get current loan state
    pub fn get_loan(env: Env) -> Loan {
        env.storage().instance().get(&LOAN).unwrap()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_disburse_and_repay() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, MicroloanContract);
        let client = MicroloanContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let borrower = Address::generate(&env);

        client.disburse(&admin, &borrower, &1000, &250, &1000);

        let loan = client.get_loan();
        assert_eq!(loan.status, LoanStatus::Active);
        assert_eq!(loan.amount, 1000);

        client.repay(&borrower);

        let loan = client.get_loan();
        assert_eq!(loan.status, LoanStatus::Repaid);
    }

    #[test]
    fn test_repayment_amount_with_interest() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, MicroloanContract);
        let client = MicroloanContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let borrower = Address::generate(&env);

        // 1000 at 2.5% interest = 1025
        client.disburse(&admin, &borrower, &1000, &250, &1000);
        assert_eq!(client.repayment_amount(), 1025);
    }
}
