use crate::blockchain::*;
use std::fmt;

#[derive(Debug)]
pub struct Transaction {
    pub sender_address: Vec<u8>,
    pub recipient_address: Vec<u8>,
    pub value: u64,
}

impl Transaction {
    pub fn new(sender: Vec<u8>, recipient: Vec<u8>, value: u64) -> Self {
        Transaction {
            sender_address: sender,
            recipient_address: recipient,
            value,
        }
    }
}

impl Serialization<Transaction> for Transaction {
    fn serialization(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();

        let len_sender = self.sender_address.len();
        bin.extend(len_sender.to_be_bytes().to_vec());
        bin.extend(&self.sender_address);

        let len_recipient = self.recipient_address.len();
        bin.extend(len_recipient.to_be_bytes().to_vec());
        bin.extend(&self.recipient_address);

        let len_value = self.value.to_be_bytes().len();
        bin.extend(len_value.to_be_bytes().to_vec());
        bin.extend(&self.value.to_be_bytes().to_vec());

        bin
    }

    fn deserialization(bytes: &Vec<u8>) -> Transaction {
        let mut pos = 0;

        // try_into is a trait used to convert slice into array
        let len_sender = usize::from_be_bytes(bytes[pos..pos+8].try_into().unwrap());
        let mut sender_address = Vec::<u8>::new();
        pos += 8;
        sender_address.extend_from_slice(&bytes[pos..pos+len_sender]);
        pos += len_sender;

        let len_recipient = usize::from_be_bytes(bytes[pos..pos+8].try_into().unwrap());
        pos += 8;
        let mut recipient_address = Vec::<u8>::new();
        recipient_address.extend_from_slice(&bytes[pos..pos+len_recipient]);
        pos += len_recipient;

        let len_value = usize::from_be_bytes(bytes[pos..pos+8].try_into().unwrap());
        pos += 8;

        let value = u64::from_be_bytes(bytes[pos..pos+len_value].try_into().unwrap());
        // pos += len_value;

        Transaction {
            sender_address,
            recipient_address,
            value,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: verify sender and recipient address, maybe there's a bad casting
        // using the to_vec() or into() methods
        // sender address: [67]
        write!(
            f,
            "\n{}\nsender address: {:?} \nrecipient address: {:?}\nvalue: {}\n{}",
            "-".repeat(40),
            self.sender_address,
            self.recipient_address,
            self.value,
            "-".repeat(40),
        )
    }
}
