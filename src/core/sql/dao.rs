//! SCRUD definition for DAO objects.

use rusqlite::Connection;
use std::error::Error;

pub trait Dao<T : Dao<T>> {

	/// The 'S' in SCRUD. Ensures the schema is set up correctly.
	fn schema<E: Error>(conn: Connection) -> Result<(), &'static E>;

	/// The 'C' in SCRUD. Creates a new entry for the given data.
	fn create<E: Error>(&self, conn: Connection) -> Result<T, &'static E>;

	/// The 'R' in SCRUD. Reads a single item by id.
	fn read<E: Error>(&self, conn: Connection) -> Result<T, &'static E>;

	/// The 'U' in SCRUD. Updates a single item.
	fn update<E: Error>(&self, conn: Connection) -> Result<T, &'static E>;

	/// The 'D' in SCRUD. Removes an entry by id.
	fn delete<E: Error>(&self, conn: Connection) -> Result<T, &'static E>;

}
