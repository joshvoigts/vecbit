# vecbit

An app for offering "bools as a service".

## Why?

A good excuse to learn how to implement rust web APIs and to learn
about web API authentication.

## Setup

Generate a master session key: 

    cargo run gen_session_master

Create a `.env` file:

    VECB_SESSION_MASTER_KEY={paste master session key}
    VECB_BIND_ADDRESS="127.0.0.1"
    VECB_BIND_PORT=8080
    VECB_DB_PATH="db.sqlite"
    VECB_ENV="development"
    VECB_SMTP_EMAIL="no.reply.vecbit@gmail.com"
    VECB_SMTP_PASSWORD={password for smtp server}
    VECB_STATIC_PATH="web/static"
