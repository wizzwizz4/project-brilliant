use rustorm::{EntityManager, Pool, table::Table, error::DbError};
use core::hash::{SipHasher, Hash, Hasher};
use core::mem::drop;
use core::hint::unreachable_unchecked;
use std::path::Path;

pub struct DB(pub EntityManager);
#[derive(Debug)]
pub struct Error(Option<DbError>);

impl From<DbError> for Error {
    fn from(error: DbError) -> Error { Error(Some(error)) }
}

pub fn open_db(uri: &str) -> Result<DB, Error> {
    let em = Pool::new().em(uri)?;
    migrate_schema(hash_schema(em.get_all_tables()?))?;
    Ok(DB(em))
}

fn hash_schema(mut tables: Vec<Table>) -> u64 {
    use rustorm::{
        table::TableKey::*,
        column::{Capacity::*, ColumnConstraint::*, Literal::*},
    };

    tables.sort_unstable_by_key(Table::complete_name);
    let mut hasher = SipHasher::new();
    let h = &mut hasher;

    for table in tables {
        table.name.name.hash(h);
        let mut columns = table.columns.clone();
        columns.sort_unstable_by(|a, b| a.name.name.cmp(&b.name.name));
        for column in columns {
            column.name.name.hash(h);
            let spec = column.specification;
            spec.sql_type.name().hash(h);
            match spec.capacity {
                Some(x) => match x {
                    Limit(x) => {
                        "Limit".hash(h);
                        x.hash(h);
                    },
                    Range(x, y) => {
                        "Range".hash(h);
                        x.hash(h);
                        y.hash(h);
                    }
                },
                None => { "None".hash(h); }
            };
            let mut constraints = spec.constraints.clone();
            constraints.sort_unstable_by_key(|x| match x {
                NotNull => 0,
                DefaultValue(_) => 1,
                AutoIncrement => 2
            });
            for constraint in constraints {
                match constraint {
                    NotNull => "NotNull".hash(h),
                    DefaultValue(x) => {
                        "DefaultValue".hash(h);
                        match x {
                            Bool(x) => { "Bool".hash(h); x.hash(h) },
                            Null => "Null".hash(h),
                            Integer(x) => { "Integer".hash(h); x.hash(h) },
                            Double(x) => {
                                "Double".hash(h);
                                x.to_bits().hash(h)
                            },
                            UuidGenerateV4 => "UuidGenerateV4".hash(h),
                            Uuid(x) => {
                                "Uuid".hash(h);
                                x.as_bytes().hash(h)
                            },
                            String(x) => {"String".hash(h); x.hash(h) },
                            Blob(x) => {"Blob".hash(h); x.hash(h) },
                            CurrentTime => "CurrentTime".hash(h),
                            CurrentDate => "CurrentDate".hash(h),
                            CurrentTimestamp => "CurrentTimestamp".hash(h),
                            ArrayInt(x) => { "ArrayInt".hash(h); x.hash(h) },
                            ArrayFloat(x) => {
                                "ArrayFloat".hash(h);
                                for double in x {
                                    double.to_bits().hash(h);
                                }
                            },
                            ArrayString(x) => {
                                "ArrayString".hash(h);
                                x.hash(h)
                            }
                        }
                    },
                    AutoIncrement => "AutoIncrement".hash(h)
                }
            }
        }
        table.is_view.hash(h);
        let mut keys = table.table_key.clone();
        keys.sort_unstable_by(|a, b| match a {
            PrimaryKey(key) => (0, &key.name),
            UniqueKey(key) => (1, &key.name),
            Key(key) => (2, &key.name),
            ForeignKey(fk) => (3, &fk.name)
        }.cmp(&match b {
            PrimaryKey(key) => (0, &key.name),
            UniqueKey(key) => (1, &key.name),
            Key(key) => (2, &key.name),
            ForeignKey(fk) => (3, &fk.name)
        }));
        for tk in keys {
            if let ForeignKey(fk) = tk {
                "ForeignKey".hash(h);
                fk.name.hash(h);
                let mut columns = fk.columns.clone();
                columns.sort_unstable_by(|a, b| a.name.cmp(&b.name));
                for x in columns { x.name.hash(h) };
                fk.foreign_table.name.hash(h);
                let mut columns = fk.referred_columns.clone();
                columns.sort_unstable_by(|a, b| a.name.cmp(&b.name));
                for x in columns { x.name.hash(h) };
            } else {
                let key = match tk {
                    PrimaryKey(key) => { "PrimaryKey".hash(h); key },
                    UniqueKey(key) => { "UniqueKey".hash(h); key },
                    Key(key) => { "Key".hash(h); key },
                    ForeignKey(_) => unsafe { unreachable_unchecked() }
                };
                key.name.hash(h);
                let mut columns = key.columns.clone();
                columns.sort_unstable_by(|a, b| a.name.cmp(&b.name));
                for x in columns { x.name.hash(h); }
            };
        }
    };
    h.finish()
}

fn migrate_schema(from: u64) -> Result<(), Error> {
    if from == 2202906307356721367 {
        // empty
        // populate!
        Ok(())
    } else {
        // Unknown schema
        Err(Error(None))
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;
    use crate::*;

    fn temp_sqlite_uri() -> String {
        let mut s = String::from("sqlite://");
        let x = NamedTempFile::new().unwrap().into_temp_path();
        s.push_str(x.to_str().unwrap().as_ref());
        s
    }

    #[test]
    fn initialise_db() {
        let db = open_db(&temp_sqlite_uri()).unwrap();
    }
}
