
use core::ptr::eq;

use crate::data::{self, Balances, Owners, OperatorApprovals, TokenApprovals};
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{ApiError, ContractPackageHash, Key, URef, U256, runtime_args,RuntimeArgs,bytesrepr::{Bytes, ToBytes}};
use contract_utils::{ContractContext, ContractStorage, set_key, get_key};

#[repr(u16)]
pub enum Error {
    //ERC721: balance query for the zero address
    QueryZeroAddress = 0,
    //ERC721: owner query for nonexistent token
    NonExistance = 1,
    // ERC721: approval to current owner
    ApprovalCurrentUser =  2,
    // approve caller is not owner nor approved for all
    NotOwnerNorAprrovedForAll = 3,
    //approve to caller
    ApproveCaller = 4,
    // transfer caller is not owner nor approved
    NotOwnerNorApproved = 5,
    // transfer from incorrect owner
    IncorrectOwner = 6,
    // transfer to the zero address
    TransferZeroAddress = 7,
    // transfer to non ERC721Receiver implementer
    NonErc721Receiver = 8,
    // mint to the zero address
    MintZeroAddress = 9,
    //token already minted
    AlreadyMinted = 10,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::User(error as u16)
    }
}

pub enum ERC721Event {
    Approval {
        owner: Key,
        spender: Key,
        value: U256,
    },
    Transfer {
        from: Key,
        to: Key,
        value: U256,
    },
    ApprovalForAll{
        owner: Key,
        operator: Key,
        approved: bool,
    }
}

impl ERC721Event {
    pub fn type_name(&self) -> String {
        match self {
            ERC721Event::Approval {
                owner: _,
                spender: _,
                value: _,
            } => "approve",
            ERC721Event::Transfer {
                from: _,
                to: _,
                value: _,
            } => "erc721_transfer",
            ERC721Event::ApprovalForAll {
                owner: _,
                operator: _,
                approved: _,
            } => "approval for all",
        }
        .to_string()
    }
}
pub trait ERC721<Storage: ContractStorage>:
    ContractContext<Storage>
{
    fn init(&mut self,name: String,symbol: String,contract_hash: Key, package_hash: ContractPackageHash) 
    {
        data::set_hash(contract_hash);
        data::set_package_hash(package_hash);
        data::set_name(name);
        data::set_symbol(symbol);
        data::Balances::init();
        //Balances::instance().set(&self.get_caller(), 1.into());
        //Owners::instance().set(&1.into(),self.get_caller());
        data::Owners::init();
        data::TokenApprovals::init();
    }



    fn balance_of(&self,owner:Key) -> U256
    {
        if!(owner != data::ZERO_ADDRESS()){
            runtime::revert(ApiError::from(Error::QueryZeroAddress));
        }
        Balances::instance().get(&owner)
        //1.into()
    }

    fn owner_of(&self,token_id:U256) -> Key
    {
        let owner:Key = Owners::instance().get(&token_id);
        if!(owner != data::ZERO_ADDRESS()){
            runtime::revert(ApiError::from(Error::NonExistance));
        }
        owner
    }

    fn name(&self) -> String
    {
        data::name()
    }

    fn symbol(&self) -> String
    {
        data::symbol()
    }
   
    fn token_uri(&self,token_id:U256) -> String
    {
        if!(self._exists(token_id)){
            runtime::revert(ApiError::from(Error::NonExistance));
        }
        let mut base_uri:String = self._base_uri();
        if base_uri.len() > 0 {
            base_uri.push_str(token_id.to_string().as_str());
            hex::encode(base_uri)
        }
        else{
            "".to_string()
        }
    }

    fn _base_uri(&self) -> String
    {
        return "".to_string();
    }

    fn approve(&mut self,to:Key,token_id:U256)
    {
        let owner:Key = self.owner_of(token_id);
        if!(to != owner){
            runtime::revert(ApiError::from(Error::ApprovalCurrentUser));
        }
        if!(self.get_caller() == owner ||self.is_approved_for_all(owner, self.get_caller())){
            runtime::revert(ApiError::from(Error::NotOwnerNorAprrovedForAll));
        }
        self._approve(to, token_id);
    }

    fn get_approved(&self, token_id:U256) -> Key
    {
            if!(self._exists(token_id)){
                runtime::revert(ApiError::from(Error::NonExistance));
            }
            TokenApprovals::instance().get(&token_id)
    }

    fn set_approved_for_all(&mut self,operator:Key,approved:bool)
    {
        self._set_approved_for_all(self.get_caller(), operator, approved)
    }

    fn is_approved_for_all(&self,owner:Key,operator:Key) -> bool
    {
        OperatorApprovals::instance().get(&owner, &operator)
    }

    fn transfer_from(&mut self,from:Key,to:Key,token_id:U256)
    {
        if!(self._is_approved_or_owner(self.get_caller(), token_id)){
            runtime::revert(ApiError::from(Error::NotOwnerNorApproved));
        }
        self._transfer(from, to, token_id);
    }

    fn safe_transfer_from(&mut self,from:Key,to:Key,token_id:U256)
    {
        self.safe_transfer_from_(from, to, token_id, Bytes::from("".as_bytes()));
    }
    //safe_transfer_from_ with data variable
    fn safe_transfer_from_(&mut self,from:Key,to:Key,token_id:U256,_data:Bytes)
    {
        if!(self._is_approved_or_owner(self.get_caller(), token_id)){
            runtime::revert(ApiError::from(Error::NotOwnerNorApproved));
        }
        self._safe_transfer(from,to,token_id,_data);
    }

    fn _safe_transfer(&mut self,from:Key,to:Key,token_id:U256,_data:Bytes)
    {
        self._transfer(from, to, token_id);
        self._check_on_erc721_received(from, to, token_id,_data);
    }

    fn _exists(&self,token_id:U256) -> bool{
        Owners::instance().get(&token_id) != data::ZERO_ADDRESS()
    }

    fn _is_approved_or_owner(&mut self,spender:Key,token_id:U256) -> bool 
    {
        if!(self._exists(token_id)){
            runtime::revert(ApiError::from(Error::NonExistance));
        }
        let owner:Key = self.owner_of(token_id);
        spender == owner || self.is_approved_for_all(owner,spender) || self.get_approved(token_id) == spender
    }

    fn _safe_mint(&mut self,to:Key,token_id:U256)
    {
        self._safe_mint_(to, token_id, Bytes::from("".as_bytes()));
    }

    fn _safe_mint_(&mut self,to:Key,token_id:U256,_data:Bytes)
    {
        self._mint(to,token_id);
        if!(self._check_on_erc721_received(data::ZERO_ADDRESS(), to, token_id, _data)){
            runtime::revert(ApiError::from(Error::NonErc721Receiver));
        }
    }
    fn mint(&mut self,to:Key,token_id:U256){
        self._mint(to,token_id);
    }
    fn _mint(&mut self,to:Key,token_id:U256)
    {
        if!(to!=data::ZERO_ADDRESS()){
            runtime::revert(ApiError::from(Error::MintZeroAddress));
        }
        Balances::instance().set(&to, Balances::instance().get(&to).checked_add(1.into()).unwrap_or_revert());
        Owners::instance().set(&token_id, to);
        self.erc721_emit(&ERC721Event::Transfer {
            from: data::ZERO_ADDRESS(),
            to: to,
            value: token_id,
        });
        self._after_token_transfer(data::ZERO_ADDRESS(), to, token_id);
    }

    fn burn(&mut self,token_id:U256){
        self._burn(token_id);
    }

    fn _burn(&mut self,token_id:U256)
    {
        let owner:Key = self.owner_of(token_id);
        self._before_token_transfer(owner, data::ZERO_ADDRESS(), token_id);
        self._approve(data::ZERO_ADDRESS(), token_id);
        Balances::instance().set(&owner, Balances::instance().get(&owner).checked_sub(1.into()).unwrap_or_revert());
       Owners::instance().set(&token_id,data::ZERO_ADDRESS());
        self.erc721_emit(&ERC721Event::Transfer {
            from: owner,
            to: data::ZERO_ADDRESS(),
            value: token_id,
        });
        self._after_token_transfer(owner, data::ZERO_ADDRESS(), token_id);
    }

    fn _transfer(&mut self,from:Key,to:Key,token_id:U256)
    {
        if!(self.owner_of(token_id) == from){
            runtime::revert(ApiError::from(Error::IncorrectOwner));
        }
        if!(to != data::ZERO_ADDRESS()){
            runtime::revert(ApiError::from(Error::TransferZeroAddress));
        }
        self._before_token_transfer(from, to, token_id);
        self._approve(data::ZERO_ADDRESS(), token_id);
        Balances::instance().set(&from, Balances::instance().get(&from).checked_sub(1.into()).unwrap_or_revert());
        Balances::instance().set(&to, Balances::instance().get(&to).checked_add(1.into()).unwrap_or_revert());
        Owners::instance().set(&token_id, to);
        self.erc721_emit(&ERC721Event::Transfer {
            from: from,
            to: to,
            value: token_id,
        });
        self._after_token_transfer(from, to, token_id);
    }

    fn _approve(&mut self,to:Key,token_id:U256)
    {
        TokenApprovals::instance().set(&token_id, to);
        self.erc721_emit(&ERC721Event::Approval {
            owner: self.owner_of(token_id),
            spender: to,
            value: token_id,
        });
    }

    fn _set_approved_for_all(&mut self,owner:Key,operator:Key,approved:bool)
    {
        if!(owner != operator){
            runtime::revert(ApiError::from(Error::ApproveCaller));
        }
        OperatorApprovals::instance().set(&owner,&operator, approved);
        self.erc721_emit(&ERC721Event::ApprovalForAll {
            owner: owner,
            operator: operator,
            approved: approved,
        });
    }

    fn _check_on_erc721_received(&mut self,from:Key,to:Key,token_id:U256,data:Bytes) -> bool
    {
        let v = to.to_formatted_string();
        let ch = v.chars().next().unwrap();
        //set_key("ch", ch.to_string());
        if eq(&ch, &'H'){
            let ret:Vec<u8> =runtime::call_contract(to.into_hash().unwrap_or_revert().into(), "on_erc721_received", runtime_args! {});
            return true;
        }
        else{
            return true;
        }
    }
    fn on_erc721_received(operator:Key,from:Key,token_id:U256,data:&[u8]) -> Vec<u8>{
        Vec::new()    
    }

    fn _after_token_transfer(&mut self, from:Key,to:Key,token_id:U256){}

    fn _before_token_transfer(&mut self, from:Key,to:Key,token_id:U256){}

    fn erc721_emit(&mut self, erc721_event: &ERC721Event) 
    {
        let mut events = Vec::new();
        let package = data::get_contract_package_hash();
        match erc721_event {
            ERC721Event::Approval {
                owner,
                spender,
                value,
            } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc721_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("spender", spender.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            ERC721Event::Transfer { from, to, value } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc721_event.type_name());
                event.insert("from", from.to_string());
                event.insert("to", to.to_string());
                event.insert("value", value.to_string());
                events.push(event);
            }
            ERC721Event::ApprovalForAll { owner, operator, approved } => {
                let mut event = BTreeMap::new();
                event.insert("contract_package_hash", package.to_string());
                event.insert("event_type", erc721_event.type_name());
                event.insert("owner", owner.to_string());
                event.insert("operator", operator.to_string());
                event.insert("approved", approved.to_string());
                events.push(event);
            }
        };
        for event in events {
            let _: URef = storage::new_uref(event);
        }
    }
}
