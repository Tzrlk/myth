//! Represents a scene in a [Game].

use time::Timespec;
use rusqlite::Connection;

use super::dao::Dao;
use super::game::Game;
use ::core::error::Error;
use ::core::error_value_required::ErrorValueRequired::new as required;

#[derive(Debug)]
struct Scene {
	id:           Option<i32>,
	game:         Option<Game>,
	desc:         Option<&'static str>,
	time_created: Option<Timespec>,
	time_updated: Option<Timespec>,
	version:      Option<i32>
}

impl Dao<Scene> for Scene {

	fn schema(conn: Connection) -> Result<(), E> {
		return conn.execute("CREATE TABLE schema ( \
			id              INTEGER PRIMARY KEY, \
			game_id         INTEGER NOT NULL, \
			desc            TEXT NOT NULL, \
			time_created    TEXT NOT NULL, \
			time_updated    TEXT NOT NULL, \
			version         INTEGER NOT NULL )\
		", &[]);
	}

	fn create(&self, conn: Connection) -> Result<Scene, Error> {

		let game = self.game.ok_or(required("game"))?;
		let game_id = game.id.ok_or(required("game_id"))?;
		let desc = self.desc.ok_or(required("desc"))?;

		let stmt = conn.prepare_cached("INSERT INTO Scene (\
			id, game_id, desc, created, updated, version ) \
			VALUES ( SEQ(), ?, ?, NOW(), NOW(), 1")?;

		let result = stmt.execute(&[ game_id, desc ]);
		return self.read(conn);

	}

	fn read(&self, conn: Connection) -> Result<Scene, Error> {

		let id = self.id.ok_or(required("id"))?;

		let stmt = conn.prepare_cached("\
			SELECT \
				id, \
				time_created, \
				time_updated, \
				version \
			FROM scene \
			WHERE id = ? \
		")?;

		return stmt.query_map(&[ id ], |row| {
			Scene {
				id:             Some(row.get(0)),
				game:           Some(Game { id: Some(row.get(1)) }),
				desc:           Some(row.get(2)),
				time_created:   Some(row.get(3)),
				time_updated:   Some(row.get(4)),
				version:        Some(row.get(5))
			}
		});

	}

	fn update(&self, conn: Connection) -> Result<Scene, Error> {

		let desc = self.desc.ok_or(required("desc"))?;
		let version = self.version.ok_or(required("version"))?;
		let id = self.id.ok_or(required("id"))?;

		let stmt = conn.prepare_cached("\
			UPDATE people ( desc, time_updated, version ) \
			VALUES ( ?, NOW(), ? ) \
			WHERE id = ? AND version = ?")?;

		stmt.execute(&[ desc, version + 1, id, version ]);
		return self.read(conn);

	}

	fn delete(&self, conn: Connection) -> Result<Scene, Error> {

		let id = self.id.ok_or(required("id"))?;

		let stmt = conn.prepare_cached("DELETE FROM Scene \
			WHERE id = ?")?;

		let existing = self.read(conn)?;
		stmt.execute(&[ id ])?;

		return existing;

	}

}
