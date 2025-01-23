macro_rules! field_to_vec {
    ($records:expr, $field:ident) => {
        &$records.iter().map(|r| r.$field.into()).collect::<Vec<_>>()
    };
}

macro_rules! field_to_vec_with_type {
    ($records:expr, $field:ident, $type:ty) => {
        &$records.iter().map(|r| r.$field).collect::<Vec<$type>>()
    };
}

macro_rules! option_field_to_vec {
    ($records:expr, $field:ident) => {
        &$records
            .iter()
            .map(|r| r.$field.unwrap_or_default())
            .collect::<Vec<_>>()
    };
}

pub mod fixtures;
