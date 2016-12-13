//! SCRUD definition for DAO objects.

use rusqlite::Connection;
use super::super::error::Error;

pub trait Dao<T : Dao<T>> {

	/// The 'S' in SCRUD. Ensures the schema is set up correctly.
	fn schema(conn: Connection) -> Result<(), Error>;

	/// The 'C' in SCRUD. Creates a new entry for the given data.
	fn create(&self, conn: Connection) -> Result<T, Error>;

	/// The 'R' in SCRUD. Reads a single item by id.
	fn read(&self, conn: Connection) -> Result<T, Error>;

	/// The 'U' in SCRUD. Updates a single item.
	fn update(&self, conn: Connection) -> Result<T, Error>;

	/// The 'D' in SCRUD. Removes an entry by id.
	fn delete(&self, conn: Connection) -> Result<T, Error>;

}
