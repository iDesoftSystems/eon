use chrono::NaiveDateTime;

pub trait EndOfDay {
    fn end_of_day(&self) -> Option<NaiveDateTime>;
}
