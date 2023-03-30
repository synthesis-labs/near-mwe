use decorator::state_safe;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Main {
    owner: String,
}

#[near_bindgen]
impl Main {
    #[state_safe]
    pub fn change_owner(&mut self, new_owner: String) -> () {
        self.owner = new_owner;
    }
}

fn main() {
    let mut m_test = Main {
        owner: String::from("bob"),
    };

    m_test.change_owner(String::from("Alice"));
}
