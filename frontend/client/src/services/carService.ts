import api from './api';
import type { CarData, Car } from '../types';

export const carService = {
  async getCarData(carId: string): Promise<CarData> {
    const response = await api.get<CarData>(`/cars/${carId}/data`);
    return response.data;
  },
  async getAvailableCars(): Promise<Car[]> {
    const response = await api.get<Car[]>('/cars');
    return response.data;
  },
};

