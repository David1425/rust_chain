/// Utility functions for CLI operations

/// Print a formatted separator line
pub fn print_separator(width: usize) {
    println!("{:-<width$}", "", width = width);
}

/// Format amount with commas for better readability
pub fn format_amount(amount: u64) -> String {
    // Simple formatting - in a real application you might want more sophisticated formatting
    if amount >= 1_000_000 {
        format!("{:.1}M", amount as f64 / 1_000_000.0)
    } else if amount >= 1_000 {
        format!("{:.1}K", amount as f64 / 1_000.0)
    } else {
        amount.to_string()
    }
}

/// Format timestamp to readable date
pub fn format_timestamp(timestamp: u64) -> String {
    if timestamp == 0 {
        "Genesis".to_string()
    } else {
        // Simple timestamp formatting - in a real application you'd use chrono or similar
        format!("Timestamp: {}", timestamp)
    }
}

/// Truncate hash for display
pub fn truncate_hash(hash: &str, length: usize) -> String {
    if hash.len() <= length {
        hash.to_string()
    } else {
        format!("{}...{}", &hash[..length/2], &hash[hash.len()-length/2..])
    }
}

/// Validate address format (simple validation)
pub fn is_valid_address(address: &str) -> bool {
    !address.is_empty() && address.len() >= 3 && address.len() <= 50
}

/// Validate amount
pub fn is_valid_amount(amount: u64) -> bool {
    amount > 0 && amount <= 1_000_000_000 // Max 1 billion
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_amount() {
        assert_eq!(format_amount(100), "100");
        assert_eq!(format_amount(1500), "1.5K");
        assert_eq!(format_amount(2_500_000), "2.5M");
    }

    #[test]
    fn test_truncate_hash() {
        let hash = "0000abcd1234567890abcdef";
        assert_eq!(truncate_hash(hash, 8), "0000...cdef");
        assert_eq!(truncate_hash("short", 10), "short");
    }

    #[test]
    fn test_address_validation() {
        assert!(is_valid_address("alice"));
        assert!(is_valid_address("bob123"));
        assert!(!is_valid_address(""));
        assert!(!is_valid_address("ab"));
    }

    #[test]
    fn test_amount_validation() {
        assert!(is_valid_amount(100));
        assert!(is_valid_amount(999_999_999));
        assert!(!is_valid_amount(0));
        assert!(!is_valid_amount(1_000_000_001));
    }
}
