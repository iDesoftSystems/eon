use std::fmt::Display;

/// Represents a unique identifier for a resource.
///
/// This enum supports various ID types (integers and strings) and implements `Display`
/// for easy formatting.
#[derive(Debug)]
pub enum ResourceId {
    U32(u32),
    U64(u64),
    I32(i32),
    I64(i64),
    String(String),
}

impl Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceId::U32(val) => write!(f, "{}", val),
            ResourceId::U64(val) => write!(f, "{}", val),
            ResourceId::I64(val) => write!(f, "{}", val),
            ResourceId::I32(val) => write!(f, "{}", val),
            ResourceId::String(val) => write!(f, "{}", val),
        }
    }
}

impl From<u32> for ResourceId {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for ResourceId {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<i32> for ResourceId {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for ResourceId {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for ResourceId {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_id_display_fmt() {
        assert_eq!(ResourceId::U32(100).to_string(), "100");
        assert_eq!(ResourceId::U64(999999).to_string(), "999999");
        assert_eq!(ResourceId::I32(-50).to_string(), "-50");
        assert_eq!(ResourceId::I64(-1000).to_string(), "-1000");
        assert_eq!(
            ResourceId::String("custom-id".to_string()).to_string(),
            "custom-id"
        );
    }

    #[test]
    fn test_resource_id_from_implementations() {
        // Test From<u32>
        let val_u32: u32 = 123;
        match ResourceId::from(val_u32) {
            ResourceId::U32(value) => assert_eq!(value, 123),
            _ => panic!("Expected U32 variant"),
        }

        // Test From<u64>
        let val_u64: u64 = 123456789;
        match ResourceId::from(val_u64) {
            ResourceId::U64(value) => assert_eq!(value, 123456789),
            _ => panic!("Expected U64 variant"),
        }

        // Test From<i32>
        let val_i32: i32 = -123;
        match ResourceId::from(val_i32) {
            ResourceId::I32(value) => assert_eq!(value, -123),
            _ => panic!("Expected I32 variant"),
        }

        // Test From<i64>
        let val_i64: i64 = -987654321;
        match ResourceId::from(val_i64) {
            ResourceId::I64(value) => assert_eq!(value, -987654321),
            _ => panic!("Expected I64 variant"),
        }

        // Test From<String>
        let val_string = "my-uuid".to_string();
        match ResourceId::from(val_string) {
            ResourceId::String(value) => assert_eq!(value, "my-uuid"),
            _ => panic!("Expected String variant"),
        }

        // Test From<&str>
        let val_str = "my-str-uuid";
        match ResourceId::from(val_str) {
            ResourceId::String(value) => assert_eq!(value, "my-str-uuid"),
            _ => panic!("Expected String variant"),
        }
    }
}
