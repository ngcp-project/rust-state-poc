-- 0003_create_mission_stages_table.sql

-- mission_stage table
CREATE TABLE IF NOT EXISTS mission_stage (
  stage_id SERIAL PRIMARY KEY,
  vehicle_name VARCHAR(255) NOT NULL,
  search_area TEXT[] NOT NULL,       
  stage_name VARCHAR(255) NOT NULL,
  target_coordinate JSONB NOT NULL, 
  CONSTRAINT fk_vehicle_name
    FOREIGN KEY (vehicle_name)
    REFERENCES vehicles (vehicle_name)
);