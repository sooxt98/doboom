-- Your SQL goes here
CREATE TABLE products (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  real_name VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  body TEXT NOT NULL,
  logo VARCHAR NOT NULL,
  owner VARCHAR NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
)
