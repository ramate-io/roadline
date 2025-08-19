use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A long id is a 512 byte id.
///
/// This is used to identify a task or dependency.
/// It is also used to search for the task or dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LongId([u8; 512]);

impl LongId {
    /// Creates a new LongId from a byte array.
    pub fn new(bytes: [u8; 512]) -> Self {
        Self(bytes)
    }

    /// Creates a new LongId from a string, padding with zeros if necessary.
    pub fn from_string(s: &str) -> Self {
        let mut bytes = [0u8; 512];
        let src_bytes = s.as_bytes();
        let copy_len = src_bytes.len().min(512);
        bytes[..copy_len].copy_from_slice(&src_bytes[..copy_len]);
        Self(bytes)
    }

    /// Returns the raw byte array.
    pub fn as_bytes(&self) -> &[u8; 512] {
        &self.0
    }

    /// Converts to a string, trimming null bytes.
    pub fn to_string_lossy(&self) -> String {
        // Find the first null byte or use the entire array
        let end = self.0.iter().position(|&b| b == 0).unwrap_or(512);
        String::from_utf8_lossy(&self.0[..end]).to_string()
    }
}

impl Serialize for LongId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as a hex string
        let hex_string = self.0.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        serializer.serialize_str(&hex_string)
    }
}

impl<'de> Deserialize<'de> for LongId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_string = String::deserialize(deserializer)?;
        
        if hex_string.len() != 1024 { // 512 bytes * 2 hex chars per byte
            return Err(serde::de::Error::custom(format!(
                "Invalid hex string length: expected 1024 chars, got {}",
                hex_string.len()
            )));
        }

        let mut bytes = [0u8; 512];
        for (i, chunk) in hex_string.as_bytes().chunks(2).enumerate() {
            if i >= 512 {
                break;
            }
            let hex_byte = std::str::from_utf8(chunk)
                .map_err(|e| serde::de::Error::custom(format!("Invalid UTF-8 in hex: {}", e)))?;
            let byte = u8::from_str_radix(hex_byte, 16)
                .map_err(|e| serde::de::Error::custom(format!("Invalid hex byte '{}': {}", hex_byte, e)))?;
            bytes[i] = byte;
        }
        
        Ok(Self(bytes))
    }
}

impl AsRef<str> for LongId {
    fn as_ref(&self) -> &str {
        // This is problematic since we can't guarantee valid UTF-8
        // Return a static reference to indicate this should be avoided
        "invalid_utf8_conversion"
    }
}

impl std::fmt::Display for LongId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_new_from_bytes() {
        let mut bytes = [0u8; 512];
        bytes[0] = 0x41; // 'A'
        bytes[1] = 0x42; // 'B'
        bytes[2] = 0x43; // 'C'
        
        let id = LongId::new(bytes);
        assert_eq!(id.as_bytes(), &bytes);
    }

    #[test]
    fn test_from_string_short() {
        let id = LongId::from_string("hello");
        let bytes = id.as_bytes();
        
        // Check that the string is at the beginning
        assert_eq!(&bytes[0..5], b"hello");
        // Check that the rest is zero-padded
        assert_eq!(&bytes[5..10], &[0u8; 5]);
        assert_eq!(bytes[511], 0);
    }

    #[test]
    fn test_from_string_empty() {
        let id = LongId::from_string("");
        let bytes = id.as_bytes();
        
        // Should be all zeros
        assert_eq!(bytes, &[0u8; 512]);
    }

    #[test]
    fn test_from_string_max_length() {
        let long_string = "a".repeat(512);
        let id = LongId::from_string(&long_string);
        let bytes = id.as_bytes();
        
        // Should be all 'a's
        assert_eq!(bytes, &[b'a'; 512]);
    }

    #[test]
    fn test_from_string_too_long() {
        let too_long_string = "a".repeat(600);
        let id = LongId::from_string(&too_long_string);
        let bytes = id.as_bytes();
        
        // Should be truncated to 512 bytes
        assert_eq!(bytes, &[b'a'; 512]);
    }

    #[test]
    fn test_to_string_lossy_simple() {
        let id = LongId::from_string("test");
        assert_eq!(id.to_string_lossy(), "test");
    }

    #[test]
    fn test_to_string_lossy_with_nulls() {
        let mut bytes = [0u8; 512];
        bytes[0] = b'h';
        bytes[1] = b'i';
        // bytes[2] is already 0
        bytes[3] = b'!'; // This should not appear in output
        
        let id = LongId::new(bytes);
        assert_eq!(id.to_string_lossy(), "hi");
    }

    #[test]
    fn test_to_string_lossy_full_array() {
        let id = LongId::from_string(&"x".repeat(512));
        let result = id.to_string_lossy();
        assert_eq!(result.len(), 512);
        assert_eq!(result, "x".repeat(512));
    }

    #[test]
    fn test_to_string_lossy_invalid_utf8() {
        let mut bytes = [0u8; 512];
        bytes[0] = 0xFF; // Invalid UTF-8
        bytes[1] = 0xFE; // Invalid UTF-8
        
        let id = LongId::new(bytes);
        let result = id.to_string_lossy();
        
        // Should handle invalid UTF-8 gracefully
        assert!(!result.is_empty());
        // from_utf8_lossy replaces invalid sequences with replacement character
        assert!(result.contains('\u{FFFD}'));
    }

    #[test]
    fn test_display_trait() {
        let id = LongId::from_string("display_test");
        let displayed = format!("{}", id);
        assert_eq!(displayed, "display_test");
    }

    #[test]
    fn test_as_ref_str() {
        let id = LongId::from_string("test");
        let s: &str = id.as_ref();
        assert_eq!(s, "invalid_utf8_conversion");
    }

    #[test]
    fn test_equality() {
        let id1 = LongId::from_string("same");
        let id2 = LongId::from_string("same");
        let id3 = LongId::from_string("different");
        
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_ordering() {
        let id1 = LongId::from_string("aaa");
        let id2 = LongId::from_string("bbb");
        let id3 = LongId::from_string("aaa");
        
        assert!(id1 < id2);
        assert!(id2 > id1);
        assert_eq!(id1.cmp(&id3), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let id1 = LongId::from_string("hash_test");
        let id2 = LongId::from_string("hash_test");
        
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        
        id1.hash(&mut hasher1);
        id2.hash(&mut hasher2);
        
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_copy_clone() {
        let id1 = LongId::from_string("copy_test");
        let id2 = id1; // Copy
        let id3 = id1.clone(); // Clone
        
        assert_eq!(id1, id2);
        assert_eq!(id1, id3);
        assert_eq!(id2, id3);
    }

    #[test]
    fn test_serialize_simple() {
        let id = LongId::from_string("abc");
        let json = serde_json::to_string(&id).unwrap();
        
        // Should be a hex string wrapped in quotes
        assert!(json.starts_with('"'));
        assert!(json.ends_with('"'));
        
        // Remove quotes and check hex format
        let hex_str = &json[1..json.len()-1];
        assert_eq!(hex_str.len(), 1024); // 512 bytes * 2 hex chars
        
        // First 6 chars should be "616263" (hex for "abc")
        assert!(hex_str.starts_with("616263"));
        
        // Rest should be zeros
        assert!(hex_str.ends_with(&"00".repeat(509)));
    }

    #[test]
    fn test_deserialize_simple() {
        let original = LongId::from_string("test");
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LongId = serde_json::from_str(&json).unwrap();
        
        assert_eq!(original, deserialized);
        assert_eq!(deserialized.to_string_lossy(), "test");
    }

    #[test]
    fn test_serialize_deserialize_full_array() {
        let original = LongId::from_string(&"z".repeat(512));
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LongId = serde_json::from_str(&json).unwrap();
        
        assert_eq!(original, deserialized);
        assert_eq!(deserialized.to_string_lossy(), "z".repeat(512));
    }

    #[test]
    fn test_serialize_deserialize_binary_data() {
        let mut bytes = [0u8; 512];
        for i in 0..512 {
            bytes[i] = (i % 256) as u8;
        }
        
        let original = LongId::new(bytes);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: LongId = serde_json::from_str(&json).unwrap();
        
        assert_eq!(original, deserialized);
        assert_eq!(original.as_bytes(), deserialized.as_bytes());
    }

    #[test]
    fn test_deserialize_invalid_length() {
        // Too short
        let short_hex = format!("\"{}\"", "ab".repeat(100)); // 200 chars instead of 1024
        let result: Result<LongId, _> = serde_json::from_str(&short_hex);
        assert!(result.is_err());
        
        // Too long
        let long_hex = format!("\"{}\"", "ab".repeat(600)); // 1200 chars instead of 1024
        let result: Result<LongId, _> = serde_json::from_str(&long_hex);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_hex() {
        // Contains non-hex characters
        let mut invalid_hex = "ab".repeat(511);
        invalid_hex.push_str("gg"); // 'g' is not a valid hex digit
        let json = format!("\"{}\"", invalid_hex);
        
        let result: Result<LongId, _> = serde_json::from_str(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_odd_length_hex() {
        // 1023 chars (odd number, should be even for valid hex)
        let odd_hex = format!("\"{}\"", "a".repeat(1023));
        let result: Result<LongId, _> = serde_json::from_str(&odd_hex);
        assert!(result.is_err());
    }

    #[test]
    fn test_round_trip_consistency() {
        let long_x = "x".repeat(100);
        let long_y = "y".repeat(512);
        
        let test_cases = vec![
            "",
            "a",
            "hello world",
            "🦀🚀", // Unicode
            &long_x,
            &long_y,
        ];
        
        for test_case in test_cases {
            let original = LongId::from_string(test_case);
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: LongId = serde_json::from_str(&json).unwrap();
            
            assert_eq!(original, deserialized, "Failed for test case: {}", test_case);
            
            // String representation should match for cases that fit
            if test_case.len() <= 512 {
                assert_eq!(original.to_string_lossy(), deserialized.to_string_lossy());
            }
        }
    }

    #[test]
    fn test_debug_format() {
        let id = LongId::from_string("debug");
        let debug_str = format!("{:?}", id);
        
        // Should contain "LongId" and show the byte array
        assert!(debug_str.contains("LongId"));
        assert!(debug_str.contains("100")); // ASCII 'd'
        assert!(debug_str.contains("101")); // ASCII 'e'
    }

    #[test]
    fn test_edge_cases() {
        // Test with various string lengths around boundaries
        for len in [0, 1, 255, 256, 511, 512, 513, 1000] {
            let test_str = if len <= 512 {
                "a".repeat(len)
            } else {
                "a".repeat(len) // Will be truncated
            };
            
            let id = LongId::from_string(&test_str);
            
            // Should never panic
            let _ = id.to_string_lossy();
            let _ = format!("{}", id);
            let _ = serde_json::to_string(&id).unwrap();
        }
    }

    #[test]
    fn test_all_zeros() {
        let id = LongId::new([0u8; 512]);
        assert_eq!(id.to_string_lossy(), "");
        
        // Should serialize/deserialize correctly
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: LongId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }

    #[test]
    fn test_all_max_bytes() {
        let id = LongId::new([255u8; 512]);
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: LongId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
        
        // Should be all "ff" in hex
        let hex_str = &json[1..json.len()-1];
        assert_eq!(hex_str, "ff".repeat(512));
    }
}