import api from './api';
import type { User, Car, Trip, CommandRequest } from '../types';

export const adminService = {
  async getUsers(): Promise<User[]> {
    const response = await api.get<User[]>('/admin/users');
    return response.data;
  },

  async getUser(userId: string): Promise<User> {
    const response = await api.get<User>(`/admin/users/${userId}`);
    return response.data;
  },

  async getCars(): Promise<Car[]> {
    const response = await api.get<Car[]>('/admin/cars');
    return response.data;
  },

  async getCar(carId: string): Promise<Car> {
    const response = await api.get<Car>(`/admin/cars/${carId}`);
    return response.data;
  },

  async getTrips(): Promise<Trip[]> {
    const response = await api.get<Trip[]>('/admin/trips');
    return response.data;
  },

  async getTrip(tripId: string): Promise<Trip> {
    const response = await api.get<Trip>(`/admin/trips/${tripId}`);
    return response.data;
  },

  async sendCommand(command: CommandRequest): Promise<{ command_id: string }> {
    const response = await api.post<{ command_id: string }>('/admin/commands', command);
    return response.data;
  },

  async getCarData(carId: string): Promise<{
    car: Car;
    telematics?: {
      fuel_level: number;
      location: { latitude: number; longitude: number };
      door_status: string;
      speed: number;
      temperature: number;
      timestamp: string;
    };
  }> {
    const response = await api.get<{
      car: Car;
      telematics?: {
        fuel_level: number;
        location: { latitude: number; longitude: number };
        door_status: string;
        speed: number;
        temperature: number;
        timestamp: string;
      };
    }>(`/cars/${carId}/data`);
    return response.data;
  },
};

