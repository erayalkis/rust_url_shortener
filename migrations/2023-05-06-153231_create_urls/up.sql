-- Your SQL goes here

CREATE TABLE urls (
  id SERIAL PRIMARY KEY,
  shortened_url VARCHAR NOT NULL,
  redirect_to VARCHAR NOT NULL
)