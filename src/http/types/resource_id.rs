use std::fmt::Display;

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
