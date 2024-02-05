use crate::errors::AccountGenError::InvalidAccountData;
use crate::toml_update::AddressInfo;
use common::{deserialize_pubkey, serialize_pubkey};
use anchor_lang::prelude::Pubkey;
use anchor_lang::{AccountDeserialize, AccountSerialize};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    #[serde(
        deserialize_with = "deserialize_pubkey",
        serialize_with = "serialize_pubkey"
    )]
    pub pubkey: Pubkey,
    pub account: Account,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Account {
    pub lamports: u64,
    pub data: Vec<String>,
    pub owner: String,
    pub executable: bool,
    pub rentEpoch: u64,
    pub space: u64,
}

impl Root {
    pub fn from_string(source_account: &str) -> Result<Root, io::Error> {
        match serde_json::from_str::<Self>(source_account) {
            Ok(account) => Ok(account),
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{:?}", e))),
        }
    }

    pub fn get_account_data<T: AccountDeserialize>(&self) -> Result<T, Box<dyn std::error::Error>> {
        let account_data = &self.account.data[0];
        let data = BASE64_STANDARD.decode(account_data).unwrap();
        match T::try_deserialize(&mut data.as_slice()) {
            Ok(account_data) => Ok(account_data),
            Err(_e) => Err(Box::new(InvalidAccountData)),
        }
    }

    pub fn update_account_data<T: AccountSerialize>(
        &self,
        account_data: &T,
    ) -> Result<Root, io::Error> {
        let mut data = Vec::new();
        account_data.try_serialize(&mut data).unwrap();
        let encoded_data = BASE64_STANDARD.encode(&data);
        let mut account = self.account.clone();
        account.data = vec![encoded_data, "base64".to_string()];
        Ok(Root {
            pubkey: self.pubkey,
            account,
        })
    }

    pub fn update_pubkey(&self, pubkey: &Pubkey) -> Result<Root, io::Error> {
        Ok(Root {
            pubkey: *pubkey,
            account: self.account.clone(),
        })
    }

    fn write_account_file(&self, account_name: &str) -> io::Result<()> {
        let account_json = serde_json::to_string(self)?;
        fs::write(
            format!("./test-accounts/{}.json", account_name),
            account_json,
        )
    }
}

pub fn process_account<T: AccountDeserialize + AccountSerialize, F>(
    account_name: &str,
    new_address: Option<Pubkey>,
    data_update: F,
    accounts_to_update: &mut Vec<AddressInfo>,
    file_suffix: &str,
) -> Result<(T, Root), Box<dyn std::error::Error>>
where
    F: Fn(T) -> T,
{
    let account_file = get_account_file(account_name)?;
    let account = Root::from_string(&account_file)?;
    let new_address = new_address.unwrap_or(account.pubkey);
    accounts_to_update.push(AddressInfo {
        name: format!("{}{}", account_name, file_suffix),
        pubkey: new_address,
    });
    let mut account_data = account.get_account_data::<T>()?;
    account_data = data_update(account_data);
    let updated_account = account
        .update_account_data(&account_data)
        .unwrap()
        .update_pubkey(&new_address)
        .unwrap();
    updated_account.write_account_file(format!("{}{}", account_name, file_suffix).as_str())?;
    Ok((account_data, account))
}

fn get_account_file(account_name: &str) -> io::Result<String> {
    fs::read_to_string(format!(
        "./external-state/account-gen/test-accounts/{}.json",
        account_name
    ))
}
