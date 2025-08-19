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