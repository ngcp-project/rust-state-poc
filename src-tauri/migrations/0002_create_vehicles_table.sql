-- 0002_create_vehicles_table.sql

-- vehicles table
CREATE TABLE IF NOT EXISTS vehicles (
  mission_name VARCHAR(255) NOT NULL,
  vehicle_name VARCHAR(255) NOT NULL,
  current_stage_id INTEGER NOT NULL,
  PRIMARY KEY (mission_name, vehicle_name),
  CONSTRAINT fk-mission_name
    FOREIGN KEY (mission_name)
    REFERENCES missions (mission_name)
    -- ON DELETE CASCADE
    -- ON UPDATE CASCADE
    -- idk if those r necessary ^
);