CREATE TABLE IF NOT EXISTS public.queues (
    name character varying COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT queues_pkey PRIMARY KEY (name)
);
CREATE SCHEMA IF NOT EXISTS queues;