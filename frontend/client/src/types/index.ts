export interface User {
  id: string;
  license_id: string;
  driving_experience: number;
  rating: number;
  email: string;
}

export interface Car {
  id: string;
  model: string;
  license_plate: string;
  state: string;
  tariff_id: string;
  base_price: number;
  price_per_minute?: number;
}

export interface CarData {
  car: Car;
  price_per_minute: number;
  telematics?: {
    fuel_level: number;
    location: {
      latitude: number;
      longitude: number;
    };
    door_status: string;
    speed: number;
    temperature: number;
    timestamp: string;
  };
}

export interface Trip {
  id: string;
  user_id: string;
  car_id: string;
  status: 'reserved' | 'active' | 'completed' | 'cancelled';
  started_at?: string;
  ended_at?: string;
  created_at: string;
}

export interface AuthResponse {
  token: string;
  user_id: string;
}

export interface RegisterRequest {
  license_id: string;
  driving_experience: number;
  rating: number;
  email: string;
  password: string;
}

export interface AuthRequest {
  email: string;
  password: string;
}

