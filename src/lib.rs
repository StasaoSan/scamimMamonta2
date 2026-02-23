#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Owner,
    Name,
    Symbol,
    Decimals,
    Balance(Address),
    TotalSupply,
}

#[contract]
pub struct Token;

fn read_balance(env: &Env, addr: &Address) -> i128 {
    env.storage()
        .instance()
        .get(&DataKey::Balance(addr.clone()))
        .unwrap_or(0_i128)
}

fn write_balance(env: &Env, addr: &Address, amount: i128) {
    env.storage()
        .instance()
        .set(&DataKey::Balance(addr.clone()), &amount);
}

fn require_positive(amount: i128) {
    if amount <= 0 {
        panic!("amount must be positive");
    }
}

#[contractimpl]
impl Token {
    /// Инициализация (вызывается один раз).
    /// owner — владелец (обычно деплоер),
    /// name/symbol/decimals — метаданные токена.
    pub fn init(env: Env, owner: Address, name: String, symbol: String, decimals: u32) {
        if env.storage().instance().has(&DataKey::Owner) {
            panic!("already initialized");
        }
        owner.require_auth();

        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &decimals);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
    }

    pub fn owner(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Owner).unwrap()
    }

    pub fn name(env: Env) -> String {
        env.storage().instance().get(&DataKey::Name).unwrap()
    }

    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&DataKey::Symbol).unwrap()
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Decimals).unwrap()
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0_i128)
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        read_balance(&env, &id)
    }

    /// Эмиссия: только owner
    pub fn mint(env: Env, to: Address, amount: i128) {
        require_positive(amount);

        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        owner.require_auth();

        let cur = read_balance(&env, &to);
        write_balance(&env, &to, cur + amount);

        let ts: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0_i128);
        env.storage().instance().set(&DataKey::TotalSupply, &(ts + amount));
    }

    /// Перевод: требуется подпись from
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        require_positive(amount);
        from.require_auth();

        let from_bal = read_balance(&env, &from);
        if from_bal < amount {
            panic!("insufficient balance");
        }
        write_balance(&env, &from, from_bal - amount);

        let to_bal = read_balance(&env, &to);
        write_balance(&env, &to, to_bal + amount);
    }

    /// Сжигание: подпись держателя
    pub fn burn(env: Env, from: Address, amount: i128) {
        require_positive(amount);
        from.require_auth();

        let bal = read_balance(&env, &from);
        if bal < amount {
            panic!("insufficient balance");
        }
        write_balance(&env, &from, bal - amount);

        let ts: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0_i128);
        env.storage().instance().set(&DataKey::TotalSupply, &(ts - amount));
    }
}