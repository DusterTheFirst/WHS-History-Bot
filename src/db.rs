use crate::model::{Class, Group};
use log::{debug, error};
use rusqlite::{Connection, Result as SQLResult, Row, NO_PARAMS, types::Value, vtab::array};
use serenity::model::id::{ChannelId, RoleId};
use std::fmt::Display;
use std::path::Path;
use std::rc::Rc;

mod sql {
    pub mod schema {
        pub static GROUP: &str = include_str!("sql/schema/group.sql");
        pub static CLASS: &str = include_str!("sql/schema/class.sql");
    }

    pub mod query {
        pub static ALL_CLASSES: &str = include_str!("sql/query/all_classes.sql");
        pub static FILTER_CLASSES_BY_ROLES: &str = include_str!("sql/query/filter_classes_by_roles.sql");
        pub static SEARCH_CLASSES: &str = include_str!("sql/query/search_classes.sql");
        pub static ALL_GROUPS: &str = include_str!("sql/query/all_groups.sql");
        pub static SEARCH_GROUPS: &str = include_str!("sql/query/search_groups.sql");
    }
}

pub struct Database {
    connection: Connection,
}

fn asu64(string: String) -> u64 {
    string.parse().unwrap()
}

impl Database {
    pub fn open<P: AsRef<Path> + Display>(file: P) -> Database {
        match Connection::open(&file) {
            Ok(connection) => {
                debug!("Connected to database: {}", file);

                // ENABLE FOREIGN KEYS
                connection
                    .execute("PRAGMA foreign_keys = ON;", NO_PARAMS)
                    .unwrap();
                debug!("Enabled Foreign Keys on database: {}", file);

                array::load_module(&connection).unwrap();
                debug!("Enabled carray() on database: {}", file);

                // CREATE TABLES IF NOT EXIST
                connection.execute(sql::schema::GROUP, NO_PARAMS).unwrap();
                connection.execute(sql::schema::CLASS, NO_PARAMS).unwrap();

                return Database { connection };
            }
            Err(e) => {
                let message = format!("Error connecting to database {}: {:?}", file, e);
                error!("{}", message);
                panic!("{}", message);
            }
        }
    }

    /// A helper function to fetch all of the classes from the database
    pub fn get_all_classes(&self) -> SQLResult<Vec<Class>> {
        let mut stmt = self
            .connection
            .prepare_cached(sql::query::ALL_CLASSES)
            .unwrap();

        stmt.query_map(NO_PARAMS, |row| Self::get_class_with_group_from_row(row))
            .unwrap()
            .collect()
    }

    /// A helper function to fetch all of the classes from the database that fit a search term
    pub fn search_classes(&self, search_term: &str) -> SQLResult<Vec<Class>> {
        let mut stmt = self
            .connection
            .prepare_cached(sql::query::SEARCH_CLASSES)
            .unwrap();

        stmt.query_map(&[format!("%{}%", search_term)], |row| Self::get_class_with_group_from_row(row))
            .unwrap()
            .collect()
    }

        /// A helper function to fetch all of the classes from the database that fit a search term
    pub fn filter_classes_by_roles(&self, roles: &[RoleId]) -> SQLResult<Vec<Class>> {
        let mut stmt = self
            .connection
            .prepare_cached(sql::query::FILTER_CLASSES_BY_ROLES)
            .unwrap();

        let roles = roles.into_iter().map(|x| x.to_string()).map(Value::from).collect();
        let ptr = Rc::new(roles);

        stmt.query_map(&[&ptr], |row| Self::get_class_with_group_from_row(row))
            .unwrap()
            .collect()
    }

    /// A helper function to fetch all of the groups from the database
    pub fn get_all_groups(&self) -> SQLResult<Vec<Group>> {
        let mut stmt = self
            .connection
            .prepare_cached(sql::query::ALL_GROUPS)
            .unwrap();

        stmt.query_map(NO_PARAMS, |row| Self::get_group_from_row(row))
            .unwrap()
            .collect()
    }

    /// A helper function to fetch all of the classes from the database that fit a search term
    pub fn search_groups(&self, search_term: &str) -> SQLResult<Vec<Group>> {
        let mut stmt = self
            .connection
            .prepare_cached(sql::query::SEARCH_GROUPS)
            .unwrap();

        stmt.query_map(&[format!("%{}%", search_term)], |row| Self::get_group_from_row(row))
            .unwrap()
            .collect()
    }

    /// A helper function to transform a row using the following schema
    /// ```
    ///     `group`.`id`,
    ///     `group`.`name`,
    ///     `group`.`channel_group`,
    ///     `group`.`vc`
    /// ```
    fn get_group_from_row(row: &Row) -> SQLResult<Group> {
        Ok(Group {
            id: row.get(0)?,
            name: row.get(1)?,
            channel_group: ChannelId(asu64(row.get(2)?)),
            vc: ChannelId(asu64(row.get(3)?)),
        })
    }

    /// A helper function to transform a row using the following schema
    /// ```
    ///     `class`.`id`,
    ///     `class`.`name`,
    ///     `class`.`role`,
    ///     `class`.`channel`,
    ///     `group`.`id`,
    ///     `group`.`name`,
    ///     `group`.`channel_group`,
    ///     `group`.`vc`
    /// ```
    fn get_class_with_group_from_row(row: &Row) -> SQLResult<Class> {
        Ok(Class {
            id: row.get(0)?,
            name: row.get(1)?,
            role: RoleId(asu64(row.get(2)?)),
            channel: ChannelId(asu64(row.get(3)?)),
            group: if let Some(id) = row.get::<_, Option<u32>>(4)? {
                Some(Group {
                    id,
                    name: row.get(5)?,
                    channel_group: ChannelId(asu64(row.get(6)?)),
                    vc: ChannelId(asu64(row.get(7)?)),
                })
            } else {
                None
            },
        })
    }
}
