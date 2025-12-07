# Многоэтапная сборка для оптимизации размера образа
FROM rust:1.91-slim as builder

# Устанавливаем зависимости для сборки
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Создаем рабочую директорию
WORKDIR /app

# Копируем все файлы и собираем
COPY cars/Cargo.toml cars/Cargo.lock* ./
COPY cars/src ./src
COPY cars/migrations ./migrations

# Собираем приложение
RUN cargo build --release

# Финальный образ
FROM debian:bookworm-slim

# Устанавливаем runtime зависимости
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Создаем непривилегированного пользователя
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Копируем скомпилированные бинарники из builder
COPY --from=builder /app/target/release/cars /app/cars-service
COPY --from=builder /app/target/release/migrate /app/migrate

# Копируем миграции и OpenAPI спецификацию
COPY cars/migrations ./migrations
COPY cars/openapi.yaml /app/openapi.yaml

# Меняем владельца файлов
RUN chown -R appuser:appuser /app

# Переключаемся на непривилегированного пользователя
USER appuser

# Открываем порт
EXPOSE 3001

# Переменные окружения по умолчанию (можно переопределить)
ENV PORT=3001
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Запускаем сервис
CMD ["./cars-service"]

