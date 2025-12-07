# Users Service

Микросервис для управления пользователями в системе каршеринга. Реализован с использованием луковой архитектуры (Onion Architecture).

## Функциональность

- ✅ Регистрация новых пользователей
- ✅ Авторизация пользователей (JWT токены)
- ✅ Получение данных пользователя
- ✅ Обновление данных пользователя

## Архитектура

Проект организован по принципу луковой архитектуры:

```
src/
├── domain/          # Ядро - модели, интерфейсы, ошибки
├── application/     # Бизнес-логика - use cases
├── infrastructure/  # Инфраструктура - репозитории, сервисы
└── presentation/   # HTTP API - handlers, routes
```

## API Endpoints

- `POST /users/register` - Регистрация нового пользователя
- `POST /users/authenticate` - Авторизация (получение JWT токена)
- `GET /users/:id` - Получение данных пользователя
- `PUT /users/:id` - Обновление данных пользователя

Подробная документация доступна в [OpenAPI спецификации](./openapi.yaml).

## Запуск локально

### Требования

- Rust 1.91+
- PostgreSQL 12+
- Cargo

### Настройка

1. Создайте файл `.env`:
```bash
DATABASE_URL=postgresql://user:password@localhost:5432/zdrive_users
JWT_SECRET=your-secret-key-change-in-production
PORT=3000
```

2. Создайте базу данных:
```bash
createdb zdrive_users
```

3. Выполните миграции базы данных:
```bash
# Используя встроенную утилиту
cargo run --package users --bin migrate

# Или используя sqlx-cli (рекомендуется)
cargo install sqlx-cli --features postgres
sqlx migrate run --source users/migrations
```

4. Запустите сервис:
```bash
cargo run --package users
```

## Запуск с Docker

### Docker вручную

1. Соберите образ:
```bash
docker build -f users/Dockerfile -t users-service:latest ..
```

2. Запустите PostgreSQL:
```bash
docker run -d \
  --name postgres \
  -e POSTGRES_USER=zdrive \
  -e POSTGRES_PASSWORD=zdrive_password \
  -e POSTGRES_DB=zdrive_users \
  -p 5432:5432 \
  postgres:16-alpine
```

3. Запустите сервис:
```bash
docker run -d \
  --name users-service \
  -p 3000:3000 \
  -e DATABASE_URL=postgresql://zdrive:zdrive_password@host.docker.internal:5432/zdrive_users \
  -e JWT_SECRET=your-secret-key \
  users-service:latest
```

## Переменные окружения

| Переменная | Описание | Обязательная | По умолчанию |
|-----------|----------|--------------|--------------|
| `DATABASE_URL` | URL подключения к PostgreSQL | Да | - |
| `JWT_SECRET` | Секретный ключ для JWT токенов | Нет | `your-secret-key` |
| `PORT` | Порт для HTTP сервера | Нет | `3000` |
| `RUST_LOG` | Уровень логирования (trace, debug, info, warn, error) | Нет | `info` |
| `RUST_BACKTRACE` | Включить backtrace для ошибок | Нет | `1` |

## Примеры использования API

### Регистрация пользователя

```bash
curl -X POST http://localhost:3000/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "license_id": "DL123456",
    "driving_experience": 5,
    "rating": 4.5,
    "email": "user@example.com",
    "password": "securePassword123"
  }'
```

### Авторизация

```bash
curl -X POST http://localhost:3000/users/authenticate \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "securePassword123"
  }'
```

### Получение пользователя

```bash
curl http://localhost:3000/users/{user_id}
```

### Обновление пользователя

```bash
curl -X PUT http://localhost:3000/users/{user_id} \
  -H "Content-Type: application/json" \
  -d '{
    "rating": 4.7,
    "driving_experience": 6
  }'
```

## OpenAPI спецификация

OpenAPI спецификация доступна в файле `openapi.yaml`. Вы можете использовать её для:

- Генерации клиентских SDK
- Импорта в Postman/Insomnia
- Просмотра в Swagger UI

Для просмотра в Swagger UI:
```bash
docker run -p 8080:8080 -e SWAGGER_JSON=/openapi.yaml -v $(pwd)/openapi.yaml:/openapi.yaml swaggerapi/swagger-ui
```

## Логирование

Сервис использует библиотеку `tracing` для структурированного логирования. Логи включают:

- **info** - Информационные сообщения (успешные операции, запуск сервиса)
- **warn** - Предупреждения (неудачные попытки авторизации, конфликты)
- **error** - Ошибки (внутренние ошибки сервера, проблемы с БД)

### Настройка уровня логирования

Уровень логирования настраивается через переменную окружения `RUST_LOG`:

```bash
# Показать все логи уровня info и выше
RUST_LOG=info cargo run --package users

# Показать все логи (включая debug и trace)
RUST_LOG=debug cargo run --package users

# Показать логи только для конкретного модуля
RUST_LOG=users::presentation=debug cargo run --package users
```

### Примеры логов

```
2024-01-15T10:30:00.123Z INFO users: Starting Users Service...
2024-01-15T10:30:00.456Z INFO users: Connecting to database...
2024-01-15T10:30:01.789Z INFO users: Database connection established
2024-01-15T10:30:01.890Z INFO users: Server running on http://0.0.0.0:3000
2024-01-15T10:30:15.234Z INFO users::presentation::handlers: Registering new user with email: user@example.com
2024-01-15T10:30:15.567Z INFO users::presentation::handlers: User registered successfully: 550e8400-e29b-41d4-a716-446655440000
```

## Миграции базы данных

Миграции находятся в папке `migrations/` и выполняются отдельной утилитой.

### Запуск миграций

```bash
# Используя встроенную утилиту
cargo run --package users --bin migrate

# Или используя sqlx-cli
sqlx migrate run --source users/migrations
```

### Создание новой миграции

Создайте новый файл в папке `migrations/` с форматом:
```
XXX_description.sql
```

Где `XXX` - порядковый номер (001, 002, 003...)

Пример:
```sql
-- migrations/002_add_user_phone.sql
ALTER TABLE users ADD COLUMN phone VARCHAR(20);
```

## Разработка

### Структура проекта

- `domain/` - Доменные модели, интерфейсы (traits), ошибки
- `application/` - Use cases (бизнес-логика)
- `infrastructure/` - Реализации репозиториев и сервисов
- `presentation/` - HTTP handlers и роутинг

### Запуск тестов

#### Unit-тесты (use cases)

```bash
cargo test --package users
```

Запускает все unit-тесты для use cases с использованием mock-реализаций. Не требует подключения к БД.

Тесты покрывают:
- Регистрацию пользователей (успешная регистрация, дублирующийся email)
- Авторизацию (успешная авторизация, неверные данные, несуществующий пользователь)
- Получение данных пользователя (успешное получение, пользователь не найден)
- Обновление данных пользователя (успешное обновление, конфликты email, пользователь не найден)

### Проверка кода

```bash
cargo clippy --package users
cargo fmt --package users
```

## Лицензия

MIT

