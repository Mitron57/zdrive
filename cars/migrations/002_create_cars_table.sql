-- Migration: Create cars table
-- Created: 2024-01-01

CREATE TABLE IF NOT EXISTS cars (
    id UUID PRIMARY KEY,
    model VARCHAR(255) NOT NULL,
    license_plate VARCHAR(50) UNIQUE NOT NULL,
    iot_serial_number VARCHAR(255) UNIQUE NOT NULL,
    state VARCHAR(20) NOT NULL CHECK (state IN ('available', 'in_use', 'maintenance', 'reserved')),
    tariff_id UUID NOT NULL REFERENCES tariffs(id) ON DELETE RESTRICT,
    base_price DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_cars_license_plate ON cars(license_plate);
CREATE INDEX IF NOT EXISTS idx_cars_iot_serial ON cars(iot_serial_number);
CREATE INDEX IF NOT EXISTS idx_cars_tariff_id ON cars(tariff_id);
CREATE INDEX IF NOT EXISTS idx_cars_state ON cars(state);
CREATE INDEX IF NOT EXISTS idx_cars_created_at ON cars(created_at);

