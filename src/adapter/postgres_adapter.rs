extern crate postgres;

use std::io::prelude::*;

use self::postgres::Connection;
use self::postgres::types::{Type, ToSql, FromSql, SessionInfo, IsNull};
use self::postgres::error::Error as PostgresError;

use super::Adapter;
use attribute::{Attribute, Attributes};
use model::Model;

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

impl Adapter for PostgresAdapter {
    fn find(&self, model: &Model, id: &Attribute) -> Option<Attributes> {
        let sql = format!("SELECT * FROM {} WHERE {} = $1 ORDER BY {} LIMIT 1", model.ty, model.primary_key, model.primary_key);
        let stmt = self.conn.prepare(&sql).unwrap();
        let rows = stmt.query(&[id]).unwrap();
        let row = try_opt!(rows.iter().next());

        Some(row.columns().iter().fold(Attributes::new(), |mut attributes, column| {
            if let Some(attribute) = row.get_opt::<&str, Attribute>(column.name()).ok() {
                attributes.insert(column.name().to_string(), attribute);
            }

            attributes
        }))
    }

    fn find_all(&self, _: &Model) -> Vec<Attributes> {
        unimplemented!()
    }

    fn find_many(&self, _: &Model, _: &[&Attribute]) -> Vec<Attributes> {
        unimplemented!()
    }
}
