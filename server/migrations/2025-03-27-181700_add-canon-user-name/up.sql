-- Your SQL goes here
CREATE FUNCTION canon_user_name(text) RETURNS text AS $$
  SELECT replace(lower($1), '-', '_')
    $$ LANGUAGE SQL
  ;
