use blake2::digest::consts::U2;
use casper_contract::{contract_api::runtime, ext_ffi::casper_get_caller};
use casper_engine_test_support::AccountHash;
use casper_types::{runtime_args, ContractPackageHash, Key, RuntimeArgs, URef, U256, U512, system::auction::RuntimeProvider, bytesrepr::ToBytes, gens::contract_arb};
use contract_utils::ContractContext;
use renvm_sig::keccak256;
use test_env::{Sender, TestContract, TestEnv};
use crate::erc721_instance::ERC721Instance;

fn deploy() -> (TestEnv, AccountHash, TestContract,TestContract) {
    let env = TestEnv::new();
    let owner = env.next_user();
    let instance = ERC721Instance::new(&env, "ERC721", Sender(owner),"sa".to_string(),"s".to_string());
    let proxy = ERC721Instance::proxy(&env, "ERC721PROXY", Sender(owner),Key::Hash(instance.contract_hash()));
    (env, owner, instance,proxy)
}

// #[test]
// fn test_deploy(){
//    let (_, _, _,_) = deploy();
// }
#[test]
fn test_mint(){
    let(_,owner,instance,_) = deploy();
    let instance =ERC721Instance::contract_instance(instance);
    //let proxy =ERC721Instance::contract_instance(proxy);
    let token_id:U256 = 1.into();
    //proxy.approve(Sender(owner),Key::Account((owner)), token_id);
    let to:Key= Key::Account(owner);
    instance.mint(Sender(owner), to, token_id)
}
#[test]
fn test_burn(){
    let(_,owner,instance,_) = deploy();
    let instance =ERC721Instance::contract_instance(instance);
    let to:Key= Key::Account(owner);
    let token_id:U256 = 1.into();
    instance.mint(Sender(owner), to, token_id);
    instance.burn(Sender(owner),token_id)
}
#[test]
fn test_approve(){
    let(_,owner,instance,_) = deploy();
    let instance =ERC721Instance::contract_instance(instance);
    let to:Key= Key::Account(owner);
    let token_id:U256 = 1.into();
    instance.mint(Sender(owner), to, token_id);
    instance.approve(Sender(owner), Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000020000000000000000".into()).unwrap(), token_id);
}
#[test]
fn test_transfer_from(){
    let(_,owner,instance,_) = deploy();
    let instance =ERC721Instance::contract_instance(instance);
    let from:Key= Key::Account(owner);
    let to:Key= Key::Account(owner);
    let token_id:U256 = 1.into();
    instance.mint(Sender(owner), to, token_id);
    instance.transfer_from(Sender(owner), from,Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000020000000000000000".into()).unwrap(), token_id);
}
#[test]
fn test_safe_transfer_from(){
    let(_,owner,instance,_) = deploy();
    let instance =ERC721Instance::contract_instance(instance);
    let from:Key= Key::Account(owner);
    let to:Key= Key::Account(owner);
    let token_id:U256 = 1.into();
    instance.mint(Sender(owner), to, token_id);
    instance.safe_transfer_from(Sender(owner), from,Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000020000000000000000".into()).unwrap(), token_id);
}
#[test]
fn test_balance_of(){
    let(_,owner,instance,proxy) = deploy();
    let from:Key= Key::Account(owner);
    let proxy =ERC721Instance::contract_instance(proxy);
    proxy.balance_of(Sender(owner), from);
    
    //let to:Key= Key::Account(owner);
    //let token_id:U256 = 1.into();
    //instance.mint(Sender(owner), to, token_id);
    //instance.safe_transfer_from(Sender(owner), from,Key::from_formatted_str("hash-0000000000000000000000000000000000000000000000020000000000000000".into()).unwrap(), token_id);
}