CREATE TABLE IF NOT EXISTS greetings (
    id         BIGINT GENERATED ALWAYS AS IDENTITY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    greeting   VARCHAR(64)
);
