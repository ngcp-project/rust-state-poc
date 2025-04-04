-- 0001_create_missions_table.sql

-- create the status enum type
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'status') THEN
    CREATE TYPE status AS ENUM ('Active', 'Inactive', 'Complete', 'Failed');
  END IF;
END$$;

-- missions table
CREATE TABLE IF NOT EXISTS missions (
  mission_name VARCHAR(255) PRIMARY KEY,
  keep_in_zones TEXT[] NOT NULL,
  keep_out_zones TEXT[] NOT NULL,
  status status
);
