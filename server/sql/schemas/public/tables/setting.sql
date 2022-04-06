--liquibase formatted sql

--changeset guillaume.marescaux:4
CREATE TABLE setting (
    pk BIGSERIAL,
    key VARCHAR(128) UNIQUE,
    value TEXT
);
-- rollback DROP TABLE setting;

--changeset guillaume.marescaux:5; runOnChange:true;
INSERT INTO setting(key, value)
     VALUES ('default_temperature', '18')
          , ('holiday_mode_enabled', 'false')
          , ('manual_mode_enabled', 'false')
          , ('manual_mode_temperature', '18')
ON CONFLICT (key) DO NOTHING
;