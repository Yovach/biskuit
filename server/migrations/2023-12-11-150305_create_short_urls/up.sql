-- Your SQL goes here

CREATE TABLE short_urls (
  id SERIAL PRIMARY KEY,
  url TEXT NOT NULL,
  token TEXT NOT NULL
)