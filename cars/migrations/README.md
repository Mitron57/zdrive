# Миграции базы данных

Эта папка содержит SQL миграции для базы данных сервиса cars.

## Структура

Миграции должны быть пронумерованы и иметь формат:
```
XXX_description.sql
```

Где `XXX` - порядковый номер миграции (001, 002, 003...)

## Запуск миграций

### Используя sqlx-cli (рекомендуется)

1. Установите sqlx-cli:
```bash
cargo install sqlx-cli --features postgres
```

2. Настройте переменную окружения:
```bash
export DATABASE_URL=postgresql://user:password@localhost:5432/zdrive_cars
```

3. Запустите миграции:
```bash
sqlx migrate run
```

### Используя встроенную утилиту

```bash
cargo run --package cars --bin migrate
```

## Откат миграций

```bash
sqlx migrate revert
```

## Создание новой миграции

```bash
sqlx migrate add description_of_migration
```

