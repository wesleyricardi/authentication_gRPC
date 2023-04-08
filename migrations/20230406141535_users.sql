ALTER TABLE users
ADD CONSTRAINT unique_id UNIQUE (id),
ADD CONSTRAINT unique_username UNIQUE (username),
ADD CONSTRAINT unique_email UNIQUE (email);

CREATE INDEX idx_id ON users (id);
CREATE INDEX idx_username ON users (username);