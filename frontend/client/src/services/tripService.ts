import api from './api';
import type { Trip } from '../types';

export const tripService = {
  async startTrip(userId: string, carId: string): Promise<{ trip_id: string }> {
    const response = await api.post<{ trip_id: string }>('/trips/start', {
      user_id: userId,
      car_id: carId,
    });
    return response.data;
  },

  async activateTrip(tripId: string): Promise<{ trip_id: string; message: string }> {
    const response = await api.put<{ trip_id: string; message: string }>('/trips/activate', {
      trip_id: tripId,
    });
    return response.data;
  },

  async getActiveTrip(userId: string): Promise<{ trip: Trip | null }> {
    const response = await api.get<{ trip: Trip | null }>(`/trips/active?user_id=${userId}`);
    return response.data;
  },

  async endTrip(tripId: string): Promise<{
    trip_id: string;
    payment_id: string;
    qr_code_url: string;
  }> {
    const response = await api.put<{
      trip_id: string;
      payment_id: string;
      qr_code_url: string;
    }>('/trips/end', { trip_id: tripId });
    return response.data;
  },

  async cancelTrip(tripId: string): Promise<{ trip_id: string; message: string }> {
    const response = await api.put<{ trip_id: string; message: string }>('/trips/cancel', {
      trip_id: tripId,
    });
    return response.data;
  },

  async sendCarCommand(carId: string, commandType: 'open_door' | 'close_door' | 'start_engine' | 'stop_engine'): Promise<{ command_id: string; message: string }> {
    const response = await api.post<{ command_id: string; message: string }>(`/cars/${carId}/commands`, {
      command_type: commandType,
    });
    return response.data;
  },
};

