--liquibase formatted sql

--changeset guillaume.marescaux:3
CREATE TABLE heater_timeslot(
    pk BIGSERIAL,
    target_temperature DOUBLE PRECISION NOT NULL,
    day INTEGER NOT NULL CHECK (day BETWEEN 0 AND 6),
    start_time TIME NOT NULL,
    end_time TIME NOT NULL CHECK (end_time > start_time)
);
-- rollback DROP TABLE heater_timeslot;