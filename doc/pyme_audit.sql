-- Create audit table and trigger function

create table public.pyme_audit (
    table text not null, 
    user text,
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

