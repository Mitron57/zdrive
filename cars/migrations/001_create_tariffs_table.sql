-- Migration: Create tariffs table
-- Created: 2024-01-01

CREATE TABLE IF NOT EXISTS tariffs (
    id UUID PRIMARY KEY,
    price_per_minute DOUBLE PRECISION NOT NULL,
    minimal_rating DOUBLE PRECISION NOT NULL,
    minimal_experience INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index on minimal_rating and minimal_experience for filtering
CREATE INDEX IF NOT EXISTS idx_tariffs_requirements ON tariffs(minimal_rating, minimal_experience);

