-- Table: public.pyme

-- DROP TABLE IF EXISTS public.pyme;

CREATE TABLE IF NOT EXISTS public.pyme
(
    created timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id integer NOT NULL DEFAULT nextval('pyme_id_seq'::regclass),
    deleted boolean NOT NULL,
    date character varying COLLATE pg_catalog."default",
    customer character varying COLLATE pg_catalog."default",
    product character varying COLLATE pg_catalog."default",
    quantity integer,
    price integer,
    CONSTRAINT pyme_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.pyme
    OWNER to sganis;
-- Index: ix_pyme_customer

-- DROP INDEX IF EXISTS public.ix_pyme_customer;

CREATE INDEX IF NOT EXISTS ix_pyme_customer
    ON public.pyme USING btree
    (customer COLLATE pg_catalog."default" ASC NULLS LAST)
    TABLESPACE pg_default;
-- Index: ix_pyme_date

-- DROP INDEX IF EXISTS public.ix_pyme_date;

CREATE INDEX IF NOT EXISTS ix_pyme_date
    ON public.pyme USING btree
    (date COLLATE pg_catalog."default" ASC NULLS LAST)
    TABLESPACE pg_default;
-- Index: ix_pyme_deleted

-- DROP INDEX IF EXISTS public.ix_pyme_deleted;

CREATE INDEX IF NOT EXISTS ix_pyme_deleted
    ON public.pyme USING btree
    (deleted ASC NULLS LAST)
    TABLESPACE pg_default;
-- Index: ix_pyme_id

-- DROP INDEX IF EXISTS public.ix_pyme_id;

CREATE INDEX IF NOT EXISTS ix_pyme_id
    ON public.pyme USING btree
    (id ASC NULLS LAST)
    TABLESPACE pg_default;
-- Index: ix_pyme_product

-- DROP INDEX IF EXISTS public.ix_pyme_product;

CREATE INDEX IF NOT EXISTS ix_pyme_product
    ON public.pyme USING btree
    (product COLLATE pg_catalog."default" ASC NULLS LAST)
    TABLESPACE pg_default;