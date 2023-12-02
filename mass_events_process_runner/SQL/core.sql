CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
-- Table: public.infos
-- DROP TABLE IF EXISTS public.infos;
CREATE TABLE IF NOT EXISTS public.infos (
    key jsonb NOT NULL,
    value jsonb NOT NULL,
    CONSTRAINT infos_pkey PRIMARY KEY (key)
);
CREATE TABLE IF NOT EXISTS public.queues (
    name character varying COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT queues_pkey PRIMARY KEY (name)
)