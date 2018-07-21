//! This module contains the core calculation functionality used by the program. Later, it might be
//! split out into the individual command modules, since it really only concerns that. It's likely
//! to contain any ORM mappings that are necessary for maintaining the database.

mod chaos;

pub mod fate;
pub mod detail;
//pub mod sql;
pub mod error;
pub mod error_value_required;

pub use self::chaos::Chaos;
