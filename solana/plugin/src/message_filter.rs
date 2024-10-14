use {crate::AddressType, solana_sdk::pubkey::Pubkey, std::collections::HashSet};

#[derive(Debug)]
pub struct MessageFilter {
    accounts: HashSet<AddressType>,
}

impl MessageFilter {
    pub fn new() -> Self {
        MessageFilter {
            accounts: HashSet::new(),
        }
    }

    pub fn add_account(&mut self, account: &Pubkey) {
        self.accounts.insert(account.to_bytes());
    }

    pub fn is_registered(&self, account: &[u8]) -> bool {
        self.accounts.contains(account)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, solana_sdk::pubkey::Pubkey};

    #[test]
    fn test_account_filtering() {
        let mut message_filter = MessageFilter::new();

        let account1 = Pubkey::new_unique();
        let account2 = Pubkey::new_unique();

        assert!(!message_filter.is_registered(&account1.to_bytes()));
        assert!(!message_filter.is_registered(&account2.to_bytes()));

        message_filter.add_account(&account1);
        assert!(message_filter.is_registered(&account1.to_bytes()));
        assert!(!message_filter.is_registered(&account2.to_bytes()));

        message_filter.add_account(&account2);
        assert!(message_filter.is_registered(&account1.to_bytes()));
        assert!(message_filter.is_registered(&account2.to_bytes()));
    }
}
