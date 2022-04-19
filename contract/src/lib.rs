use near_sdk::{env, near_bindgen, AccountId};
use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize};

use near_sdk::collections::{Vector};
use chrono;
use std::cmp::min;


const MESSAGE_LIMIT: u64 = 10;
const PREMIUM_LIMIT: u128 = 10_000_000_000_000_000_000_000;


#[serde(crate = "near_sdk::serde")]
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Serialize)]
pub struct PostedMessage {
    premium: bool,
    sender: AccountId,
    text: String,
    date: String,
}

/// Not intended to use in smart contract. INTERNAL USE ONLY. 
impl Default for PostedMessage {
  fn default() -> Self {
    Self {
      premium: false,
      sender: "none".parse().unwrap(),
      text: "".to_owned(),
      date: "".to_owned()
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

impl MessageList {

}


// We use camelCase, against all odds,
// so we don't need to change anything in the pre-existing frontend. 

#[near_bindgen]
impl MessageList {
    // https://stackoverflow.com/questions/63210984/chrono-kills-my-rust-webassembly-function
    // #[cfg(all(target_arch = "wasm32", not(target_os = "wasi"), feature = "wasmbind"))]
    // pub fn now(&mut self) -> DateTime<Utc> {
    //     let now = js_sys::Date::new_0();
    //     DateTime::<Utc>::from(now)
    // }
   

    #[payable]
    pub fn addMessage(&mut self, text: String, date: String) {
      // if locality {
      //   let date = chrono::offset::Utc::now().to_string();
      // } else {
      //   let date = chrono::offset::Local::now().to_string();
      // }

      let message = PostedMessage {
        premium: env::attached_deposit() >= PREMIUM_LIMIT,
        // money: env::attached_deposit(),
        sender: env::signer_account_id(),
        text,
        // date: self.now().to_string(),
        date
      };

      self.messages.push(&message);
    }

    // #[result_serializer(borsh)]
    pub fn getMessages(&self) -> Vec<PostedMessage> {
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

      // for i in 0..num_messages {
      //   self.messages.get(i + start_index)
      // }

      // for i in 0..num_messages {
      //   result[i] = self.messages.get(i + start_index);
      // }

      arr
      // self.messages.to_vec()
    }
}


#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext, Balance};


    fn sender() -> AccountId {
      "bob_near".parse().unwrap()
    }

    fn sender2() -> AccountId {
      "alice_near".parse().unwrap()
    }


    fn get_context(is_view: bool, amount: Balance) -> VMContext {
      VMContextBuilder::new()
          .signer_account_id(sender())
          .attached_deposit(amount)
          .is_view(is_view)
          .build()
    }

    fn debug(contract: &MessageList) {
      eprintln!("{:#?}", contract.getMessages());
      assert_eq!(1, 2, "DEBUG MODE ACTIVATED!");
    }

    fn single_message(amount: Balance) -> (MessageList, &'static str) {
      let context = get_context(false, amount);
      testing_env!(context);

      let mut contract = MessageList::default();
      let our_message: &str = "Hello, world!";

      let date = chrono::offset::Utc::now().to_string();

      contract.addMessage(our_message.to_owned(), date);

      (contract, our_message)
    }


    #[test]
    fn test_single_set_message_then_get() {
      let (contract, our_message) = single_message(0);

      // debug(&contract);
      let first_msg = contract.getMessages()[0].clone();

      assert_eq!(
        first_msg.text,
        our_message.to_owned()
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

      let first_msg = contract.getMessages()[0].clone();
      assert_eq!(
        first_msg.premium,
        true 
      );
    }

    #[test]
    fn test_multiple_messages() {
      let (mut contract, our_message) = single_message(0);

      let second_msg: &str = "how bout Alice?";
      contract.addMessage(second_msg.to_owned(), chrono::offset::Utc::now().to_string());
      // debug(&contract);

      let messages = contract.getMessages();

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
        contract.addMessage(i.to_string(), chrono::offset::Utc::now().to_string());
      }

      let messages = contract.getMessages();

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

