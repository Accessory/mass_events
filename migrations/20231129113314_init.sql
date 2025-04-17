CREATE EXTENSION IF NOT EXISTS "moddatetime";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS public.schedules
(
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    cron_line character varying COLLATE pg_catalog."default" NOT NULL,
    command character varying COLLATE pg_catalog."default" NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT now(),
    modified_at timestamp with time zone NOT NULL DEFAULT now(),
    CONSTRAINT schedules_pkey PRIMARY KEY (id)
);

CREATE TRIGGER update_modified_at_schedules
	BEFORE UPDATE ON public.schedules
	FOR EACH ROW
	EXECUTE PROCEDURE moddatetime (modified_at);