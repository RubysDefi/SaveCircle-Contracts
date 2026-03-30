#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Vec, Symbol,
};

// ── Storage Keys ──────────────────────────────────────────────────────────────

const CIRCLE: Symbol = symbol_short!("CIRCLE");

// ── Data Types ────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum CircleStatus {
    Pending,
    Active,
    Completed,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct CircleState {
    pub admin: Address,
    pub members: Vec<Address>,
    pub contribution_amount: i128,
    pub current_cycle: u32,
    pub payout_order: Vec<Address>,
    pub status: CircleStatus,
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct SavingsCircleContract;

#[contractimpl]
impl SavingsCircleContract {
    /// Initialize a new savings circle
    pub fn create_circle(
        env: Env,
        admin: Address,
        members: Vec<Address>,
        contribution_amount: i128,
        payout_order: Vec<Address>,
    ) {
        admin.require_auth();

        let state = CircleState {
            admin,
            members,
            contribution_amount,
            current_cycle: 0,
            payout_order,
            status: CircleStatus::Pending,
        };

        env.storage().instance().set(&CIRCLE, &state);
    }

    /// Activate the circle — only admin can call
    pub fn activate(env: Env, admin: Address) {
        admin.require_auth();
        let mut state: CircleState = env.storage().instance().get(&CIRCLE).unwrap();
        assert!(state.admin == admin, "Only admin can activate");
        state.status = CircleStatus::Active;
        env.storage().instance().set(&CIRCLE, &state);
    }

    /// Record a contribution from a member
    /// TODO: Verify the actual token transfer and amount
    pub fn contribute(env: Env, member: Address) {
        member.require_auth();
        let state: CircleState = env.storage().instance().get(&CIRCLE).unwrap();
        assert!(state.status == CircleStatus::Active, "Circle is not active");
        assert!(state.members.contains(&member), "Not a circle member");
        // TODO: integrate with Stellar token contract to pull contribution_amount
        // from member's account into the circle's vault account
    }

    /// Trigger payout to the next member in the rotation
    /// TODO: implement multi-sig approval before payout is released
    pub fn trigger_payout(env: Env, admin: Address) {
        admin.require_auth();
        let mut state: CircleState = env.storage().instance().get(&CIRCLE).unwrap();
        assert!(state.admin == admin, "Only admin can trigger payout");
        assert!(state.status == CircleStatus::Active, "Circle is not active");

        let cycle = state.current_cycle as usize;
        assert!(cycle < state.payout_order.len() as usize, "All cycles completed");

        // TODO: transfer accumulated funds to payout_order[cycle] via token contract

        state.current_cycle += 1;

        if state.current_cycle as usize >= state.payout_order.len() as usize {
            state.status = CircleStatus::Completed;
        }

        env.storage().instance().set(&CIRCLE, &state);
    }

    /// Get current circle state
    pub fn get_circle(env: Env) -> CircleState {
        env.storage().instance().get(&CIRCLE).unwrap()
    }

    /// Get the address of the next payout recipient
    pub fn get_next_recipient(env: Env) -> Address {
        let state: CircleState = env.storage().instance().get(&CIRCLE).unwrap();
        let cycle = state.current_cycle as usize;
        assert!(cycle < state.payout_order.len() as usize, "All cycles completed");
        state.payout_order.get(cycle as u32).unwrap()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::Env;

    #[test]
    fn test_create_and_activate_circle() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, SavingsCircleContract);
        let client = SavingsCircleContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let member1 = Address::generate(&env);
        let member2 = Address::generate(&env);

        let members = Vec::from_array(&env, [admin.clone(), member1.clone(), member2.clone()]);
        let payout_order = Vec::from_array(&env, [member1.clone(), member2.clone(), admin.clone()]);

        client.create_circle(&admin, &members, &1000, &payout_order);

        let state = client.get_circle();
        assert_eq!(state.status, CircleStatus::Pending);
        assert_eq!(state.current_cycle, 0);

        client.activate(&admin);
        let state = client.get_circle();
        assert_eq!(state.status, CircleStatus::Active);
    }

    #[test]
    fn test_get_next_recipient() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, SavingsCircleContract);
        let client = SavingsCircleContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let member1 = Address::generate(&env);

        let members = Vec::from_array(&env, [admin.clone(), member1.clone()]);
        let payout_order = Vec::from_array(&env, [member1.clone(), admin.clone()]);

        client.create_circle(&admin, &members, &500, &payout_order);
        client.activate(&admin);

        let next = client.get_next_recipient();
        assert_eq!(next, member1);
    }
}
