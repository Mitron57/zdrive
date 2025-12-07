-- Migration: Create trips table
-- Created: 2024-01-01

CREATE TABLE IF NOT EXISTS trips (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    car_id UUID NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('reserved', 'active', 'completed', 'cancelled')),
    started_at TIMESTAMP WITH TIME ZONE,
    ended_at TIMESTAMP WITH TIME ZONE,
    cancelled_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_trips_user_id ON trips(user_id);
CREATE INDEX IF NOT EXISTS idx_trips_car_id ON trips(car_id);
CREATE INDEX IF NOT EXISTS idx_trips_status ON trips(status);
CREATE INDEX IF NOT EXISTS idx_trips_created_at ON trips(created_at);

-- Composite index for finding active trips
CREATE INDEX IF NOT EXISTS idx_trips_user_active ON trips(user_id, status) WHERE status IN ('reserved', 'active');
CREATE INDEX IF NOT EXISTS idx_trips_car_active ON trips(car_id, status) WHERE status IN ('reserved', 'active');

