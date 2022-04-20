use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize};
use near_sdk::collections::{Vector};

use std::cmp::min;
use near_helper::{yoctonear_to_near, expect_lightweight};


const MESSAGE_LIMIT: u64 = 10;
const PREMIUM_LIMIT: u128 = 10_000_000_000_000_000_000_000;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessage {
    premium: bool,
    money: f64,
    sender: AccountId,
    text: String,
    datetime: String,
}

/// INTERNAL USE ONLY. 
impl Default for PostedMessage {
  fn default() -> Self {
    Self {
      premium: false,
      money: 0f64,
      sender: "not_found".parse().unwrap(),
      text: "".to_owned(),
      datetime: "".to_owned()
    }
  }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MessageList {
    messages: Vector<PostedMessage>,
}

impl Default for MessageList {
    fn default() -> Self {
      Self {
        messages: Vector::new(b"a".to_vec()),
      }
    }
}




#[near_bindgen]
impl MessageList {
    /// Add a message. Include the datetime. 
    #[payable]
    pub fn add_message(&mut self, text: String, datetime: String) {

      let mut money = 0f64;
      if env::attached_deposit() > 0u128 {
        money = yoctonear_to_near(env::attached_deposit());
      }

      let message = PostedMessage {
        premium: env::attached_deposit() >= PREMIUM_LIMIT,
        money: money,
        sender: env::signer_account_id(),
        text,
        datetime
      };

      self.messages.push(&message);
    }


    // Cannot borsh serialize
    /// Get latest 10 messages. 
    pub fn get_messages(&self) -> Vec<PostedMessage> {
      let num_messages = min(MESSAGE_LIMIT, self.messages.len());
      let start_index = self.messages.len() - num_messages;
      let init_message: PostedMessage = match self.messages.get(0) {
        Some(message) => message,
        None => PostedMessage::default()
      };
      let mut arr = vec![init_message.clone(); num_messages as usize];

      for i in 0..num_messages {
        arr[i as usize] = match self.messages.get(start_index + i) {
          Some(message) => message,
          None => init_message.clone()
        }
      }

      arr
    }


    /// Get a single message
    pub fn get_single_message(&self, index: u64) -> Vec<PostedMessage> {
      match self.messages.get(index) {
        Some(message) => vec![message],
        None => vec![PostedMessage::default()]
      }
    }
}


#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext, Balance};
    use chrono;


    fn sender() -> AccountId {
      "bob_near".parse().unwrap()
    }

    // fn sender2() -> AccountId {
    //   "alice_near".parse().unwrap()
    // }


    fn get_context(is_view: bool, amount: Balance) -> VMContext {
      VMContextBuilder::new()
          .signer_account_id(sender())
          .attached_deposit(amount)
          .is_view(is_view)
          .build()
    }

    fn debug(contract: &MessageList) {
      eprintln!("{:#?}", contract.get_messages());
      assert_eq!(1, 2, "DEBUG MODE ACTIVATED!");
    }

    fn single_message(amount: Balance) -> (MessageList, &'static str) {
      let context = get_context(false, amount);
      testing_env!(context);

      let mut contract = MessageList::default();
      let our_message: &str = "Hello, world!";

      let date = chrono::offset::Utc::now().to_string();

      contract.add_message(our_message.to_owned(), date);

      (contract, our_message)
    }


    #[test]
    fn test_single_set_message_then_get() {
      let (contract, our_message) = single_message(0);

      // debug(&contract);
      let first_msg = contract.get_messages()[0].clone();

      assert_eq!(
        first_msg.text,
        our_message.to_owned()
      );
      assert_eq!(
        first_msg.money,
        0f64
      );
      assert_eq!(
        first_msg.sender,
        sender()
      );
      assert_eq!(
        first_msg.premium,
        false
      );
      
      // Unsure how to test for date; it could change anytime. 
    }

    #[test]
    fn test_single_message_premium() {
      let (contract, _our_message) = single_message(PREMIUM_LIMIT);

      // debug(&contract);
      let first_msg = contract.get_messages()[0].clone();
      assert_eq!(
        first_msg.premium,
        true 
      );
      assert_eq!(
        first_msg.money,
        0.01f64
      );
    }

    #[test]
    fn test_multiple_messages() {
      let (mut contract, our_message) = single_message(0);

      let second_msg: &str = "how bout Alice?";
      contract.add_message(second_msg.to_owned(), chrono::offset::Utc::now().to_string());
      // debug(&contract);

      let messages = contract.get_messages();

      assert_eq!(messages.len(), 2);

      // Check second message correct
      assert_eq!(
        messages[1].text,
        second_msg.to_owned()
      );
    }


    #[test]
    fn test_max_message_is_ten() {
      let (mut contract, our_message) = single_message(0);

      for i in 0..11 {
        contract.add_message(i.to_string(), chrono::offset::Utc::now().to_string());
      }

      let messages = contract.get_messages();

      assert!(
        messages.len() <= MESSAGE_LIMIT as usize,
        "Unexpected Message length is longer than MESSAGE_LIMIT"
      );

      let last_message = messages[9].clone();

      assert_eq!(
        last_message.text,
        "10".to_owned()
      );
    }
}

