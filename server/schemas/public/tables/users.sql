--liquibase formatted sql

--changeset guillaume.marescaux:1

CREATE USER server WITH ENCRYPTED PASSWORD 'server';