-- Initialize the todo_app database with basic schema

-- Create todos table
CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create an index on completed status for faster queries
CREATE INDEX IF NOT EXISTS idx_todos_completed ON todos(completed);

-- Create an index on created_at for sorting
CREATE INDEX IF NOT EXISTS idx_todos_created_at ON todos(created_at);

-- Insert some sample data
-- INSERT INTO todos (title, description, completed) VALUES
--     ('Setup Rust project', 'Initialize the Rust todo application with Axum', true),
--     ('Add PostgreSQL integration', 'Connect the app to PostgreSQL database', false),
--     ('Implement CRUD operations', 'Create endpoints for todo management', false),
--     ('Add authentication', 'Implement user authentication and authorization', false),
--     ('Write tests', 'Add comprehensive test coverage', false);


-- Create a function to automatically update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger to automatically update updated_at
CREATE TRIGGER update_todos_updated_at 
    BEFORE UPDATE ON todos 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255),
    surname VARCHAR(255),
    fone VARCHAR(15),
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    activated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);

CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);


-- Create trigger to automatically update updated_at
CREATE TRIGGER update_users_updated_at 
    BEFORE UPDATE ON users 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- -- Create trigger to automatically update activated_at
-- CREATE TRIGGER update_users_activated_at
--     BEFORE UPDATE ON users 
--     FOR EACH ROW 
--     WHEN (OLD.active IS DISTINCT FROM NEW.active AND NEW.active = TRUE)
--     EXECUTE FUNCTION update_activated_at_column();
