use core::convert::TryInto;

use alloc::string::{String, ToString};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, CLTyped, ContractPackageHash, Key, U256};
use contract_utils::{get_key, set_key};
use contract_utils::Dict;

pub const SELF_CONTRACT_HASH: &str = "self_contract_hash";
pub const SELF_PACKAGE_HASH: &str = "self_package_hash";
pub const RESULT: &str = "result";
pub const OWNERS: &str = "owners";
pub const NAME:&str = "name";
pub const SYMBOL:&str = "symbol";
pub struct Owners {
    dict: Dict,
}

impl Owners {
    pub fn instance() -> Owners {
        Owners {
            dict: Dict::instance(OWNERS),
        }
    }

    pub fn init() {
        Dict::init(OWNERS)
    }

    pub fn get(&self, owner: &U256) -> Key {
        self.dict.get(owner.to_string().as_str()).unwrap_or_revert()
    }

    pub fn set(&self, owner: &U256, value: Key) {
        self.dict.set(owner.to_string().as_str(), value)
    }
}
pub const BALACNES: &str = "balances";
pub struct Balances {
    dict: Dict,
}

impl Balances {
    pub fn instance() -> Balances{
        Balances {
            dict: Dict::instance(BALACNES),
        }
    }

    pub fn init() {
        Dict::init(BALACNES)
    }

    pub fn get(&self, owner: &Key) -> U256 {
        self.dict.get_by_key(owner).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, value: U256) {
        self.dict.set_by_key(owner, value)
    }
}
pub const TOEKEN_APPROVALS: &str = "token_approvals";
pub struct TokenApprovals {
    dict: Dict,
}

impl TokenApprovals {
    pub fn instance() -> TokenApprovals{
        TokenApprovals {
            dict: Dict::instance(TOEKEN_APPROVALS),
        }
    }

    pub fn init() {
        Dict::init(TOEKEN_APPROVALS)
    }

    pub fn get(&self, owner: &U256) -> Key {
        self.dict.get(owner.to_string().as_str()).unwrap_or_revert()
    }

    pub fn set(&self, owner: &U256, value: Key) {
        self.dict.set(owner.to_string().as_str(), value)
    }
}
pub const Operator_Approvals: &str = "operator_approvals";
pub struct OperatorApprovals {
    dict: Dict,
}

impl OperatorApprovals {
    pub fn instance() -> OperatorApprovals{
        OperatorApprovals {
            dict: Dict::instance(Operator_Approvals),
        }
    }

    pub fn init() {
        Dict::init(Operator_Approvals)
    }

    pub fn get(&self, owner: &Key, operator: &Key) -> bool {
        self.dict.get_by_keys((owner, operator)).unwrap_or_default()
    }

    pub fn set(&self, owner: &Key, operator: &Key, value: bool) {
        self.dict.set_by_keys((owner, operator), value);
    }
}
pub fn set_result<T: ToBytes + CLTyped>(value: T) {
    match runtime::get_key(RESULT) {
        Some(key) => {
            let key_ref = key.try_into().unwrap_or_revert();
            storage::write(key_ref, value);
        }
        None => {
            let key = storage::new_uref(value).into();
            runtime::put_key(RESULT, key);
        }
    }
}
pub fn name() -> String {
    get_key(NAME).unwrap_or_revert()
}

pub fn set_name(name: String) {
    set_key(NAME, name);
}

pub fn symbol() -> String {
    get_key(SYMBOL).unwrap_or_revert()
}

pub fn set_symbol(symbol: String) {
    set_key(SYMBOL, symbol);
}
pub fn set_hash(contract_hash: Key) {
    set_key(SELF_CONTRACT_HASH, contract_hash);
}

pub fn get_hash() -> Key {
    get_key(SELF_CONTRACT_HASH).unwrap_or_revert()
}

pub fn set_package_hash(package_hash: ContractPackageHash) {
    set_key(SELF_PACKAGE_HASH, package_hash);
}

pub fn get_contract_package_hash() -> ContractPackageHash {
    get_key(SELF_PACKAGE_HASH).unwrap_or_revert()
}
