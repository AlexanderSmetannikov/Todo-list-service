CREATE TABLE IF NOT EXISTS users (
	id integer PRIMARY KEY AUTOINCREMENT,
	username string UNIQUE,
  password string
);

CREATE TABLE IF NOT EXISTS tasks (
	task_id integer PRIMARY KEY AUTOINCREMENT,
	description text,
	done boolean
);

CREATE TABLE IF NOT EXISTS shared (
	id integer,
	task_id integer
);




