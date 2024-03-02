use deadpool_postgres::GenericClient;
use postgres_types::{FromSql, ToSql};

use crate::pool::PgClient;

const SQL: &str = r#"
CALL pg_get_procedure(p_result1 := $1, p_result2 := $2)
"#;

#[derive(Debug, ToSql, FromSql, PartialEq)]
#[postgres(name = "type_pg_get_procedure", rename_all = "snake_case")]
struct TypePgGetProcedure {
    content: String,
    count: i64,
}

pub(crate) async fn execute(pg_client: &PgClient) -> anyhow::Result<()> {
    let data1 = TypePgGetProcedure {
        content: "abc".to_string(),
        count: 1,
    };

    let data2 = TypePgGetProcedure {
        content: "efg".to_string(),
        count: 2,
    };

    let res = pg_client.query_one(SQL, &[&data1, &data2]).await?;
    let res1: TypePgGetProcedure = res.get(0);
    let res2: TypePgGetProcedure = res.get(1);
    assert_eq!("xxxabcxxx", res1.content);
    assert_eq!(2, res1.count);
    assert_eq!("yyyefgyyy", res2.content);
    assert_eq!(20, res2.count);

    Ok(())
}