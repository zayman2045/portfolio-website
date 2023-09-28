CREATE TABLE IF NOT EXISTS users (
  id          SERIAL PRIMARY KEY,
  username    VARCHAR(64) NOT NULL UNIQUE,
  password    VARCHAR(64) NOT NULL,
  token       TEXT DEFAULT NULL
);

INSERT INTO users (username, password) VALUES ('fake_user', 'fake_password');