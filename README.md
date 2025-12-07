# ZDrive - Car Sharing Microservices Backend

Микросервисная архитектура для каршеринга на Rust с использованием луковой архитектуры.

## Архитектура

Проект состоит из следующих микросервисов:

- **users** (порт 3000) - Управление пользователями, аутентификация
- **cars** (порт 3001) - Управление машинами и тарифами
- **trips** (порт 3002) - Управление поездками
- **telematics** (порт 3003) - Телематика, IoT команды, сенсорные данные
- **billing** (порт 3004) - Платежи и QR-коды
- **dispatcher** (порт 8080) - API Gateway, объединяет все сервисы

## Запуск через Docker Compose

### Быстрый старт

```bash
# Запустить все сервисы
docker-compose up -d

# Просмотр логов
docker-compose logs -f

# Остановить все сервисы
docker-compose down

# Остановить и удалить volumes (удалит данные БД)
docker-compose down -v
```

### Доступ к сервисам

После запуска все сервисы доступны через Traefik:

- **Клиентский фронтенд**: http://localhost/client (или http://client.localhost)
- **Админский фронтенд**: http://localhost/admin (или http://admin.localhost)
- **API Gateway (Dispatcher)**: http://localhost/api (или http://api.localhost)
- **Traefik Dashboard**: http://localhost:8080

### Миграции базы данных

Перед использованием необходимо выполнить миграции для каждого сервиса:

```bash
# Users service
docker-compose exec users-service /app/users-service migrate

# Cars service
docker-compose exec cars-service /app/cars-service migrate

# Trips service
docker-compose exec trips-service /app/trips-service migrate

# Billing service
docker-compose exec billing-service /app/billing-service migrate
```

Или выполнить миграции локально перед запуском:

```bash
# Users
cd users && cargo run --bin migrate

# Cars
cd cars && cargo run --bin migrate

# Trips
cd trips && cargo run --bin migrate

# Billing
cd billing && cargo run --bin migrate
```

## Локальная разработка

### Требования

- Rust 1.91+
- PostgreSQL 16+
- Redis 7+
- RabbitMQ 3+

### Переменные окружения

Создайте `.env` файлы в каждой директории сервиса или используйте переменные окружения:

**users/.env:**
```
DATABASE_URL=postgresql://zdrive:zdrive_password@localhost:5432/zdrive
JWT_SECRET=your-secret-jwt-key
PORT=3000
```

**cars/.env:**
```
DATABASE_URL=postgresql://zdrive:zdrive_password@localhost:5432/zdrive
PORT=3001
```

**trips/.env:**
```
DATABASE_URL=postgresql://zdrive:zdrive_password@localhost:5432/zdrive
PORT=3002
```

**telematics/.env:**
```
RABBITMQ_URL=amqp://zdrive:zdrive_password@localhost:5672/%2f
REDIS_URL=redis://localhost:6379
PORT=3003
```

**billing/.env:**
```
DATABASE_URL=postgresql://zdrive:zdrive_password@localhost:5432/zdrive
PORT=3004
```

**dispatcher/.env:**
```
USERS_SERVICE_URL=http://localhost:3000
CARS_SERVICE_URL=http://localhost:3001
TRIPS_SERVICE_URL=http://localhost:3002
TELEMATICS_SERVICE_URL=http://localhost:3003
BILLING_SERVICE_URL=http://localhost:3004
PORT=8080
```

### Запуск локально

```bash
# В отдельных терминалах
cd users && cargo run
cd cars && cargo run
cd trips && cargo run
cd telematics && cargo run
cd billing && cargo run
cd dispatcher && cargo run
```

## Фронтенды

### Клиентский фронтенд

Доступен по адресу http://localhost/client

Функциональность:
- Регистрация и авторизация
- Поиск машины по ID
- Просмотр данных о машине (топливо, местоположение, статус)
- Начало поездки
- Завершение поездки (с созданием платежа)
- Отмена поездки
- Просмотр QR-кода для оплаты

### Админский фронтенд

Доступен по адресу http://localhost/admin

Функциональность:
- Авторизация
- Просмотр всех пользователей
- Просмотр всех машин
- Просмотр всех поездок
- Отправка команд на машины (открыть/закрыть двери, запустить/остановить двигатель)

## API Endpoints

Все API endpoints доступны через Dispatcher (API Gateway):

### Клиентские endpoints

- `POST /auth/register` - Регистрация
- `POST /auth/authenticate` - Аутентификация
- `POST /trips/start` - Начать поездку
- `PUT /trips/end` - Завершить поездку
- `PUT /trips/cancel` - Отменить поездку
- `GET /cars/{car_id}/data` - Данные о машине + телематика

### Админские endpoints

- `GET /admin/users` - Все пользователи
- `GET /admin/users/{id}` - Пользователь по ID
- `GET /admin/cars` - Все машины
- `GET /admin/cars/{id}` - Машина по ID
- `GET /admin/trips` - Все поездки
- `GET /admin/trips/{id}` - Поездка по ID
- `POST /admin/commands` - Отправить команду на машину

## OpenAPI спецификации

Каждый сервис имеет свою OpenAPI спецификацию:

- `users/openapi.yaml`
- `cars/openapi.yaml`
- `trips/openapi.yaml`
- `telematics/openapi.yaml`
- `billing/openapi.yaml`
- `dispatcher/openapi.yaml`

## Структура проекта

```
zdrive/
├── users/          # Users service
├── cars/           # Cars service
├── trips/          # Trips service
├── telematics/     # Telematics service
├── billing/        # Billing service
├── dispatcher/     # API Gateway
├── frontend/
│   ├── client/     # Клиентский фронтенд (React)
│   └── admin/      # Админский фронтенд (React)
├── docker-compose.yaml
└── Cargo.toml      # Workspace configuration
```

Каждый сервис следует луковой архитектуре:

- `domain/` - Доменные модели, интерфейсы, ошибки
- `application/` - Use cases (бизнес-логика)
- `infrastructure/` - Реализации (репозитории, внешние сервисы)
- `presentation/` - HTTP handlers, роутинг

## Тестирование

```bash
# Запустить все тесты
cargo test --workspace

# Тесты конкретного сервиса
cargo test --package users
```

## Лицензия

MIT

