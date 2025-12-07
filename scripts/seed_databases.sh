#!/bin/bash

# Скрипт для заполнения всех баз данных тестовыми данными
# Использование: ./scripts/seed_databases.sh [--docker|--podman]
#   --docker: использовать docker exec для подключения к контейнерам
#   --podman: использовать podman exec для подключения к контейнерам
#   без параметров: использовать прямое подключение через psql

set -e

# Цвета для вывода
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Параметры подключения к базам данных
DB_USER="zdrive"
DB_PASSWORD="zdrive_password"
DB_HOST="localhost"

# Определяем режим работы
USE_DOCKER=false
USE_PODMAN=false
CONTAINER_CMD=""

if [ "$1" == "--docker" ]; then
    USE_DOCKER=true
    CONTAINER_CMD="docker exec -i"
elif [ "$1" == "--podman" ]; then
    USE_PODMAN=true
    CONTAINER_CMD="podman exec -i"
fi

# Функция для выполнения SQL запроса
execute_sql() {
    local db_name=$1
    local sql=$2
    
    if [ "$USE_DOCKER" = true ]; then
        local container_name="postgres-${db_name#zdrive_}"
        echo "$sql" | $CONTAINER_CMD $container_name psql -U $DB_USER -d $db_name
    elif [ "$USE_PODMAN" = true ]; then
        local container_name="postgres-${db_name#zdrive_}"
        echo "$sql" | $CONTAINER_CMD $container_name psql -U $DB_USER -d $db_name
    else
        PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -U $DB_USER -d $db_name -c "$sql"
    fi
}

# Функция для выполнения SQL из файла
execute_sql_file() {
    local db_name=$1
    local file=$2
    
    if [ "$USE_DOCKER" = true ] || [ "$USE_PODMAN" = true ]; then
        local container_name="postgres-${db_name#zdrive_}"
        $CONTAINER_CMD $container_name psql -U $DB_USER -d $db_name < "$file"
    else
        PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -U $DB_USER -d $db_name -f "$file"
    fi
}

echo -e "${GREEN}🚀 Начинаем заполнение баз данных тестовыми данными...${NC}\n"

# ============================================
# 1. USERS DATABASE
# ============================================
echo -e "${YELLOW}📝 Заполняем базу users...${NC}"

# Пароли: все тестовые пароли = "password123"
# Хеш bcrypt для "password123": $2a$10$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy
USERS_SQL="
INSERT INTO users (id, license_id, driving_experience, rating, email, password_hash, created_at, updated_at)
VALUES 
    ('550e8400-e29b-41d4-a716-446655440001', '77АА123456', 5, 4.5, 'user1@example.com', 
     '\$2a\$10\$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy', NOW(), NOW()),
    
    ('550e8400-e29b-41d4-a716-446655440002', '77ББ234567', 3, 4.0, 'user2@example.com',
     '\$2a\$10\$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy', NOW(), NOW()),
    
    ('550e8400-e29b-41d4-a716-446655440003', '77ВВ345678', 7, 4.8, 'user3@example.com',
     '\$2a\$10\$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy', NOW(), NOW()),
    
    ('550e8400-e29b-41d4-a716-446655440004', '77ГГ456789', 2, 3.5, 'user4@example.com',
     '\$2a\$10\$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy', NOW(), NOW()),
    
    ('550e8400-e29b-41d4-a716-446655440005', '77ДД567890', 10, 5.0, 'user5@example.com',
     '\$2a\$10\$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy', NOW(), NOW()),
    
    ('550e8400-e29b-41d4-a716-446655440006', '77ЕЕ678901', 1, 3.0, 'user6@example.com',
     '\$2a\$10\$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy', NOW(), NOW()),
    
    ('550e8400-e29b-41d4-a716-446655440007', '77ЖЖ789012', 8, 4.7, 'admin@example.com',
     '\$2a\$10\$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy', NOW(), NOW())
ON CONFLICT (id) DO NOTHING;
"

execute_sql "zdrive_users" "$USERS_SQL"
echo -e "${GREEN}✅ Users заполнены${NC}\n"

# ============================================
# 2. CARS DATABASE - Tariffs
# ============================================
echo -e "${YELLOW}📝 Заполняем тарифы...${NC}"

TARIFFS_SQL="
INSERT INTO tariffs (id, price_per_minute, minimal_rating, minimal_experience, created_at, updated_at)
VALUES 
    ('660e8400-e29b-41d4-a716-446655440001', 2.5, 0.0, 0, NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440002', 4.0, 3.0, 1, NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440003', 6.0, 4.0, 3, NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440004', 10.0, 4.5, 5, NOW(), NOW())
ON CONFLICT (id) DO NOTHING;
"

execute_sql "zdrive_cars" "$TARIFFS_SQL"
echo -e "${GREEN}✅ Тарифы заполнены${NC}\n"

# ============================================
# 3. CARS DATABASE - Cars
# ============================================
echo -e "${YELLOW}📝 Заполняем машины...${NC}"

CARS_SQL="
INSERT INTO cars (id, model, license_plate, iot_serial_number, state, tariff_id, base_price, created_at, updated_at)
VALUES 
    -- Эконом класс (tariff 1)
    ('770e8400-e29b-41d4-a716-446655440001', 'Lada Granta', 'А123БВ777', 'IOT-LADA-001', 'available', 
     '660e8400-e29b-41d4-a716-446655440001', 150.0, NOW(), NOW()),
    
    ('770e8400-e29b-41d4-a716-446655440002', 'Lada Vesta', 'В456ГД777', 'IOT-LADA-002', 'available',
     '660e8400-e29b-41d4-a716-446655440001', 180.0, NOW(), NOW()),
    
    ('770e8400-e29b-41d4-a716-446655440003', 'Lada Granta', 'С789ЕЖ777', 'IOT-LADA-003', 'available',
     '660e8400-e29b-41d4-a716-446655440001', 150.0, NOW(), NOW()),
    
    -- Комфорт класс (tariff 2)
    ('770e8400-e29b-41d4-a716-446655440004', 'Kia Rio', 'М123НП777', 'IOT-KIA-001', 'available',
     '660e8400-e29b-41d4-a716-446655440002', 250.0, NOW(), NOW()),
    
    ('770e8400-e29b-41d4-a716-446655440005', 'Hyundai Solaris', 'О456РС777', 'IOT-HYUNDAI-001', 'available',
     '660e8400-e29b-41d4-a716-446655440002', 270.0, NOW(), NOW()),
    
    ('770e8400-e29b-41d4-a716-446655440006', 'Kia Rio', 'Т789УФ777', 'IOT-KIA-002', 'in_use',
     '660e8400-e29b-41d4-a716-446655440002', 250.0, NOW(), NOW()),
    
    -- Бизнес класс (tariff 3)
    ('770e8400-e29b-41d4-a716-446655440007', 'Toyota Camry', 'Х123ЦЧ777', 'IOT-TOYOTA-001', 'available',
     '660e8400-e29b-41d4-a716-446655440003', 400.0, NOW(), NOW()),
    
    ('770e8400-e29b-41d4-a716-446655440008', 'Skoda Octavia', 'Ш456ЩЫ777', 'IOT-SKODA-001', 'available',
     '660e8400-e29b-41d4-a716-446655440003', 380.0, NOW(), NOW()),
    
    ('770e8400-e29b-41d4-a716-446655440009', 'Toyota Camry', 'Э789ЮЯ777', 'IOT-TOYOTA-002', 'reserved',
     '660e8400-e29b-41d4-a716-446655440003', 400.0, NOW(), NOW()),
    
    -- Премиум класс (tariff 4)
    ('770e8400-e29b-41d4-a716-446655440010', 'BMW 320i', 'А001БВ777', 'IOT-BMW-001', 'available',
     '660e8400-e29b-41d4-a716-446655440004', 600.0, NOW(), NOW()),
    
    ('770e8400-e29b-41d4-a716-446655440011', 'Mercedes-Benz C200', 'В002ГД777', 'IOT-MERCEDES-001', 'available',
     '660e8400-e29b-41d4-a716-446655440004', 650.0, NOW(), NOW()),
    
    ('770e8400-e29b-41d4-a716-446655440012', 'BMW 320i', 'С003ЕЖ777', 'IOT-BMW-002', 'maintenance',
     '660e8400-e29b-41d4-a716-446655440004', 600.0, NOW(), NOW())
ON CONFLICT (id) DO NOTHING;
"

execute_sql "zdrive_cars" "$CARS_SQL"
echo -e "${GREEN}✅ Машины заполнены${NC}\n"

# ============================================
# 4. TRIPS DATABASE
# ============================================
echo -e "${YELLOW}📝 Заполняем поездки...${NC}"

TRIPS_SQL="
INSERT INTO trips (id, user_id, car_id, status, started_at, ended_at, cancelled_at, created_at)
VALUES 
    -- Завершенные поездки
    ('880e8400-e29b-41d4-a716-446655440001', 
     '550e8400-e29b-41d4-a716-446655440001', 
     '770e8400-e29b-41d4-a716-446655440001',
     'completed', 
     NOW() - INTERVAL '2 hours', 
     NOW() - INTERVAL '1 hour', 
     NULL, 
     NOW() - INTERVAL '3 hours'),
    
    ('880e8400-e29b-41d4-a716-446655440002', 
     '550e8400-e29b-41d4-a716-446655440002', 
     '770e8400-e29b-41d4-a716-446655440004',
     'completed', 
     NOW() - INTERVAL '5 hours', 
     NOW() - INTERVAL '3 hours', 
     NULL, 
     NOW() - INTERVAL '6 hours'),
    
    -- Активные поездки
    ('880e8400-e29b-41d4-a716-446655440003', 
     '550e8400-e29b-41d4-a716-446655440003', 
     '770e8400-e29b-41d4-a716-446655440006',
     'active', 
     NOW() - INTERVAL '30 minutes', 
     NULL, 
     NULL, 
     NOW() - INTERVAL '35 minutes'),
    
    -- Зарезервированные поездки
    ('880e8400-e29b-41d4-a716-446655440004', 
     '550e8400-e29b-41d4-a716-446655440004', 
     '770e8400-e29b-41d4-a716-446655440009',
     'reserved', 
     NULL, 
     NULL, 
     NULL, 
     NOW() - INTERVAL '10 minutes'),
    
    -- Отмененные поездки
    ('880e8400-e29b-41d4-a716-446655440005', 
     '550e8400-e29b-41d4-a716-446655440005', 
     '770e8400-e29b-41d4-a716-446655440007',
     'cancelled', 
     NULL, 
     NULL, 
     NOW() - INTERVAL '1 hour', 
     NOW() - INTERVAL '2 hours')
ON CONFLICT (id) DO NOTHING;
"

execute_sql "zdrive_trips" "$TRIPS_SQL"
echo -e "${GREEN}✅ Поездки заполнены${NC}\n"

# ============================================
# 5. BILLING DATABASE - Payments
# ============================================
echo -e "${YELLOW}📝 Заполняем платежи...${NC}"

PAYMENTS_SQL="
INSERT INTO payments (id, trip_id, user_id, amount, status, bank_reference, qr_code_url, created_at, paid_at)
VALUES 
    -- Оплаченные платежи
    ('990e8400-e29b-41d4-a716-446655440001', 
     '880e8400-e29b-41d4-a716-446655440001', 
     '550e8400-e29b-41d4-a716-446655440001',
     450.0, 
     'paid', 
     'BANK-REF-001', 
     'https://example.com/qr/payment-001.png',
     NOW() - INTERVAL '1 hour', 
     NOW() - INTERVAL '55 minutes'),
    
    ('990e8400-e29b-41d4-a716-446655440002', 
     '880e8400-e29b-41d4-a716-446655440002', 
     '550e8400-e29b-41d4-a716-446655440002',
     720.0, 
     'paid', 
     'BANK-REF-002', 
     'https://example.com/qr/payment-002.png',
     NOW() - INTERVAL '3 hours', 
     NOW() - INTERVAL '2 hours 50 minutes'),
    
    -- Ожидающие оплаты
    ('990e8400-e29b-41d4-a716-446655440003', 
     '880e8400-e29b-41d4-a716-446655440003', 
     '550e8400-e29b-41d4-a716-446655440003',
     375.0, 
     'pending', 
     NULL, 
     'https://example.com/qr/payment-003.png',
     NOW() - INTERVAL '25 minutes', 
     NULL),
    
    -- Отмененные платежи
    ('990e8400-e29b-41d4-a716-446655440004', 
     '880e8400-e29b-41d4-a716-446655440005', 
     '550e8400-e29b-41d4-a716-446655440005',
     400.0, 
     'cancelled', 
     NULL, 
     NULL,
     NOW() - INTERVAL '1 hour 30 minutes', 
     NULL)
ON CONFLICT (id) DO NOTHING;
"

execute_sql "zdrive_billing" "$PAYMENTS_SQL"
echo -e "${GREEN}✅ Платежи заполнены${NC}\n"

# ============================================
# Итоговая статистика
# ============================================
echo -e "${GREEN}📊 Статистика заполненных данных:${NC}\n"

echo -e "${YELLOW}Users:${NC}"
execute_sql "zdrive_users" "SELECT COUNT(*) as total_users FROM users;"

echo -e "\n${YELLOW}Tariffs:${NC}"
execute_sql "zdrive_cars" "SELECT COUNT(*) as total_tariffs FROM tariffs;"

echo -e "\n${YELLOW}Cars:${NC}"
execute_sql "zdrive_cars" "SELECT COUNT(*) as total_cars, state, COUNT(*) FROM cars GROUP BY state;"

echo -e "\n${YELLOW}Trips:${NC}"
execute_sql "zdrive_trips" "SELECT COUNT(*) as total_trips, status, COUNT(*) FROM trips GROUP BY status;"

echo -e "\n${YELLOW}Payments:${NC}"
execute_sql "zdrive_billing" "SELECT COUNT(*) as total_payments, status, COUNT(*) FROM payments GROUP BY status;"

echo -e "\n${GREEN}✅ Все базы данных успешно заполнены тестовыми данными!${NC}\n"

echo -e "${YELLOW}📝 Тестовые учетные данные:${NC}"
echo -e "Email: user1@example.com - Password: password123"
echo -e "Email: user2@example.com - Password: password123"
echo -e "Email: admin@example.com - Password: password123"
echo -e "И так далее для всех пользователей...\n"

