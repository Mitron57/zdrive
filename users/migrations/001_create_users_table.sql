-- Migration: Create users table
-- Created: 2024-01-01

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    license_id VARCHAR(255) NOT NULL,
    driving_experience INTEGER NOT NULL,
    rating DOUBLE PRECISION NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index on email for faster lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Create index on created_at for sorting
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);

