-- Create audit table and trigger function

create table public.audit (
    table_name text not null, 
    user_name text,
    action_tstamp timestamp with time zone not null default current_timestamp,
    action TEXT NOT NULL check (action in ('I','D','U')),
    original_data json,
    new_data json,
    query text
) with (fillfactor=100);

create index audit_table_idx 
on public.audit(table_name);

create index audit_tstamp_idx 
on public.audit(action_tstamp);

create index audit_action_idx 
on public.audit(action);


CREATE OR REPLACE FUNCTION public.if_modified_func() RETURNS trigger AS $body$
DECLARE
    v_old_data json;
    v_new_data json;
BEGIN
    /*  If this actually for real auditing (where you need to log EVERY action),
        then you would need to use something like dblink or plperl that could log outside the transaction,
        regardless of whether the transaction committed or rolled back.
    */

    /* This dance with casting the NEW and OLD values to a ROW is not necessary in pg 9.0+ */

    if (TG_OP = 'UPDATE') then
        v_old_data := row_to_json(OLD);
        v_new_data := row_to_json(NEW);
        insert into public.audit (table_name,user_name,action,original_data,new_data,query) 
        values (TG_TABLE_NAME::TEXT,session_user::TEXT,substring(TG_OP,1,1),v_old_data,v_new_data, current_query());
        RETURN NEW;
    elsif (TG_OP = 'DELETE') then
        v_old_data := row_to_json(OLD);
        insert into public.audit (table_name,user_name,action,original_data,query)
        values (TG_TABLE_NAME::TEXT,session_user::TEXT,substring(TG_OP,1,1),v_old_data, current_query());
        RETURN OLD;
    elsif (TG_OP = 'INSERT') then
        v_new_data := row_to_json(NEW);
        insert into public.audit (table_name,user_name,action,new_data,query)
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
-- To add this trigger to a table, use:
CREATE TRIGGER user_audit_tgr
AFTER UPDATE ON public.user
FOR EACH ROW EXECUTE PROCEDURE public.if_modified_func();
-- --
-- CREATE TRIGGER booking_audit_tgr
-- AFTER UPDATE ON public.booking
-- FOR EACH ROW EXECUTE PROCEDURE public.if_modified_func();
-- --
CREATE TRIGGER pyme_order_audit_tgr
AFTER UPDATE ON public.pyme_order
FOR EACH ROW EXECUTE PROCEDURE public.if_modified_func();
--
CREATE TRIGGER pyme_order_item_audit_tgr
AFTER UPDATE ON public.pyme_order_item
FOR EACH ROW EXECUTE PROCEDURE public.if_modified_func();
--
CREATE TRIGGER pyme_product_audit_tgr
AFTER UPDATE ON public.pyme_product
FOR EACH ROW EXECUTE PROCEDURE public.if_modified_func();
--
