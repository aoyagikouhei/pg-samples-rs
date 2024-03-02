DROP TYPE IF EXISTS type_pg_get_procedure CASCADE;
CREATE TYPE type_pg_get_procedure AS (
  content TEXT
  ,count BIGINT
);

CREATE OR REPLACE PROCEDURE pg_get_procedure(
  p_result1 INOUT type_pg_get_procedure DEFAULT NULL
  ,p_result2 INOUT type_pg_get_procedure DEFAULT NULL
) AS $PROCEDURE$
DECLARE
BEGIN
  p_result1.content := 'xxx' || p_result1.content || 'xxx';
  p_result1.count := p_result1.count + 1;
  p_result2.content := 'yyy' || p_result2.content || 'yyy';
  p_result2.count := p_result2.count * 10;
END;
$PROCEDURE$ LANGUAGE plpgsql;
