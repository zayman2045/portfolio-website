CREATE TABLE IF NOT EXISTS users (
  id          SERIAL PRIMARY KEY,
  username    VARCHAR(64) NOT NULL UNIQUE,
  password    VARCHAR(64) NOT NULL
);

INSERT INTO users (username, password) VALUES ('sample_user', 'sample_password');

CREATE TABLE IF NOT EXISTS missions (
  id      SERIAL PRIMARY KEY,
  title   VARCHAR(255) NOT NULL,
  content TEXT
);

INSERT INTO missions (title, content) VALUES ('sample_mission', 'sample_content');