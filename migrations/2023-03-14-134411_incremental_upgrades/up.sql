CREATE TABLE incremental_updates
(
    id                   SERIAL PRIMARY KEY,
    last_processed_block BIGINT    NOT NULL,
    chain_id             INT       NOT NULL,
    created_at           TIMESTAMP NOT NULL
)