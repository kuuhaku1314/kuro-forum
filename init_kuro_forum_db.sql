CREATE TABLE if not exists user_tab
(
    id
    INTEGER
    PRIMARY
    KEY
    AUTOINCREMENT,
    nickname
    TEXT
    NOT
    NULL,
    username
    TEXT
    NOT
    NULL
    UNIQUE,
    password
    TEXT
    NOT
    NULL,
    salt
    TEXT
    NOT
    NULL,
    create_time
    INTEGER
    NOT
    NULL,
    update_time
    INTEGER
    NOT
    NULL
);

CREATE TABLE if not exists kv_storage_tab
(
    id
    INTEGER
    PRIMARY
    KEY
    AUTOINCREMENT,
    storage_key
    TEXT
    NOT
    NULL
    UNIQUE,
    storage_value
    TEXT
    NOT
    NULL,
    create_time
    INTEGER
    NOT
    NULL,
    update_time
    INTEGER
    NOT
    NULL
)