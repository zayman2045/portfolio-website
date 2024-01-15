CREATE TABLE IF NOT EXISTS users (
  id          SERIAL PRIMARY KEY,
  username    VARCHAR(64) NOT NULL UNIQUE,
  password    VARCHAR(64) NOT NULL
);

INSERT INTO users (username, password) VALUES ('sample_user', 'sample_password');

CREATE TABLE IF NOT EXISTS missions (
  id      SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  title   VARCHAR(255) NOT NULL,
  content TEXT,
  FOREIGN KEY (user_id) REFERENCES users(id)
);

INSERT INTO missions (user_id, title, content) VALUES (1, 'sample_mission', 'sample_content');
