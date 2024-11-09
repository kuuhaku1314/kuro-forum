use diesel::table;
table! {
    user_tab (id) {
        id -> BigInt,
        nickname -> Text,
        username -> Text,
        password -> Text,
        salt -> Text,
        create_time -> BigInt,
        update_time -> BigInt
    }
}
