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

### Схема взаимодействия API

```mermaid
graph LR
    subgraph "Client APIs"
        CF_API[Client Frontend API<br/>RESTful HTTP/JSON]
        AF_API[Admin Frontend API<br/>RESTful HTTP/JSON]
    end
    
    subgraph "API Gateway"
        DG_API[Dispatcher API<br/>RESTful HTTP/JSON<br/>JWT Authentication]
    end
    
    subgraph "Microservice APIs"
        US_API[Users Service API<br/>RESTful HTTP/JSON<br/>Port 3000]
        CS_API[Cars Service API<br/>RESTful HTTP/JSON<br/>Port 3001]
        TS_API[Trips Service API<br/>RESTful HTTP/JSON<br/>Port 3002]
        TMS_API[Telematics Service API<br/>RESTful HTTP/JSON<br/>Port 3003]
        BS_API[Billing Service API<br/>RESTful HTTP/JSON<br/>Port 3004]
    end
    
    subgraph "Message Queue"
        RMQ_API[RabbitMQ<br/>AMQP Protocol<br/>Port 5672]
    end
    
    CF_API -->|HTTP/JSON| DG_API
    AF_API -->|HTTP/JSON| DG_API
    
    DG_API -->|HTTP/JSON| US_API
    DG_API -->|HTTP/JSON| CS_API
    DG_API -->|HTTP/JSON| TS_API
    DG_API -->|HTTP/JSON| TMS_API
    DG_API -->|HTTP/JSON| BS_API
    
    TMS_API -->|AMQP| RMQ_API
    
    style CF_API fill:#e1f5ff
    style AF_API fill:#e1f5ff
    style DG_API fill:#fff4e1
    style US_API fill:#e8f5e9
    style CS_API fill:#e8f5e9
    style TS_API fill:#e8f5e9
    style TMS_API fill:#e8f5e9
    style BS_API fill:#e8f5e9
    style RMQ_API fill:#fff9c4
```

### Архитектура контейнеров

```mermaid
graph TB
    subgraph "Network: zdrive-network"
        subgraph "Frontend Containers"
            CF_C[Client Frontend<br/>Nginx + React<br/>Port 80]
            AF_C[Admin Frontend<br/>Nginx + React<br/>Port 80]
        end
        
        subgraph "Reverse Proxy"
            TR_C[Traefik<br/>Reverse Proxy<br/>Ports 80, 8080]
        end
        
        subgraph "API Gateway Container"
            DG_C[Dispatcher Service<br/>Rust + Axum<br/>Port 8080]
        end
        
        subgraph "Microservice Containers"
            US_C[Users Service<br/>Rust + Axum<br/>Port 3000]
            CS_C[Cars Service<br/>Rust + Axum<br/>Port 3001]
            TS_C[Trips Service<br/>Rust + Axum<br/>Port 3002]
            TMS_C[Telematics Service<br/>Rust + Axum<br/>Port 3003]
            BS_C[Billing Service<br/>Rust + Axum<br/>Port 3004]
        end
        
        subgraph "Database Containers"
            PGU_C[(PostgreSQL 16<br/>Users DB<br/>Port 5432)]
            PGC_C[(PostgreSQL 16<br/>Cars DB<br/>Port 5432)]
            PGT_C[(PostgreSQL 16<br/>Trips DB<br/>Port 5432)]
            PGB_C[(PostgreSQL 16<br/>Billing DB<br/>Port 5432)]
        end
        
        subgraph "Cache Container"
            RD_C[(Redis 7<br/>Cache<br/>Port 6379)]
        end
        
        subgraph "Message Broker Container"
            RMQ_C[RabbitMQ 3<br/>Management<br/>Ports 5672, 15672]
        end
        
        subgraph "Storage Volumes"
            V1[postgres_users_data<br/>Volume]
            V2[postgres_cars_data<br/>Volume]
            V3[postgres_trips_data<br/>Volume]
            V4[postgres_billing_data<br/>Volume]
            V5[redis_data<br/>Volume]
            V6[rabbitmq_data<br/>Volume]
        end
    end
    
    CF_C --> TR_C
    AF_C --> TR_C
    TR_C --> DG_C
    DG_C --> US_C
    DG_C --> CS_C
    DG_C --> TS_C
    DG_C --> TMS_C
    DG_C --> BS_C
    
    US_C --> PGU_C
    CS_C --> PGC_C
    TS_C --> PGT_C
    BS_C --> PGB_C
    TMS_C --> RD_C
    
    TMS_C --> RMQ_C
    
    PGU_C --> V1
    PGC_C --> V2
    PGT_C --> V3
    PGB_C --> V4
    RD_C --> V5
    RMQ_C --> V6
    
    style CF_C fill:#e1f5ff
    style AF_C fill:#e1f5ff
    style TR_C fill:#ffebee
    style DG_C fill:#fff4e1
    style US_C fill:#e8f5e9
    style CS_C fill:#e8f5e9
    style TS_C fill:#e8f5e9
    style TMS_C fill:#e8f5e9
    style BS_C fill:#e8f5e9
    style PGU_C fill:#f3e5f5
    style PGC_C fill:#f3e5f5
    style PGT_C fill:#f3e5f5
    style PGB_C fill:#f3e5f5
    style RD_C fill:#f3e5f5
    style RMQ_C fill:#fff9c4
    style V1 fill:#e0e0e0
    style V2 fill:#e0e0e0
    style V3 fill:#e0e0e0
    style V4 fill:#e0e0e0
    style V5 fill:#e0e0e0
    style V6 fill:#e0e0e0
```

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

