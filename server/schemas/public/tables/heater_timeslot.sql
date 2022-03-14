--liquibase formatted sql

--changeset guillaume.marescaux:3
CREATE TABLE heater_timeslot(
    pk BIGSERIAL,
    target_temperature DOUBLE PRECISION NOT NULL,
    start_day INTEGER NOT NULL CHECK (start_day BETWEEN 0 AND 6),
    start_time TIME NOT NULL,
    end_day INTEGER NOT NULL CHECK (end_day BETWEEN 0 AND 6),
    end_time TIME NOT NULL
);
