ALTER TABLE users ALTER COLUMN updatedAt SET DEFAULT now();
CREATE OR REPLACE FUNCTION update_users_updatedAt()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updatedAt = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER tr_users_update_updatedAt
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_users_updatedAt();