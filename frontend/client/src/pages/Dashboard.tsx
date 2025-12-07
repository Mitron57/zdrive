import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../store/authStore';
import { carService } from '../services/carService';
import { tripService } from '../services/tripService';
import type { CarData, Car } from '../types';

export default function Dashboard() {
  const [carId, setCarId] = useState('');
  const [carData, setCarData] = useState<CarData | null>(null);
  const [availableCars, setAvailableCars] = useState<Car[]>([]);
  const [loading, setLoading] = useState(false);
  const [loadingCars, setLoadingCars] = useState(false);
  const [error, setError] = useState('');
  const [activeTripId, setActiveTripId] = useState<string | null>(null);
  const [tripStatus, setTripStatus] = useState<'reserved' | 'active' | null>(null);
  const [tripStartTime, setTripStartTime] = useState<Date | null>(null);
  const [tripCarData, setTripCarData] = useState<CarData | null>(null);
  const [tripCarId, setTripCarId] = useState<string | null>(null);
  const [estimatedCost, setEstimatedCost] = useState<number | null>(null);
  const [sendingCommand, setSendingCommand] = useState(false);
  const [paymentQr, setPaymentQr] = useState<string | null>(null);
  const userId = useAuthStore((state) => state.userId);
  const clearAuth = useAuthStore((state) => state.clearAuth);
  const navigate = useNavigate();

  useEffect(() => {
    loadAvailableCars();
    if (userId) {
      loadActiveTrip();
    }
  }, [userId]);

  const handleGetCarData = async () => {
    if (!carId) {
      setError('Введите ID машины');
      return;
    }
    setLoading(true);
    setError('');
    try {
      const data = await carService.getCarData(carId);
      setCarData(data);
    } catch (err: any) {
      setError(err.response?.data?.error || 'Ошибка получения данных');
    } finally {
      setLoading(false);
    }
  };

  const loadActiveTrip = async () => {
    if (!userId) return;
    try {
      const response = await tripService.getActiveTrip(userId);
      if (response.trip) {
        setActiveTripId(response.trip.id);
        setTripStatus(response.trip.status as 'reserved' | 'active');
        setTripCarId(response.trip.car_id);
        if (response.trip.started_at) {
          setTripStartTime(new Date(response.trip.started_at));
          // Загружаем данные машины для расчета стоимости
          try {
            const carData = await carService.getCarData(response.trip.car_id);
            setTripCarData(carData);
            calculateEstimatedCost(carData, new Date(response.trip.started_at));
          } catch (err) {
            console.error('Ошибка загрузки данных машины:', err);
          }
        }
      }
    } catch (err: any) {
      console.error('Ошибка загрузки активной поездки:', err);
    }
  };

  const calculateEstimatedCost = (carData: CarData, startTime: Date) => {
    const now = new Date();
    const minutes = Math.max(1, Math.floor((now.getTime() - startTime.getTime()) / (1000 * 60)));
    const cost = (carData.price_per_minute * minutes) + carData.car.base_price;
    setEstimatedCost(cost);
  };

  // Обновляем расчет стоимости каждые 10 секунд для активной поездки
  useEffect(() => {
    if (tripStatus === 'active' && tripStartTime && tripCarData) {
      const updateCost = () => {
        if (!tripStartTime || !tripCarData) return;
        const now = new Date();
        const minutes = Math.max(1, (now.getTime() - tripStartTime.getTime()) / (1000 * 60));
        const cost = (tripCarData.price_per_minute * minutes) + tripCarData.car.base_price;
        setEstimatedCost(cost);
      };
      
      updateCost(); // Сразу обновляем
      const interval = setInterval(updateCost, 10000); // Обновляем каждые 10 секунд для более плавного отображения
      return () => clearInterval(interval);
    } else {
      // Сбрасываем стоимость, если поездка не активна
      setEstimatedCost(null);
    }
  }, [tripStatus, tripStartTime, tripCarData]);

  const handleReserveTrip = async () => {
    if (!carId || !userId) return;
    setLoading(true);
    setError('');
    try {
      const response = await tripService.startTrip(userId, carId);
      setActiveTripId(response.trip_id);
      setTripStatus('reserved');
      alert('Машина забронирована!');
    } catch (err: any) {
      setError(err.response?.data?.error || 'Ошибка бронирования');
    } finally {
      setLoading(false);
    }
  };

  const handleActivateTrip = async () => {
    if (!activeTripId || !carData) return;
    setLoading(true);
    setError('');
    try {
      await tripService.activateTrip(activeTripId);
      setTripStatus('active');
      const startTime = new Date();
      setTripStartTime(startTime);
      setTripCarData(carData);
      setTripCarId(carId);
      calculateEstimatedCost(carData, startTime);
      alert('Поездка начата!');
    } catch (err: any) {
      setError(err.response?.data?.error || 'Ошибка начала поездки');
    } finally {
      setLoading(false);
    }
  };

  const handleEndTrip = async () => {
    if (!activeTripId) return;
    setLoading(true);
    setError('');
    try {
      const response = await tripService.endTrip(activeTripId);
      setPaymentQr(response.qr_code_url);
      setActiveTripId(null);
      setTripStatus(null);
      setTripCarId(null);
      setTripCarData(null);
      setTripStartTime(null);
      setEstimatedCost(null);
      alert('Поездка завершена!');
    } catch (err: any) {
      setError(err.response?.data?.error || 'Ошибка завершения поездки');
    } finally {
      setLoading(false);
    }
  };

  const handleCancelTrip = async () => {
    if (!activeTripId) return;
    setLoading(true);
    setError('');
    try {
      await tripService.cancelTrip(activeTripId);
      setActiveTripId(null);
      setTripStatus(null);
      setTripCarId(null);
      setTripCarData(null);
      setTripStartTime(null);
      setEstimatedCost(null);
      alert('Поездка отменена');
    } catch (err: any) {
      setError(err.response?.data?.error || 'Ошибка отмены поездки');
    } finally {
      setLoading(false);
    }
  };

  const handleSendCommand = async (commandType: 'open_door' | 'close_door' | 'start_engine' | 'stop_engine') => {
    if (!tripCarId || tripStatus !== 'active') {
      alert('Команды доступны только во время активной поездки');
      return;
    }
    setSendingCommand(true);
    setError('');
    try {
      const response = await tripService.sendCarCommand(tripCarId, commandType);
      const commandNames: Record<string, string> = {
        'open_door': 'Открыть двери',
        'close_door': 'Закрыть двери',
        'start_engine': 'Запустить двигатель',
        'stop_engine': 'Заглушить двигатель',
      };
      alert(`${commandNames[commandType]}: команда отправлена успешно!`);
    } catch (err: any) {
      setError(err.response?.data?.error || 'Ошибка отправки команды');
    } finally {
      setSendingCommand(false);
    }
  };

  const loadAvailableCars = async () => {
    setLoadingCars(true);
    try {
      const cars = await carService.getAvailableCars();
      setAvailableCars(cars);
    } catch (err: any) {
      console.error('Ошибка загрузки машин:', err);
    } finally {
      setLoadingCars(false);
    }
  };

  const handleSelectCar = async (car: Car) => {
    setCarId(car.id);
    setError('');
    setLoading(true);
    try {
      const data = await carService.getCarData(car.id);
      setCarData(data);
    } catch (err: any) {
      setError(err.response?.data?.error || 'Ошибка получения данных');
    } finally {
      setLoading(false);
    }
  };

  const handleLogout = () => {
    clearAuth();
    navigate('/login');
  };

  return (
    <div style={{ maxWidth: '1200px', margin: '0 auto', padding: '20px' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
        <h1>Панель управления</h1>
        <button onClick={handleLogout} style={{ padding: '8px 16px', cursor: 'pointer' }}>
          Выйти
        </button>
      </div>

      {error && <div style={{ color: 'red', marginBottom: '15px', padding: '10px', backgroundColor: '#ffe6e6' }}>{error}</div>}

      <div style={{ marginBottom: '30px', padding: '20px', border: '1px solid #ddd', borderRadius: '8px' }}>
        <h2>Доступные машины</h2>
        {loadingCars ? (
          <p>Загрузка машин...</p>
        ) : availableCars.length === 0 ? (
          <p>Нет доступных машин</p>
        ) : (
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(250px, 1fr))', gap: '15px', marginBottom: '20px' }}>
            {availableCars.map((car) => (
              <div
                key={car.id}
                onClick={() => handleSelectCar(car)}
                style={{
                  padding: '15px',
                  border: '1px solid #ddd',
                  borderRadius: '8px',
                  cursor: 'pointer',
                  backgroundColor: carId === car.id ? '#e3f2fd' : 'white',
                  transition: 'background-color 0.2s',
                }}
                onMouseEnter={(e) => {
                  if (carId !== car.id) {
                    e.currentTarget.style.backgroundColor = '#f5f5f5';
                  }
                }}
                onMouseLeave={(e) => {
                  if (carId !== car.id) {
                    e.currentTarget.style.backgroundColor = 'white';
                  }
                }}
              >
                <h3 style={{ margin: '0 0 10px 0' }}>{car.model}</h3>
                <p style={{ margin: '5px 0' }}><strong>Госномер:</strong> {car.license_plate}</p>
                <p style={{ margin: '5px 0' }}><strong>Базовая цена:</strong> {car.base_price} ₽</p>
                {car.price_per_minute !== undefined && (
                  <p style={{ margin: '5px 0' }}><strong>Цена за минуту:</strong> {car.price_per_minute.toFixed(2)} ₽/мин</p>
                )}
                <p style={{ margin: '5px 0', color: '#28a745' }}><strong>Доступна</strong></p>
              </div>
            ))}
          </div>
        )}
      </div>

      <div style={{ marginBottom: '30px', padding: '20px', border: '1px solid #ddd', borderRadius: '8px' }}>
        <h2>Детали выбранной машины</h2>
        {!carData ? (
          <p style={{ color: '#666' }}>Выберите машину из списка выше</p>
        ) : (
          <div style={{ marginTop: '20px', padding: '15px', backgroundColor: '#f5f5f5', borderRadius: '4px' }}>
            <h3>{carData.car.model}</h3>
            <p><strong>Госномер:</strong> {carData.car.license_plate}</p>
            <p><strong>Состояние:</strong> {carData.car.state}</p>
            <p><strong>Базовая цена:</strong> {carData.car.base_price} ₽</p>
            <p><strong>Цена за минуту:</strong> {carData.price_per_minute.toFixed(2)} ₽/мин</p>
            
            {carData.telematics && (
              <div style={{ marginTop: '15px', padding: '10px', backgroundColor: 'white', borderRadius: '4px' }}>
                <h4>Телематика:</h4>
                <p><strong>Топливо:</strong> {carData.telematics.fuel_level}%</p>
                <p><strong>Местоположение:</strong> {carData.telematics.location.latitude.toFixed(6)}, {carData.telematics.location.longitude.toFixed(6)}</p>
                <p><strong>Двери:</strong> {carData.telematics.door_status === 'open' ? 'Открыты' : 'Закрыты'}</p>
                <p><strong>Скорость:</strong> {carData.telematics.speed} км/ч</p>
                <p><strong>Температура:</strong> {carData.telematics.temperature}°C</p>
              </div>
            )}

            {!activeTripId && (
              <button
                onClick={handleReserveTrip}
                disabled={loading}
                style={{
                  marginTop: '15px',
                  padding: '10px 20px',
                  backgroundColor: '#007bff',
                  color: 'white',
                  border: 'none',
                  borderRadius: '4px',
                  cursor: 'pointer',
                }}
              >
                Забронировать
              </button>
            )}
          </div>
        )}
      </div>

      {activeTripId && (
        <div style={{ marginBottom: '30px', padding: '20px', border: '1px solid #ffc107', borderRadius: '8px', backgroundColor: tripStatus === 'active' ? '#fff3cd' : '#e7f3ff' }}>
          <h2>{tripStatus === 'active' ? 'Активная поездка' : 'Забронированная поездка'}</h2>
          <p><strong>ID поездки:</strong> {activeTripId}</p>
          <p><strong>Статус:</strong> {tripStatus === 'active' ? 'Активна' : 'Забронирована'}</p>
          {tripStatus === 'active' && tripStartTime && (
            <div style={{ marginTop: '15px', padding: '10px', backgroundColor: 'white', borderRadius: '4px' }}>
              <p><strong>Время начала:</strong> {tripStartTime.toLocaleString()}</p>
              {estimatedCost !== null && tripStartTime && (
                <div style={{ marginTop: '10px' }}>
                  <p style={{ fontSize: '18px', fontWeight: 'bold', color: '#28a745' }}>
                    <strong>Примерная стоимость:</strong> {estimatedCost.toFixed(2)} ₽
                  </p>
                  <p style={{ fontSize: '14px', color: '#666', marginTop: '5px' }}>
                    Время поездки: {Math.floor((new Date().getTime() - tripStartTime.getTime()) / (1000 * 60))} мин
                  </p>
                </div>
              )}
              <div style={{ marginTop: '20px', padding: '15px', backgroundColor: '#f0f0f0', borderRadius: '4px' }}>
                <h3 style={{ margin: '0 0 15px 0' }}>Управление машиной</h3>
                <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '10px' }}>
                  <button
                    onClick={() => handleSendCommand('open_door')}
                    disabled={sendingCommand}
                    style={{
                      padding: '12px 20px',
                      backgroundColor: '#007bff',
                      color: 'white',
                      border: 'none',
                      borderRadius: '4px',
                      cursor: sendingCommand ? 'not-allowed' : 'pointer',
                      opacity: sendingCommand ? 0.6 : 1,
                    }}
                  >
                    🔓 Открыть двери
                  </button>
                  <button
                    onClick={() => handleSendCommand('close_door')}
                    disabled={sendingCommand}
                    style={{
                      padding: '12px 20px',
                      backgroundColor: '#6c757d',
                      color: 'white',
                      border: 'none',
                      borderRadius: '4px',
                      cursor: sendingCommand ? 'not-allowed' : 'pointer',
                      opacity: sendingCommand ? 0.6 : 1,
                    }}
                  >
                    🔒 Закрыть двери
                  </button>
                  <button
                    onClick={() => handleSendCommand('start_engine')}
                    disabled={sendingCommand}
                    style={{
                      padding: '12px 20px',
                      backgroundColor: '#28a745',
                      color: 'white',
                      border: 'none',
                      borderRadius: '4px',
                      cursor: sendingCommand ? 'not-allowed' : 'pointer',
                      opacity: sendingCommand ? 0.6 : 1,
                    }}
                  >
                    🚗 Запустить двигатель
                  </button>
                  <button
                    onClick={() => handleSendCommand('stop_engine')}
                    disabled={sendingCommand}
                    style={{
                      padding: '12px 20px',
                      backgroundColor: '#dc3545',
                      color: 'white',
                      border: 'none',
                      borderRadius: '4px',
                      cursor: sendingCommand ? 'not-allowed' : 'pointer',
                      opacity: sendingCommand ? 0.6 : 1,
                    }}
                  >
                    🛑 Заглушить двигатель
                  </button>
                </div>
              </div>
            </div>
          )}
          <div style={{ display: 'flex', gap: '10px', marginTop: '15px' }}>
            {tripStatus === 'reserved' && (
              <button
                onClick={handleActivateTrip}
                disabled={loading}
                style={{
                  padding: '10px 20px',
                  backgroundColor: '#28a745',
                  color: 'white',
                  border: 'none',
                  borderRadius: '4px',
                  cursor: 'pointer',
                }}
              >
                Начать поездку
              </button>
            )}
            {tripStatus === 'active' && (
              <>
                {estimatedCost !== null && (
                  <div style={{ 
                    padding: '10px 15px', 
                    backgroundColor: '#e7f3ff', 
                    borderRadius: '4px',
                    border: '1px solid #007bff',
                    marginRight: 'auto'
                  }}>
                    <p style={{ margin: 0, fontWeight: 'bold' }}>К оплате: {estimatedCost.toFixed(2)} ₽</p>
                  </div>
                )}
                <button
                  onClick={handleEndTrip}
                  disabled={loading}
                  style={{
                    padding: '10px 20px',
                    backgroundColor: '#28a745',
                    color: 'white',
                    border: 'none',
                    borderRadius: '4px',
                    cursor: 'pointer',
                  }}
                >
                  Завершить поездку
                </button>
              </>
            )}
            <button
              onClick={handleCancelTrip}
              disabled={loading}
              style={{
                padding: '10px 20px',
                backgroundColor: '#dc3545',
                color: 'white',
                border: 'none',
                borderRadius: '4px',
                cursor: 'pointer',
              }}
            >
              Отменить поездку
            </button>
          </div>
        </div>
      )}

      {paymentQr && (
        <div style={{ marginTop: '30px', padding: '20px', border: '1px solid #007bff', borderRadius: '8px' }}>
          <h2>Оплата</h2>
          <p>Отсканируйте QR-код для оплаты:</p>
          <img src={paymentQr} alt="QR Code" style={{ maxWidth: '300px', marginTop: '15px' }} />
        </div>
      )}
    </div>
  );
}

