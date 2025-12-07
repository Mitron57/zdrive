CREATE TABLE IF NOT EXISTS payments (
    id UUID PRIMARY KEY,
    trip_id UUID NOT NULL,
    user_id UUID NOT NULL,
    amount DOUBLE PRECISION NOT NULL CHECK (amount > 0),
    status VARCHAR(20) NOT NULL CHECK (status IN ('pending', 'paid', 'failed', 'cancelled')),
    bank_reference VARCHAR(255),
    qr_code_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    paid_at TIMESTAMP WITH TIME ZONE
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_payments_trip_id ON payments(trip_id);
CREATE INDEX IF NOT EXISTS idx_payments_user_id ON payments(user_id);
CREATE INDEX IF NOT EXISTS idx_payments_status ON payments(status);
CREATE INDEX IF NOT EXISTS idx_payments_created_at ON payments(created_at);

