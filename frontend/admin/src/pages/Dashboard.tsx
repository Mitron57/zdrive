import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../store/authStore';
import { adminService } from '../services/adminService';
import type { User, Car, Trip, CommandRequest } from '../types';

// Компонент для отображения строки машины с деталями
function CarRow({ car }: { car: Car }) {
  const [showDetails, setShowDetails] = useState(false);
  const [carData, setCarData] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  const handleShowDetails = async () => {
    if (showDetails) {
      setShowDetails(false);
      return;
    }
    setLoading(true);
    try {
      const data = await adminService.getCarData(car.id);
      setCarData(data);
      setShowDetails(true);
    } catch (err: any) {
      console.error('Ошибка загрузки данных машины:', err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <>
      <tr>
        <td style={{ padding: '10px', border: '1px solid #ddd' }}>{car.id}</td>
        <td style={{ padding: '10px', border: '1px solid #ddd' }}>{car.model}</td>
        <td style={{ padding: '10px', border: '1px solid #ddd' }}>{car.license_plate}</td>
        <td style={{ padding: '10px', border: '1px solid #ddd' }}>{car.state}</td>
        <td style={{ padding: '10px', border: '1px solid #ddd' }}>{car.base_price} ₽</td>
        <td style={{ padding: '10px', border: '1px solid #ddd' }}>
          <button
            onClick={handleShowDetails}
            disabled={loading}
            style={{
              padding: '5px 10px',
              backgroundColor: showDetails ? '#dc3545' : '#007bff',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer',
            }}
          >
            {loading ? 'Загрузка...' : showDetails ? 'Скрыть' : 'Детали'}
          </button>
        </td>
      </tr>
      {showDetails && carData && (
        <tr>
          <td colSpan={6} style={{ padding: '20px', border: '1px solid #ddd', backgroundColor: '#f9f9f9' }}>
            <div>
              <h3 style={{ marginTop: 0 }}>Детали машины: {carData.car.model}</h3>
              {carData.telematics ? (
                <div style={{ marginTop: '15px' }}>
                  <h4>Данные телематики:</h4>
                  <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: '10px', marginTop: '10px' }}>
                    <div>
                      <strong>Топливо:</strong> {carData.telematics.fuel_level}%
                    </div>
                    <div>
                      <strong>Скорость:</strong> {carData.telematics.speed} км/ч
                    </div>
                    <div>
                      <strong>Температура:</strong> {carData.telematics.temperature}°C
                    </div>
                    <div>
                      <strong>Двери:</strong> {carData.telematics.door_status === 'open' ? 'Открыты' : 'Закрыты'}
                    </div>
                    <div style={{ gridColumn: '1 / -1' }}>
                      <strong>Местоположение:</strong> {carData.telematics.location.latitude.toFixed(6)}, {carData.telematics.location.longitude.toFixed(6)}
                    </div>
                    <div style={{ gridColumn: '1 / -1' }}>
                      <strong>Время обновления:</strong> {new Date(carData.telematics.timestamp).toLocaleString()}
                    </div>
                  </div>
                </div>
              ) : (
                <p style={{ color: '#666' }}>Данные телематики недоступны</p>
              )}
            </div>
          </td>
        </tr>
      )}
    </>
  );
}

export default function Dashboard() {
  const [activeTab, setActiveTab] = useState<'users' | 'cars' | 'trips' | 'commands'>('users');
  const [users, setUsers] = useState<User[]>([]);
  const [cars, setCars] = useState<Car[]>([]);
  const [trips, setTrips] = useState<Trip[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [commandCarId, setCommandCarId] = useState('');
  const [commandType, setCommandType] = useState<CommandRequest['command_type']>('open_door');
  const clearAuth = useAuthStore((state) => state.clearAuth);
  const navigate = useNavigate();

  useEffect(() => {
    loadData();
  }, [activeTab]);

  const loadData = async () => {
    setLoading(true);
    setError('');
    try {
      switch (activeTab) {
        case 'users':
          const usersData = await adminService.getUsers();
          setUsers(usersData);
          break;
        case 'cars':
          const carsData = await adminService.getCars();
          setCars(carsData);
          break;
        case 'trips':
          const tripsData = await adminService.getTrips();
          setTrips(tripsData);
          break;
      }
    } catch (err: any) {
      setError(err.response?.data?.error || 'Ошибка загрузки данных');
    } finally {
      setLoading(false);
    }
  };

  const handleSendCommand = async () => {
    if (!commandCarId) {
      setError('Введите ID машины');
      return;
    }
    setLoading(true);
    setError('');
    try {
      await adminService.sendCommand({ car_id: commandCarId, command_type: commandType });
      alert('Команда отправлена!');
      setCommandCarId('');
    } catch (err: any) {
      setError(err.response?.data?.error || 'Ошибка отправки команды');
    } finally {
      setLoading(false);
    }
  };

  const handleLogout = () => {
    clearAuth();
    navigate('/login');
  };

  return (
    <div style={{ maxWidth: '1400px', margin: '0 auto', padding: '20px' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
        <h1>Админ-панель</h1>
        <button onClick={handleLogout} style={{ padding: '8px 16px', cursor: 'pointer' }}>
          Выйти
        </button>
      </div>

      <div style={{ display: 'flex', gap: '10px', marginBottom: '20px', borderBottom: '1px solid #ddd' }}>
        <button
          onClick={() => setActiveTab('users')}
          style={{
            padding: '10px 20px',
            border: 'none',
            backgroundColor: activeTab === 'users' ? '#007bff' : '#f0f0f0',
            color: activeTab === 'users' ? 'white' : 'black',
            cursor: 'pointer',
          }}
        >
          Пользователи
        </button>
        <button
          onClick={() => setActiveTab('cars')}
          style={{
            padding: '10px 20px',
            border: 'none',
            backgroundColor: activeTab === 'cars' ? '#007bff' : '#f0f0f0',
            color: activeTab === 'cars' ? 'white' : 'black',
            cursor: 'pointer',
          }}
        >
          Машины
        </button>
        <button
          onClick={() => setActiveTab('trips')}
          style={{
            padding: '10px 20px',
            border: 'none',
            backgroundColor: activeTab === 'trips' ? '#007bff' : '#f0f0f0',
            color: activeTab === 'trips' ? 'white' : 'black',
            cursor: 'pointer',
          }}
        >
          Поездки
        </button>
        <button
          onClick={() => setActiveTab('commands')}
          style={{
            padding: '10px 20px',
            border: 'none',
            backgroundColor: activeTab === 'commands' ? '#007bff' : '#f0f0f0',
            color: activeTab === 'commands' ? 'white' : 'black',
            cursor: 'pointer',
          }}
        >
          Команды
        </button>
      </div>

      {error && <div style={{ color: 'red', marginBottom: '15px', padding: '10px', backgroundColor: '#ffe6e6' }}>{error}</div>}

      {loading && <div style={{ marginBottom: '15px' }}>Загрузка...</div>}

      {activeTab === 'users' && (
        <div>
          <h2>Пользователи ({users.length})</h2>
          <table style={{ width: '100%', borderCollapse: 'collapse', marginTop: '15px' }}>
            <thead>
              <tr style={{ backgroundColor: '#f0f0f0' }}>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>ID</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Email</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>ВУ</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Опыт</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Рейтинг</th>
              </tr>
            </thead>
            <tbody>
              {users.map((user) => (
                <tr key={user.id}>
                  <td style={{ padding: '10px', border: '1px solid #ddd' }}>{user.id}</td>
                  <td style={{ padding: '10px', border: '1px solid #ddd' }}>{user.email}</td>
                  <td style={{ padding: '10px', border: '1px solid #ddd' }}>{user.license_id}</td>
                  <td style={{ padding: '10px', border: '1px solid #ddd' }}>{user.driving_experience} лет</td>
                  <td style={{ padding: '10px', border: '1px solid #ddd' }}>{user.rating}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {activeTab === 'cars' && (
        <div>
          <h2>Машины ({cars.length})</h2>
          <table style={{ width: '100%', borderCollapse: 'collapse', marginTop: '15px' }}>
            <thead>
              <tr style={{ backgroundColor: '#f0f0f0' }}>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>ID</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Модель</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Госномер</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Состояние</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Цена</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Действия</th>
              </tr>
            </thead>
            <tbody>
              {cars.map((car) => (
                <CarRow key={car.id} car={car} />
              ))}
            </tbody>
          </table>
        </div>
      )}

      {activeTab === 'trips' && (
        <div>
          <h2>Поездки ({trips.length})</h2>
          <table style={{ width: '100%', borderCollapse: 'collapse', marginTop: '15px' }}>
            <thead>
              <tr style={{ backgroundColor: '#f0f0f0' }}>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>ID</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Пользователь</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Машина</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Статус</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Начало</th>
                <th style={{ padding: '10px', textAlign: 'left', border: '1px solid #ddd' }}>Конец</th>
              </tr>
            </thead>
            <tbody>
              {trips.map((trip) => (
                <tr key={trip.id}>
                  <td style={{ padding: '10px', border: '1px solid #ddd' }}>{trip.id}</td>
                  <td style={{ padding: '10px', border: '1px solid #ddd' }}>{trip.user_id}</td>
                  <td style={{ padding: '10px', border: '1px solid #ddd' }}>{trip.car_id}</td>
                  <td style={{ padding: '10px', border: '1px solid #ddd' }}>{trip.status}</td>
                  <td style={{ padding: '10px', border: '1px solid #ddd' }}>
                    {trip.started_at ? new Date(trip.started_at).toLocaleString() : '-'}
                  </td>
                  <td style={{ padding: '10px', border: '1px solid #ddd' }}>
                    {trip.ended_at ? new Date(trip.ended_at).toLocaleString() : '-'}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {activeTab === 'commands' && (
        <div>
          <h2>Отправка команд</h2>
          <div style={{ marginTop: '20px', padding: '20px', border: '1px solid #ddd', borderRadius: '8px' }}>
            <div style={{ marginBottom: '15px' }}>
              <label>
                ID машины:
                <input
                  type="text"
                  value={commandCarId}
                  onChange={(e) => setCommandCarId(e.target.value)}
                  style={{ width: '100%', padding: '8px', marginTop: '5px' }}
                />
              </label>
            </div>
            <div style={{ marginBottom: '15px' }}>
              <label>
                Тип команды:
                <select
                  value={commandType}
                  onChange={(e) => setCommandType(e.target.value as CommandRequest['command_type'])}
                  style={{ width: '100%', padding: '8px', marginTop: '5px' }}
                >
                  <option value="open_door">Открыть двери</option>
                  <option value="close_door">Закрыть двери</option>
                  <option value="start_engine">Запустить двигатель</option>
                  <option value="stop_engine">Остановить двигатель</option>
                </select>
              </label>
            </div>
            <button
              onClick={handleSendCommand}
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
              Отправить команду
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

