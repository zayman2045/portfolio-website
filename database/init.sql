CREATE TABLE IF NOT EXISTS users (
  id          SERIAL PRIMARY KEY,
  username    VARCHAR(64) NOT NULL UNIQUE,
  password    VARCHAR(64) NOT NULL,
);

INSERT INTO users (username, password) VALUES ('sample_user', 'sample_password');