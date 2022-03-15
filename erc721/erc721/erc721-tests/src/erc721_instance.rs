use casper_engine_test_support::{TestContext, AccountHash};
use casper_types::{
    bytesrepr::FromBytes, runtime_args, CLTyped, ContractPackageHash, Key, RuntimeArgs, URef, U256,
};
use test_env::{Sender, TestContract, TestEnv};

pub struct ERC721Instance(TestContract);

impl ERC721Instance {

    pub fn new(env: &TestEnv, contract_name: &str, sender: Sender,name:String,symbol:String) -> ERC721Instance {
        ERC721Instance(TestContract::new(
            env,
            "erc721.wasm",
            contract_name,
            sender,
            runtime_args! {
                "name" => name,
                "symbol" => symbol,
            },
            0,
        ))
    }
    
    // Initialize Function
    // pub fn initialize(
    //     &self,
    //     sender: Sender,
    //     token_id: Vec<U256>,
    //     token_address: Key,
    //     token_owner: Key,
    //     floor_asked: U256,
    //     total_asked: U256,
    //     payment_time: U256,
    //     payment_rate: U256,
    // ) {
    //     self.0.call_contract(
    //         sender,
    //         "initialize",
    //         runtime_args! {
    //             "token_id" => token_id,
    //             "token_address" => token_address,
    //             "token_owner" => token_owner,
    //             "floor_asked" => floor_asked,
    //             "total_asked" => total_asked,
    //             "payment_time" => payment_time,
    //             "payment_rate" => payment_rate
    //         },
    //         0
    //     );
    // }
    // Result methods
    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn package_hash(&self) -> ContractPackageHash {
        self.0.query_named_key("self_package_hash".to_string())
    }
}
