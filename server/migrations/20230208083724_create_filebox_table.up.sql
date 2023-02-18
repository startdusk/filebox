CREATE TYPE file_type AS ENUM ('text', 'file');

CREATE TABLE
    IF NOT EXISTS filebox (
        id BIGSERIAL NOT NULL,
        code VARCHAR(10) NOT NULL,
        name VARCHAR(30) NOT NULL,
        size BIGINT NOT NULL DEFAULT 0,
        file_type file_type NOT NULL DEFAULT 'file',
        file_path VARCHAR(250) NOT NULL DEFAULT '',
        text VARCHAR(2000) NOT NULL DEFAULT '',
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        expired_at TIMESTAMP NOT NULL,
        used_at TIMESTAMP DEFAULT NULL,
        CONSTRAINT filebox_pkey PRIMARY KEY (id)
    );

CREATE INDEX filebox_code_idx ON filebox (code);