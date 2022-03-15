use casper_contract::{contract_api::runtime, ext_ffi::casper_get_caller};
use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512, system::auction::RuntimeProvider, bytesrepr::ToBytes, gens::contract_arb};
use contract_utils::ContractContext;
use renvm_sig::keccak256;
use test_env::{Sender, TestContract, TestEnv};
use crate::erc721_instance::ERC721Instance;

fn deploy() -> (TestEnv, AccountHash, ERC721Instance) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = ERC721Instance::new(&env, "ERC721", Sender(owner),"sa".to_string(),"s".to_string());
    (env, owner, instance)
}

#[test]
fn test_deploy(){
   let (_, _, _) = deploy();
}