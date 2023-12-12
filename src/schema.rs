// @generated automatically by Diesel CLI.

diesel::table! {
    short_urls (id) {
        id -> Int4,
        url -> Text,
        token -> Text,
    }
}
