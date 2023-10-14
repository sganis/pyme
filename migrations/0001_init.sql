-- Table: public.pyme_order

-- DROP TABLE IF EXISTS public.pyme_order;

CREATE TABLE IF NOT EXISTS public.pyme_order
(
    created timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id integer NOT NULL DEFAULT nextval('pyme_id_seq'::regclass),
    deleted boolean NOT NULL DEFAULT false,
    date character varying COLLATE pg_catalog."default",
    customer character varying COLLATE pg_catalog."default",
    price integer,
    paid boolean DEFAULT true,
    notes character varying COLLATE pg_catalog."default",
    username character varying COLLATE pg_catalog."default",
    items json,
    CONSTRAINT pyme_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.pyme_order
    OWNER to sganis;
-- Index: ix_pyme_customer

-- DROP INDEX IF EXISTS public.ix_pyme_customer;

CREATE INDEX IF NOT EXISTS ix_pyme_customer
    ON public.pyme_order USING btree
    (customer COLLATE pg_catalog."default" ASC NULLS LAST)
    TABLESPACE pg_default;
-- Index: ix_pyme_date

-- DROP INDEX IF EXISTS public.ix_pyme_date;

CREATE INDEX IF NOT EXISTS ix_pyme_date
    ON public.pyme_order USING btree
    (date COLLATE pg_catalog."default" ASC NULLS LAST)
    TABLESPACE pg_default;
-- Index: ix_pyme_deleted

-- DROP INDEX IF EXISTS public.ix_pyme_deleted;

CREATE INDEX IF NOT EXISTS ix_pyme_deleted
    ON public.pyme_order USING btree
    (deleted ASC NULLS LAST)
    TABLESPACE pg_default;
-- Index: ix_pyme_id

-- DROP INDEX IF EXISTS public.ix_pyme_id;

CREATE INDEX IF NOT EXISTS ix_pyme_id
    ON public.pyme_order USING btree
    (id ASC NULLS LAST)
    TABLESPACE pg_default;



-- Table: public.pyme_config

-- DROP TABLE IF EXISTS public.pyme_config;

CREATE TABLE IF NOT EXISTS public.pyme_config
(
    created timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    id integer NOT NULL DEFAULT nextval('pymeproduct_id_seq'::regclass),
    deleted boolean NOT NULL,
    key character varying COLLATE pg_catalog."default",
    username character varying COLLATE pg_catalog."default",
    value json,
    CONSTRAINT pyme_config_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.pyme_config
    OWNER to sganis;
-- Index: ix_pyme_config_deleted

-- DROP INDEX IF EXISTS public.ix_pyme_config_deleted;

CREATE INDEX IF NOT EXISTS ix_pyme_config_deleted
    ON public.pyme_config USING btree
    (deleted ASC NULLS LAST)
    TABLESPACE pg_default;
-- Index: ix_pyme_config_id

-- DROP INDEX IF EXISTS public.ix_pyme_config_id;

CREATE INDEX IF NOT EXISTS ix_pyme_config_id
    ON public.pyme_config USING btree
    (id ASC NULLS LAST)
    TABLESPACE pg_default;
-- Index: ix_pyme_config_key

-- DROP INDEX IF EXISTS public.ix_pyme_config_key;

CREATE INDEX IF NOT EXISTS ix_pyme_config_key
    ON public.pyme_config USING btree
    (key COLLATE pg_catalog."default" ASC NULLS LAST)
    TABLESPACE pg_default;

-- Trigger: pyme_config_audit_tgr

-- DROP TRIGGER IF EXISTS pyme_config_audit_tgr ON public.pyme_config;

CREATE OR REPLACE TRIGGER pyme_config_audit_tgr
    AFTER UPDATE 
    ON public.pyme_config
    FOR EACH ROW
    EXECUTE FUNCTION public.if_modified_func();

-- Trigger: pyme_product_audit_tgr

-- DROP TRIGGER IF EXISTS pyme_product_audit_tgr ON public.pyme_config;

CREATE OR REPLACE TRIGGER pyme_product_audit_tgr
    AFTER UPDATE 
    ON public.pyme_config
    FOR EACH ROW
    EXECUTE FUNCTION public.if_modified_func();





-- Create audit table and trigger function
create table public.pyme_audit (
    tablename text not null, 
    username text,
    date timestamp with time zone not null default current_timestamp,
    action TEXT NOT NULL check (action in ('I','D','U')),
    old_data json,
    new_data json,
    query text
) with (fillfactor=100);

create index pyme_audit_tablename_idx 
on public.pyme_audit(tablename);

create index pyme_audit_date_idx 
on public.pyme_audit(date);

create index pyme_audit_action_idx 
on public.pyme_audit(action);


CREATE OR REPLACE FUNCTION public.if_modified_func() 
RETURNS trigger AS $body$
DECLARE
    v_old_data json;
    v_new_data json;
BEGIN
    if (TG_OP = 'UPDATE') then
        v_old_data := row_to_json(OLD);
        v_new_data := row_to_json(NEW);
        insert into public.pyme_audit (tablename,username,action,old_data,new_data,query) 
        values (TG_TABLE_NAME::TEXT,session_user::TEXT,substring(TG_OP,1,1),v_old_data,v_new_data, current_query());
        RETURN NEW;
    elsif (TG_OP = 'DELETE') then
        v_old_data := row_to_json(OLD);
        insert into public.pyme_audit (tablename,username,action,old_data,query)
        values (TG_TABLE_NAME::TEXT,session_user::TEXT,substring(TG_OP,1,1),v_old_data, current_query());
        RETURN OLD;
    elsif (TG_OP = 'INSERT') then
        v_new_data := row_to_json(NEW);
        insert into public.pyme_audit (tablename,username,action,new_data,query)
        values (TG_TABLE_NAME::TEXT,session_user::TEXT,substring(TG_OP,1,1),v_new_data, current_query());
        RETURN NEW;
    else
        RAISE WARNING '[public.IF_MODIFIED_FUNC] - Other action occurred: %, at %',TG_OP,now();
        RETURN NULL;
    end if;

EXCEPTION
    WHEN data_exception THEN
        RAISE WARNING '[public.IF_MODIFIED_FUNC] - UDF ERROR [DATA EXCEPTION] - SQLSTATE: %, SQLERRM: %',SQLSTATE,SQLERRM;
        RETURN NULL;
    WHEN unique_violation THEN
        RAISE WARNING '[public.IF_MODIFIED_FUNC] - UDF ERROR [UNIQUE] - SQLSTATE: %, SQLERRM: %',SQLSTATE,SQLERRM;
        RETURN NULL;
    WHEN others THEN
        RAISE WARNING '[public.IF_MODIFIED_FUNC] - UDF ERROR [OTHER] - SQLSTATE: %, SQLERRM: %',SQLSTATE,SQLERRM;
        RETURN NULL;
END;
$body$
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = pg_catalog, public;

--
-- --
CREATE TRIGGER pyme_order_audit_tgr
AFTER UPDATE ON public.pyme_order
FOR EACH ROW EXECUTE PROCEDURE public.if_modified_func();
--
CREATE TRIGGER pyme_config_audit_tgr
AFTER UPDATE ON public.pyme_config
FOR EACH ROW EXECUTE PROCEDURE public.if_modified_func();

