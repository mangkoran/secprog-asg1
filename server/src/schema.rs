// @generated automatically by Diesel CLI.

diesel::table! {
    user (username) {
        username -> Text,
        password -> Text,
    }
}
