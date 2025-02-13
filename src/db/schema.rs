// @generated automatically by Diesel CLI.

diesel::table! {
    short_urls (id) {
        id -> Int4,
        original_url -> Text,
        #[max_length = 16]
        short_code -> Varchar,
        created_at -> Timestamp,
        encrypted -> Bool,
        expired -> Bool,
        transaction_hash -> Nullable<Text>,
    }
}
