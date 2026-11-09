//! Rumah & Resto — NEP-245 multi-token ledger for the in-game economy.
//!
//! The game keeps a market full of wallet-enabled NPCs and player stalls. This contract is the
//! ledger underneath it: every item (coin, scrap, shard, cell) is a fungible multi-token, and the
//! transfer trail is readable so the market panel can show what actually settled on chain.
//!
//! Deliberately small: mint, transfer, balances, supply, recent trail. No approvals, no royalties,
//! no enumeration tricks — the game does not need them and every extra path is attack surface.

use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near, AccountId, NearToken, PanicOnDefault};
use std::collections::{BTreeMap, BTreeSet};

const LOG_LIMIT: usize = 60;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Event {
    pub at: u64,
    pub kind: String,
    pub to: AccountId,
    pub token: String,
    pub amount: U128,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    supply: BTreeMap<String, u128>,
    balances: BTreeMap<(AccountId, String), u128>,
    holders: BTreeMap<String, BTreeSet<AccountId>>,
    log: Vec<Event>,
}

#[near]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            owner: env::predecessor_account_id(),
            supply: BTreeMap::new(),
            balances: BTreeMap::new(),
            holders: BTreeMap::new(),
            log: Vec::new(),
        }
    }

    // ------------------------------------------------------------------ minting

    /// Mint tokens to a receiver. Owner only: the market agents are simulated, so the game's
    /// economy mints through the owner key rather than letting anyone print money.
    pub fn mt_mint(&mut self, token_id: String, amount: U128, receiver_id: Option<AccountId>) -> U128 {
        self.assert_owner();
        let to = receiver_id.unwrap_or_else(env::predecessor_account_id);
        let amount = amount.0;
        assert!(amount > 0, "amount must be positive");

        *self.supply.entry(token_id.clone()).or_insert(0) += amount;
        *self.balances.entry((to.clone(), token_id.clone())).or_insert(0) += amount;
        self.holders.entry(token_id.clone()).or_default().insert(to.clone());
        self.push_event("mt_mint", to, token_id.clone(), amount);
        U128(amount)
    }

    // ---------------------------------------------------------------- transfers

    pub fn mt_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: String,
        amount: U128,
        #[allow(unused_variables)] approval_id: Option<u64>,
        #[allow(unused_variables)] memo: Option<String>,
    ) {
        let from = env::predecessor_account_id();
        self.move_tokens(from, receiver_id, token_id, amount.0);
    }

    pub fn mt_batch_transfer(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<String>,
        amounts: Vec<U128>,
        #[allow(unused_variables)] approval_ids: Option<Vec<Option<u64>>>,
        #[allow(unused_variables)] memo: Option<String>,
    ) {
        assert_eq!(token_ids.len(), amounts.len(), "token_ids and amounts differ in length");
        let from = env::predecessor_account_id();
        for (token, amount) in token_ids.into_iter().zip(amounts) {
            self.move_tokens(from.clone(), receiver_id.clone(), token, amount.0);
        }
    }

    /// Settle a trade between two simulated market agents. The game backend signs this with the
    /// owner key: NPC wallets are economy actors, not key holders, so they cannot sign themselves.
    pub fn settle(&mut self, from: AccountId, to: AccountId, token_id: String, amount: U128) {
        self.assert_owner();
        self.move_tokens(from, to, token_id, amount.0);
    }

    fn move_tokens(&mut self, from: AccountId, to: AccountId, token_id: String, amount: u128) {
        assert!(amount > 0, "amount must be positive");
        assert_ne!(from, to, "cannot transfer to self");
        let key = (from.clone(), token_id.clone());
        let held = self.balances.get(&key).copied().unwrap_or(0);
        assert!(held >= amount, "not enough balance");

        let left = held - amount;
        if left == 0 {
            self.balances.remove(&key);
            if let Some(set) = self.holders.get_mut(&token_id) {
                set.remove(&from);
            }
        } else {
            self.balances.insert(key, left);
        }

        *self.balances.entry((to.clone(), token_id.clone())).or_insert(0) += amount;
        self.holders.entry(token_id.clone()).or_default().insert(to.clone());
        self.push_event("mt_transfer", to, token_id, amount);
    }

    // ------------------------------------------------------------------- views

    pub fn mt_balance_of(&self, account_id: AccountId, token_id: String) -> U128 {
        U128(self.balances.get(&(account_id, token_id)).copied().unwrap_or(0))
    }

    pub fn mt_batch_balance_of(&self, account_id: AccountId, token_ids: Vec<String>) -> Vec<U128> {
        token_ids
            .into_iter()
            .map(|t| U128(self.balances.get(&(account_id.clone(), t)).copied().unwrap_or(0)))
            .collect()
    }

    pub fn mt_tokens_for_owner(&self, account_id: AccountId) -> Vec<String> {
        let mut out: Vec<String> = self
            .balances
            .iter()
            .filter(|((who, _), amount)| who == &account_id && **amount > 0)
            .map(|((_, token), _)| token.clone())
            .collect();
        out.sort();
        out
    }

    pub fn mt_supply(&self, token_id: String) -> U128 {
        U128(self.supply.get(&token_id).copied().unwrap_or(0))
    }

    /// The most recent ledger entries, newest last. The market panel renders this.
    pub fn recent(&self, n: u32) -> Vec<Event> {
        let take = (n as usize).min(self.log.len());
        self.log[self.log.len() - take..].to_vec()
    }

    pub fn totals(&self) -> BTreeMap<String, U128> {
        self.supply.iter().map(|(k, v)| (k.clone(), U128(*v))).collect()
    }

    pub fn owner(&self) -> AccountId {
        self.owner.clone()
    }

    // ------------------------------------------------------------------ helpers

    fn assert_owner(&self) {
        assert_eq!(env::predecessor_account_id(), self.owner, "only the owner can mint");
    }

    fn push_event(&mut self, kind: &str, to: AccountId, token: String, amount: u128) {
        self.log.push(Event {
            at: env::block_timestamp_ms(),
            kind: kind.to_string(),
            to,
            token,
            amount: U128(amount),
        });
        if self.log.len() > LOG_LIMIT {
            self.log.remove(0);
        }
    }

    /// Accept a top-up so the account keeps storage headroom.
    #[payable]
    pub fn poke(&mut self) {
        let _ = NearToken::from_yoctonear(env::attached_deposit().as_yoctonear());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn ctx(who: AccountId) -> VMContextBuilder {
        let mut b = VMContextBuilder::new();
        b.predecessor_account_id(who);
        b
    }

    #[test]
    fn owner_mints_and_balance_shows() {
        testing_env!(ctx(accounts(0)).build());
        let mut c = Contract::new();
        c.mt_mint("coin".into(), U128(100), Some(accounts(1)));
        assert_eq!(c.mt_balance_of(accounts(1), "coin".into()).0, 100);
        assert_eq!(c.mt_supply("coin".into()).0, 100);
        assert_eq!(c.mt_tokens_for_owner(accounts(1)), vec!["coin".to_string()]);
    }

    #[test]
    fn transfer_moves_balance_between_wallets() {
        testing_env!(ctx(accounts(0)).build());
        let mut c = Contract::new();
        c.mt_mint("scrap".into(), U128(50), Some(accounts(1)));
        testing_env!(ctx(accounts(1)).build());
        c.mt_transfer(accounts(2), "scrap".into(), U128(20), None, None);
        assert_eq!(c.mt_balance_of(accounts(1), "scrap".into()).0, 30);
        assert_eq!(c.mt_balance_of(accounts(2), "scrap".into()).0, 20);
        assert_eq!(c.recent(5).len(), 2);
    }

    #[test]
    #[should_panic(expected = "not enough balance")]
    fn cannot_spend_more_than_held() {
        testing_env!(ctx(accounts(0)).build());
        let mut c = Contract::new();
        c.mt_mint("cell".into(), U128(5), Some(accounts(1)));
        testing_env!(ctx(accounts(1)).build());
        c.mt_transfer(accounts(2), "cell".into(), U128(9), None, None);
    }

    #[test]
    #[should_panic(expected = "only the owner can mint")]
    fn outsiders_cannot_mint() {
        let mut c = Contract::new();
        testing_env!(ctx(accounts(3)).build());
        c.mt_mint("coin".into(), U128(1000), Some(accounts(3)));
    }

    #[test]
    fn batch_transfer_and_batch_balance() {
        testing_env!(ctx(accounts(0)).build());
        let mut c = Contract::new();
        c.mt_mint("coin".into(), U128(10), Some(accounts(1)));
        c.mt_mint("shard".into(), U128(3), Some(accounts(1)));
        testing_env!(ctx(accounts(1)).build());
        c.mt_batch_transfer(accounts(2), vec!["coin".into(), "shard".into()],
                            vec![U128(4), U128(1)], None, None);
        let got = c.mt_batch_balance_of(accounts(2), vec!["coin".into(), "shard".into()]);
        assert_eq!(got[0].0, 4);
        assert_eq!(got[1].0, 1);
    }

    #[test]
    fn owner_settles_agent_trades() {
        testing_env!(ctx(accounts(0)).build());
        let mut c = Contract::new();
        c.mt_mint("wortel".into(), U128(20), Some("sari.testnet".parse().unwrap()));
        c.settle("sari.testnet".parse().unwrap(), "rumah-resto.testnet".parse().unwrap(),
                 "wortel".into(), U128(6));
        assert_eq!(c.mt_balance_of("sari.testnet".parse().unwrap(), "wortel".into()).0, 14);
        assert_eq!(c.mt_balance_of("rumah-resto.testnet".parse().unwrap(), "wortel".into()).0, 6);
    }

    #[test]
    #[should_panic(expected = "only the owner can mint")]
    fn agent_settle_needs_owner() {
        testing_env!(ctx(accounts(0)).build());
        let mut c = Contract::new();
        testing_env!(ctx("sari.testnet".parse().unwrap()).build());
        c.settle("sari.testnet".parse().unwrap(), "rumah-resto.testnet".parse().unwrap(),
                 "wortel".into(), U128(1));
    }

    #[test]
    fn log_is_bounded() {
        testing_env!(ctx(accounts(0)).build());
        let mut c = Contract::new();
        for i in 0..(LOG_LIMIT + 10) {
            c.mt_mint("coin".into(), U128(1), Some(accounts(1)));
            let _ = i;
        }
        assert_eq!(c.recent(1000).len(), LOG_LIMIT);
    }
}
