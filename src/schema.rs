// @generated automatically by Diesel CLI.

diesel::table! {
    urls (id) {
        id -> Int4,
        shortened_url -> Varchar,
        redirect_to -> Varchar,
    }
}
