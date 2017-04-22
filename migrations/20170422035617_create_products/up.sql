CREATE TABLE products (
	id SERIAL PRIMARY KEY,
	description TEXT NOT NULL,
	published BOOLEAN NOT NULL DEFAULT 'f',
	user_id INTEGER REFERENCES users
)

