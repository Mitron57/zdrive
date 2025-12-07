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

export interface AuthRequest {
  email: string;
  password: string;
}

export interface CommandRequest {
  car_id: string;
  command_type: 'open_door' | 'close_door' | 'start_engine' | 'stop_engine';
}

