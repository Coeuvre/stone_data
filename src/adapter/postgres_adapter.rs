extern crate postgres;

use std::io::prelude::*;

use self::postgres::Connection;
use self::postgres::types::{Type, ToSql, FromSql, SessionInfo, IsNull};
use self::postgres::error::Error as PostgresError;

use super::Adapter;
use attribute::{Attribute, Attributes};
use query::{Query, Filter, SortOrder};

macro_rules! accepts {
    ($($expected:pat),+) => (
        fn accepts(ty: &Type) -> bool {
            match *ty {
                $($expected)|+ => true,
                _ => false
            }
        }
    )
}

macro_rules! to_sql_checked {
    () => {
        fn to_sql_checked(&self, ty: &Type, out: &mut ::std::io::Write, ctx: &SessionInfo) -> Result<IsNull, PostgresError> {
            if !<Self as ToSql>::accepts(ty) {
                return Err(PostgresError::WrongType(ty.clone()));
            }
            self.to_sql(ty, out, ctx)
        }
    }
}

macro_rules! accepts_for_attribute {
    () => {
        accepts!(
            // Bool
            Type::Bool,
            // I8
            Type::Char,
            // I16
            Type::Int2,
            // I32
            Type::Int4,
            // I64
            Type::Int8,
            // F32
            Type::Float4,
            // F64
            Type::Float8,
            // String
            Type::Varchar, Type::Text, Type::Bpchar, Type::Name,
            // Postgres Enum as String
            Type::Other(_)
        );
    }
}

pub struct PostgresAdapter {
    conn: Connection,
}

impl PostgresAdapter {
    pub fn new(conn: Connection) -> PostgresAdapter {
        PostgresAdapter {
            conn: conn,
        }
    }

    pub fn query_raw(&self, sql: &str, params: &[&Attribute]) -> Option<Vec<Attributes>> {
        let stmt = self.conn.prepare(&sql).unwrap();
        let params: Vec<&ToSql> = params.into_iter().map(|param| *param as &ToSql).collect();
        let rows = stmt.query(params.as_slice()).unwrap();

        let mut attributes = vec![];

        for row in rows {
            attributes.push(row.columns().iter().fold(Attributes::new(), |mut attributes, column| {
                if let Some(attribute) = row.get_opt::<&str, Attribute>(column.name()).ok() {
                    attributes.insert(column.name().to_string(), attribute);
                }

                attributes
            }));
        }

        Some(attributes)
    }
}

impl ToSql for Attribute {
    to_sql_checked!();

    fn to_sql<W: Write+?Sized>(&self, ty: &Type, w: &mut W, ctx: &SessionInfo) -> Result<IsNull, PostgresError> {
        match *self {
            Attribute::Bool(ref attribute) => attribute.to_sql(ty, w, ctx),
            Attribute::I8(ref attribute) => attribute.to_sql(ty, w, ctx),
            Attribute::I16(ref attribute) => attribute.to_sql(ty, w, ctx),
            Attribute::I32(ref attribute) => attribute.to_sql(ty, w, ctx),
            Attribute::I64(ref attribute) => attribute.to_sql(ty, w, ctx),
            Attribute::F32(ref attribute) => attribute.to_sql(ty, w, ctx),
            Attribute::F64(ref attribute) => attribute.to_sql(ty, w, ctx),
            Attribute::String(ref attribute) => attribute.to_sql(ty, w, ctx),
        }
    }

    accepts_for_attribute!();
}

impl FromSql for Attribute {
    fn from_sql<R: Read>(ty: &Type, raw: &mut R, ctx: &SessionInfo) -> Result<Self, PostgresError> {
        match *ty {
            Type::Bool => Ok(Attribute::Bool(Some(try!(FromSql::from_sql(ty, raw, ctx))))),
            Type::Char => Ok(Attribute::I8(Some(try!(FromSql::from_sql(ty, raw, ctx))))),
            Type::Int2 => Ok(Attribute::I16(Some(try!(FromSql::from_sql(ty, raw, ctx))))),
            Type::Int4 => Ok(Attribute::I32(Some(try!(FromSql::from_sql(ty, raw, ctx))))),
            Type::Int8 => Ok(Attribute::I64(Some(try!(FromSql::from_sql(ty, raw, ctx))))),
            Type::Float4 => Ok(Attribute::F32(Some(try!(FromSql::from_sql(ty, raw, ctx))))),
            Type::Float8 => Ok(Attribute::F64(Some(try!(FromSql::from_sql(ty, raw, ctx))))),
            Type::Varchar | Type::Text | Type::Bpchar | Type::Name | Type::Other(_)
                => Ok(Attribute::String(Some(try!(FromSql::from_sql(ty, raw, ctx))))),
            _ => Err(PostgresError::WrongType(ty.clone()))
        }
    }

    accepts_for_attribute!();
}

fn build_sql<'a>(query: &'a Query) -> (String, Vec<&'a Attribute>) {
    let mut params = vec![];
    let mut sql = format!("SELECT {} FROM {}", match query.fields {
        None => "*".to_string(),
        Some(ref fields) => fields.iter().fold(String::new(), |mut fields, field| {
            if fields.len () > 0 {
                fields.push(',');
            }
            fields.push_str(field);
            fields
        })
    }, query.table.unwrap());

    if let Some(ref filter) = query.filter {
        sql.push_str(" WHERE ");
        build_filter(&mut sql, &mut params, filter);
    }

    if let Some(ref sort) = query.sort {
        let order_str = sort.iter().fold(String::new(), |mut order_str, &(ref name, ref order)| {
            if order_str.len() > 0 {
                order_str.push(',');
            }
            order_str.push_str(format!("{} {}", name, match order {
                &SortOrder::ASC => "ASC",
                &SortOrder::DESC => "DESC",
            }).as_str());

            order_str
        });
        sql.push_str(format!(" ORDER BY {}", order_str).as_str());
    }

    if let Some(ref limit) = query.limit {
        sql.push_str(format!(" LIMIT {}", limit).as_str());
    }

    if let Some(ref offset) = query.offset {
        sql.push_str(format!(" OFFSET {}", offset).as_str());
    }

    (sql, params)
}

fn build_filter<'a>(sql: &mut String, params: &mut Vec<&'a Attribute>, filter: &Filter<'a>) {
    match filter {
        &Filter::IsNull(name) => sql.push_str(format!("{} IS NULL", name).as_str()),
        &Filter::Equal(name, attribute) => {
            params.push(attribute);
            sql.push_str(format!("{}=${}", name, params.len()).as_str());
        },
        &Filter::In(name, ref attributes) => {
            let (params_str, _) = attributes.iter().fold((String::new(), params), |(mut params_str, params), attribute| {
                if params_str.len() > 0 {
                    params_str.push(',')
                }
                params.push(attribute);
                params_str.push_str(format!("${}", params.len()).as_str());
                (params_str, params)
            });
            sql.push_str(format!("{} IN ({})", name, params_str).as_str());
        },
        &Filter::And(ref f1, ref f2) => {
            sql.push('(');
            build_filter(sql, params, f1);
            sql.push_str(") AND (");
            build_filter(sql, params, f2);
            sql.push(')');
        },
        &Filter::Or(ref f1, ref f2) => {
            sql.push('(');
            build_filter(sql, params, f1);
            sql.push_str(") OR (");
            build_filter(sql, params, f2);
            sql.push(')');
        },
    }
}

impl Adapter for PostgresAdapter {
    fn query(&self, query: &Query) -> Option<Vec<Attributes>> {
        let (sql, params) = build_sql(query);
        self.query_raw(sql.as_str(), &params)
    }
}

#[cfg(test)]
mod tests {
    extern crate env_logger;

    use super::*;
    use super::postgres::{Connection, SslMode};

    use adapter::Adapter;
    use query::{Query, SortOrder};

    model! {
        User {
            type: "cheezmall.user",
            primary_key: "user_id",
            attributes: {
                "name": String,
                "email": String,
                "telephone": String,
                "password": String,
                "remember_token": String,
            },
            relationships: {},
        }
    }

    const DB_CONNECTION_URL: &'static str = "";

    #[test]
    fn test_connection() {
        env_logger::init().unwrap();
        let conn = Connection::connect(DB_CONNECTION_URL, &SslMode::None).unwrap();
        let adapter = PostgresAdapter::new(conn);
        let user_id = 10001.into();
        let name = "admin".to_string().into();
        let query = Query::table(User.ty).select(vec!["user_id", "name"])
                        .filter("user_id").eq(&user_id).and("name").eq(&name).and("telephone").is_null()
                        .order_by("user_id", SortOrder::ASC).limit(1);
        adapter.query(&query);
    }
}
