use anchor_lang::{AccountDeserialize, AccountSerialize, AnchorDeserialize, AnchorSerialize};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::io;

use anchor_lang::prelude::Pubkey;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    pub pubkey: String,
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub lamports: u64,
    pub data: Vec<String>,
    pub owner: String,
    pub executable: bool,
    pub rentEpoch: u64,
    pub space: u64,
}

impl Root {
    pub fn from_string(source_account: &String) -> Result<Root, io::Error> {
        match serde_json::from_str::<Self>(&source_account) {
            Ok(account) => Ok(account),
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))),
        }
    }

    pub fn get_account_data<T: AccountDeserialize>(&self) -> Result<T, io::Error> {
        let account_data = &self.account.data[0];
        let data = BASE64_STANDARD.decode(account_data).unwrap();
        T::try_deserialize(&mut data.as_slice())
            .or_else(|e| return Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))))
    }

    pub fn update_account_data<T: AccountSerialize>(
        &self,
        account_data: &T,
    ) -> Result<Root, io::Error> {
        println!("account_data before: {:?}", self.account.data);
        let mut data = Vec::new();
        account_data.try_serialize(&mut data).unwrap();
        let encoded_data = BASE64_STANDARD.encode(&data);
        let mut account = self.account.clone();
        account.data = vec![encoded_data];
        println!("account_data after: {:?}", &account.data);
        Ok(Root {
            pubkey: self.pubkey.clone(),
            account,
        })
    }

    pub fn update_pubkey(&self, pubkey: &Pubkey) -> Result<Root, io::Error> {
        Ok(Root {
            pubkey: pubkey.to_string(),
            account: self.account.clone(),
        })
    }
}
