-- Add migration script here
-- Step 1: Create a new sequence for the id column
CREATE SEQUENCE IF NOT EXISTS users_id_seq;

-- Step 2: Set the default value of the id column to use the sequence
ALTER TABLE users
    ALTER COLUMN id SET DEFAULT nextval('users_id_seq');

-- Step 3: Optionally, set the sequence to start from the maximum current value
SELECT setval('users_id_seq', COALESCE((SELECT MAX(id) FROM users), 1), false);