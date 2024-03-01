use postgres_types::{FromSql, ToSql};

use crate::pool::PgClient;

const SQL: &str = r#"
SELECT
    ROW(t1.*)::type_pg_get_select_composite
FROM
    pg_get_select_composite(
        p_value := $1
    ) AS t1 
"#;

#[derive(Debug, ToSql, FromSql, PartialEq)]
#[postgres(name = "type_enum_os", rename_all = "snake_case")]
enum TypeEnumOs {
    Linux,
    MacOs,
    Windows
}

#[derive(Debug, ToSql, FromSql, PartialEq)]
#[postgres(name = "domain_integer", rename_all = "snake_case")]
struct DomainInteger(i64);

#[derive(Debug, ToSql, FromSql, PartialEq)]
#[postgres(transparent)]
struct MemorySizeUnit(i64);

#[derive(Debug, ToSql, FromSql, PartialEq)]
#[postgres(name = "type_pg_get_select_composite", rename_all = "snake_case")]
struct TypePgGetSelectComposite {
    os: TypeEnumOs,
    cpu_count: DomainInteger,
    memory_size: MemorySizeUnit,
}

pub(crate) async fn execute(pg_client: &PgClient) -> anyhow::Result<()> {
    let data = TypePgGetSelectComposite {
        os: TypeEnumOs::Linux,
        cpu_count: DomainInteger(2),
        memory_size: MemorySizeUnit(1024),
    };

    let rows: Vec<TypePgGetSelectComposite> = pg_client.query(SQL, &[&data]).await?
        .iter()
        .map(|row| row.get(0))
        .collect();

    assert_eq!(rows[0], data);
    assert_eq!(rows[1], data);
    println!("{:?}", rows[0]);

    Ok(())
}