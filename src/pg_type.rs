use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
    time::SystemTime,
};

use bit_vec::BitVec;
use chrono::prelude::*;
use eui48::{Eui48, MacAddress};
use geo_types::{Coord, LineString, Point, Rect};
use rust_decimal::Decimal;
use serde_json::Value;
use uuid::Uuid;

use crate::pool::PgClient;

#[derive(Debug, PartialEq)]
struct Data {
    bool_val: bool,
    bool_array_val: Vec<bool>,
    bool_option_some_val: Option<bool>,
    bool_option_none_val: Option<bool>,
    char_val: i8,
    smallint_val: i16,
    int_val: i32,
    oid_val: u32,
    bigint_val: i64,
    real_val: f32,
    double_val: f64,
    text_val: String,
    bytes_val: Vec<u8>,
    hstore_val: HashMap<String, Option<String>>,
    system_time_val: SystemTime,
    inet_val: IpAddr,
    timestamp_val: NaiveDateTime,
    timestamptz_val: DateTime<Utc>,
    date_val: NaiveDate,
    time_val: NaiveTime,
    macaddr_val: MacAddress,
    point_val: Point<f64>,
    box_val: Rect<f64>,
    path_val: LineString<f64>,
    jsonb_val: Value,
    uuid_val: Uuid,
    varbit_val: BitVec,
    decimal_val: Decimal,
}

const SQL: &str = r#"
SELECT
    $1::BOOL AS bool_val
    ,$2::BOOL[] AS bool_array_val
    ,$3::BOOL AS bool_option_some_val
    ,$4::BOOL AS bool_option_none_val
    ,$5::"char" AS char_val
    ,$6::SMALLINT AS smallint_val
    ,$7::INT AS int_val
    ,$8::OID AS oid_val
    ,$9::BIGINT AS bigint_val
    ,$10::REAL AS real_val
    ,$11::DOUBLE PRECISION AS double_val
    ,$12::TEXT AS text_val
    ,$13::BYTEA AS bytes_val
    ,$14::HSTORE AS hstore_val
    ,$15::TIMESTAMPTZ AS system_time_val
    ,$16::INET AS inet_val
    ,$17::TIMESTAMP AS timestamp_val
    ,$18::TIMESTAMPTZ AS timestamptz_val
    ,$19::DATE AS date_val
    ,$20::TIME AS time_val
    ,$21::MACADDR AS macaddr_val
    ,$22::POINT AS point_val
    ,$23::BOX AS box_val
    ,$24::PATH AS path_val
    ,$25::JSONB AS jsonb_val
    ,$26::UUID AS uuid_val
    ,$27::VARBIT AS varbit_val
    ,$28::NUMERIC AS decimal_val
"#;

pub(crate) async fn execute(pg_client: &PgClient) -> anyhow::Result<()> {
    let data = Data {
        bool_val: true,
        bool_array_val: vec![true, false],
        bool_option_some_val: Some(true),
        bool_option_none_val: None,
        char_val: 1,
        smallint_val: 2,
        int_val: 3,
        oid_val: 4,
        bigint_val: 5,
        real_val: 6.1,
        double_val: 7.1,
        text_val: "予定表～①💖ﾊﾝｶｸだ".to_string(),
        bytes_val: vec![240, 159, 146, 150],
        hstore_val: {
            let mut hstore_val = HashMap::new();
            hstore_val.insert("key".to_string(), Some("value".to_string()));
            hstore_val
        },
        system_time_val: SystemTime::UNIX_EPOCH, // SystemTime::now()は精度の差で一致しないが使える。
        inet_val: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        timestamp_val: NaiveDateTime::from_str("2001-02-03T04:05:06").unwrap(),
        timestamptz_val: Utc.with_ymd_and_hms(2001, 2, 3, 4, 5, 6).unwrap(),
        date_val: NaiveDate::from_ymd_opt(2002, 3, 4).unwrap(),
        time_val: NaiveTime::from_hms_milli_opt(8, 59, 59, 100).unwrap(),
        macaddr_val: {
            let eui: Eui48 = [0x12, 0x34, 0x56, 0xAB, 0xCD, 0xEF];
            MacAddress::new(eui)
        },
        point_val: Point::new(1.234, 2.345),
        box_val: Rect::new(Coord { x: 0., y: 0. }, Coord { x: 10., y: 20. }),
        path_val: LineString(vec![Coord { x: 0., y: 0. }, Coord { x: 10., y: 20. }]),
        jsonb_val: {
            let json_data = r#"{
                "name" : "予定表～①💖ﾊﾝｶｸだ",
                "age" : 92233720368547758070
            }"#;
            serde_json::from_str(json_data).unwrap()
        },
        uuid_val: Uuid::new_v4(),
        varbit_val: {
            let mut varbit_val = BitVec::from_elem(10, false);
            varbit_val.set(2, true);
            varbit_val
        },
        decimal_val: Decimal::from_str("1.1").unwrap(),
    };

    let stmt = pg_client.prepare_cached(SQL).await?;
    let rows = pg_client
        .query(
            &stmt,
            &[
                &data.bool_val,
                &data.bool_array_val,
                &data.bool_option_some_val,
                &data.bool_option_none_val,
                &data.char_val,
                &data.smallint_val,
                &data.int_val,
                &data.oid_val,
                &data.bigint_val,
                &data.real_val,
                &data.double_val,
                &data.text_val,
                &data.bytes_val,
                &data.hstore_val,
                &data.system_time_val,
                &data.inet_val,
                &data.timestamp_val,
                &data.timestamptz_val,
                &data.date_val,
                &data.time_val,
                &data.macaddr_val,
                &data.point_val,
                &data.box_val,
                &data.path_val,
                &data.jsonb_val,
                &data.uuid_val,
                &data.varbit_val,
                &data.decimal_val,
            ],
        )
        .await?;

    let row = rows.first().unwrap();
    let res = Data {
        bool_val: row.get("bool_val"),
        bool_array_val: row.get("bool_array_val"),
        bool_option_some_val: row.get("bool_option_some_val"),
        bool_option_none_val: row.get("bool_option_none_val"),
        char_val: row.get("char_val"),
        smallint_val: row.get("smallint_val"),
        int_val: row.get("int_val"),
        oid_val: row.get("oid_val"),
        bigint_val: row.get("bigint_val"),
        real_val: row.get("real_val"),
        double_val: row.get("double_val"),
        text_val: row.get("text_val"),
        bytes_val: row.get("bytes_val"),
        hstore_val: row.get("hstore_val"),
        system_time_val: row.get("system_time_val"),
        inet_val: row.get("inet_val"),
        timestamp_val: row.get("timestamp_val"),
        timestamptz_val: row.get("timestamptz_val"),
        date_val: row.get("date_val"),
        time_val: row.get("time_val"),
        macaddr_val: row.get("macaddr_val"),
        point_val: row.get("point_val"),
        box_val: row.get("box_val"),
        path_val: row.get("path_val"),
        jsonb_val: row.get("jsonb_val"),
        uuid_val: row.get("uuid_val"),
        varbit_val: row.get("varbit_val"),
        decimal_val: row.get("decimal_val"),
    };
    assert_eq!(data, res);
    println!("{:?}", data);

    Ok(())
}
