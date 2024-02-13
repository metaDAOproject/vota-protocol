use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use toml::{Table, Value};

pub struct AddressInfo {
    pub name: String,
    pub pubkey: Pubkey,
}

pub fn update_anchor_toml(table: &mut Value, update_info: Vec<AddressInfo>) {
    let addresses = table
        .get_next_table("test")
        .unwrap()
        .get_next_table("validator")
        .unwrap()
        .get_next_table("account");
    match addresses {
        Some(v) => {
            if let Value::Array(accounts) = v {
                accounts.clear();
                for info in update_info {
                    let mut account = Table::new();
                    account.insert(
                        "address".to_string(),
                        Value::String(info.pubkey.to_string()),
                    );
                    account.insert(
                        "filename".to_string(),
                        Value::String(format!("./test-accounts/{}.json", info.name)),
                    );
                    accounts.push(Value::Table(account))
                }
            }
        }
        None => println!("Could not find an existing account array"),
    }
}

trait TableDiver {
    fn get_next_table(&mut self, name: &str) -> Option<&mut Value>;
}

impl TableDiver for Value {
    fn get_next_table(&mut self, name: &str) -> Option<&mut Value> {
        match self {
            Value::Table(t) => t.iter_mut().find(|(k, _)| *k == name).map(|(_, v)| v),
            _ => None,
        }
    }
}

#[test]
fn test_update_anchor_toml() {
    let input = r#"[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"

[[test.validator.account]]
address = "7Jmy8EEofM24NRcy5BUmqHgYmXqo5P2N35RBQ6uuJdDe"
filename = "./test-accounts/epoch-gauge.json"

[[test.validator.account]]
address = "C8tANDkvD7K1PHzFDoaK4HmYzjixkw6HFY6dv1akQ85e"
filename = "./test-accounts/epoch-gauge-vote.json"
"#;
    let expected = r#"[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"

[[test.validator.account]]
address = "7Jmy8EEofM24NRcy5BUmqHgYmXqo5P2N35RBQ6uuMatt"
filename = "./test-accounts/epoch-gauge.json"
"#;
    let new_addresses = vec![AddressInfo {
        name: "epoch-gauge".to_string(),
        pubkey: pubkey!("7Jmy8EEofM24NRcy5BUmqHgYmXqo5P2N35RBQ6uuMatt"),
    }];
    let mut input_table = Value::Table(input.parse::<Table>().unwrap());
    update_anchor_toml(&mut input_table, new_addresses);
    assert_eq!(expected, toml::to_string(&input_table).unwrap());
}
