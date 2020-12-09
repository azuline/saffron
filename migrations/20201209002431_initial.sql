-- Add migration script here

CREATE TABLE users (
    id INTEGER NOT NULL,
    nickname VARCHAR NOT NULL,
    token_prefix BLOB NOT NULL,
    token_hash BLOB NOT NULL,
    csrf_token BLOB NOT NULL,
    PRIMARY KEY (id),
    UNIQUE (token_prefix)
);

CREATE TABLE files (
    id INTEGER NOT NULL,
    filename VARCHAR NOT NULL,
    uploader_id INTEGER NOT NULL,
    uploaded_on TIMESTAMP DEFAULT (CURRENT_TIMESTAMP) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE(filename)
);

CREATE TABLE secret_key (
    key BLOB NOT NULL,
    PRIMARY KEY (key)
)
