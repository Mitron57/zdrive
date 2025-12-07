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
COPY telematics/Cargo.toml telematics/Cargo.lock* ./
COPY telematics/src ./src

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

# Копируем скомпилированный бинарник из builder
COPY --from=builder /app/target/release/telematics /app/telematics-service

# Копируем OpenAPI спецификацию (опционально)
COPY telematics/openapi.yaml /app/openapi.yaml

# Меняем владельца файлов
RUN chown -R appuser:appuser /app

# Переключаемся на непривилегированного пользователя
USER appuser

# Открываем порт
EXPOSE 3003

# Переменные окружения по умолчанию (можно переопределить)
ENV PORT=3003
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

# Запускаем сервис
CMD ["./telematics-service"]

