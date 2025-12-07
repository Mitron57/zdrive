# Скрипты для работы с базой данных

## seed_databases.sh

Скрипт для заполнения всех баз данных тестовыми данными.

### Использование

#### Вариант 1: Прямое подключение (требует установленный psql)
```bash
./scripts/seed_databases.sh
```

#### Вариант 2: Через Docker
```bash
./scripts/seed_databases.sh --docker
```

#### Вариант 3: Через Podman
```bash
./scripts/seed_databases.sh --podman
```

### Что заполняется

1. **Users (zdrive_users)** - 7 тестовых пользователей
   - Все пароли: `password123`
   - Email: user1@example.com, user2@example.com, ..., admin@example.com

2. **Tariffs (zdrive_cars)** - 4 тарифа
   - Эконом (0.0 рейтинг, 0 опыта)
   - Комфорт (3.0 рейтинг, 1 год опыта)
   - Бизнес (4.0 рейтинг, 3 года опыта)
   - Премиум (4.5 рейтинг, 5 лет опыта)

3. **Cars (zdrive_cars)** - 12 машин
   - Эконом класс: Lada Granta, Lada Vesta
   - Комфорт класс: Kia Rio, Hyundai Solaris
   - Бизнес класс: Toyota Camry, Skoda Octavia
   - Премиум класс: BMW 320i, Mercedes-Benz C200
   - Разные состояния: available, in_use, reserved, maintenance

4. **Trips (zdrive_trips)** - 5 поездок
   - Завершенные поездки
   - Активные поездки
   - Зарезервированные поездки
   - Отмененные поездки

5. **Payments (zdrive_billing)** - 4 платежа
   - Оплаченные платежи
   - Ожидающие оплаты
   - Отмененные платежи

### Требования

- PostgreSQL клиент (psql) установлен локально, ИЛИ
- Docker/Podman с запущенными контейнерами postgres-*

### Примечания

- Скрипт использует `ON CONFLICT DO NOTHING`, поэтому его можно запускать несколько раз
- Все UUID фиксированные для консистентности данных между базами
- Пароли хешированы с помощью bcrypt

## seed_telematics_redis.sh

Скрипт для заполнения Redis тестовыми телематическими данными для всех автомобилей.

### Использование

#### Вариант 1: Прямое подключение (требует установленный redis-cli)
```bash
./scripts/seed_telematics_redis.sh
```

#### Вариант 2: Через Docker
```bash
./scripts/seed_telematics_redis.sh --docker
```

#### Вариант 3: Через Podman
```bash
./scripts/seed_telematics_redis.sh --podman
```

### Что заполняется

Телематические данные для всех 12 автомобилей из базы данных:
- Уровень топлива (fuel_level)
- Местоположение (latitude, longitude) - координаты Москвы
- Статус дверей (door_status: open/closed/locked)
- Скорость (speed) - 0.0 для стоящих машин, 45.5 для машины в использовании
- Температура (temperature)
- Временная метка (timestamp)

### Требования

- Redis клиент (redis-cli) установлен локально, ИЛИ
- Docker/Podman с запущенным контейнером redis
- Python 3 для генерации JSON

### Примечания

- Данные хранятся в Redis как hash set с ключом "sensors"
- VIN соответствует iot_serial_number из базы данных автомобилей
- Скрипт можно запускать несколько раз - данные будут обновлены

