--liquibase formatted sql

--changeset guillaume.marescaux:2
CREATE TABLE temperature_history(
    pk BIGSERIAL,
    temperature DOUBLE PRECISION NOT NULL
);
