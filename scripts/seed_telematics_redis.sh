#!/bin/bash

# Скрипт для заполнения Redis тестовыми телематическими данными
# Использование: ./scripts/seed_telematics_redis.sh [--docker|--podman]
#   --docker: использовать docker exec для подключения к контейнеру Redis
#   --podman: использовать podman exec для подключения к контейнеру Redis
#   без параметров: использовать прямое подключение через redis-cli

set -e

# Цвета для вывода
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Параметры подключения
REDIS_HOST="localhost"
REDIS_PORT="6379"
REDIS_CONTAINER="redis"

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

# Функция для выполнения команды Redis
execute_redis() {
    local cmd=$1
    
    if [ "$USE_DOCKER" = true ] || [ "$USE_PODMAN" = true ]; then
        $CONTAINER_CMD $REDIS_CONTAINER redis-cli $cmd
    else
        redis-cli -h $REDIS_HOST -p $REDIS_PORT $cmd
    fi
}

# Функция для вставки телематических данных
insert_sensor_data() {
    local vin=$1
    local license_plate=$2
    local fuel_level=$3
    local lat=$4
    local lon=$5
    local door_status=$6
    local speed=$7
    local temp=$8
    
    # Генерируем timestamp в правильном ISO формате (RFC3339) с миллисекундами через Python
    local timestamp=$(python3 -c "from datetime import datetime, timezone; print(datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z')")
    
    # Вставляем данные в Redis как hash с отдельными полями
    # Структура: HSET sensors:{vin} field1 value1 field2 value2 ...
    python3 <<PYEOF
import sys
import subprocess

vin = "${vin}"
license_plate = "${license_plate}"
fuel_level = ${fuel_level}
lat = ${lat}
lon = ${lon}
door_status = "${door_status}"
speed = ${speed}
temp = ${temp}
timestamp = "${timestamp}"

# Определяем режим работы (передаем как строки из bash)
use_docker_str = "${USE_DOCKER}"
use_podman_str = "${USE_PODMAN}"
redis_host = "${REDIS_HOST}"
redis_port = ${REDIS_PORT}
redis_container = "${REDIS_CONTAINER}"

# Преобразуем строки в булевы значения
use_docker = use_docker_str.lower() == 'true'
use_podman = use_podman_str.lower() == 'true'

# Формируем ключ hash
hash_key = f"sensors:{vin}"

# Формируем команду HSET с отдельными полями
hset_args = [
    "HSET", hash_key,
    "vin", vin,
    "license_plate", license_plate,
    "fuel_level", str(fuel_level),
    "location_latitude", str(lat),
    "location_longitude", str(lon),
    "door_status", door_status,
    "speed", str(speed),
    "temperature", str(temp),
    "timestamp", timestamp
]

# Выполняем команду через redis-cli
if use_docker:
    cmd = ["docker", "exec", "-i", redis_container, "redis-cli"] + hset_args
elif use_podman:
    cmd = ["podman", "exec", "-i", redis_container, "redis-cli"] + hset_args
else:
    cmd = ["redis-cli", "-h", redis_host, "-p", str(redis_port)] + hset_args

proc = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
stdout, stderr = proc.communicate()

if proc.returncode != 0:
    error_msg = stderr.decode('utf-8', errors='ignore') if stderr else 'Unknown error'
    print(f"Error inserting data for {vin}: {error_msg}", file=sys.stderr)
    sys.exit(1)
PYEOF
    echo -e "${GREEN}✅ Данные для ${license_plate} (VIN: ${vin}) добавлены${NC}"
}

echo -e "${GREEN}🚀 Начинаем заполнение Redis телематическими данными...${NC}\n"

# Проверяем подключение к Redis
echo -e "${YELLOW}🔍 Проверяем подключение к Redis...${NC}"
if execute_redis PING | grep -q "PONG"; then
    echo -e "${GREEN}✅ Подключение к Redis успешно${NC}\n"
else
    echo -e "${RED}❌ Ошибка подключения к Redis${NC}"
    exit 1
fi

# Тестовые данные для автомобилей из seed_databases.sh
# Используем iot_serial_number как VIN

echo -e "${YELLOW}📝 Заполняем телематические данные...${NC}\n"

# Lada Granta - А123БВ777
insert_sensor_data \
    "IOT-LADA-001" \
    "А123БВ777" \
    85.5 \
    55.7558 \
    37.6173 \
    "closed" \
    0.0 \
    22.5

# Lada Vesta - В456ГД777
insert_sensor_data \
    "IOT-LADA-002" \
    "В456ГД777" \
    92.0 \
    55.7512 \
    37.6184 \
    "closed" \
    0.0 \
    21.8

# Lada Granta - С789ЕЖ777
insert_sensor_data \
    "IOT-LADA-003" \
    "С789ЕЖ777" \
    78.3 \
    55.7520 \
    37.6150 \
    "locked" \
    0.0 \
    23.1

# Kia Rio - М123НП777
insert_sensor_data \
    "IOT-KIA-001" \
    "М123НП777" \
    88.7 \
    55.7580 \
    37.6200 \
    "closed" \
    0.0 \
    20.5

# Hyundai Solaris - О456РС777
insert_sensor_data \
    "IOT-HYUNDAI-001" \
    "О456РС777" \
    95.2 \
    55.7540 \
    37.6190 \
    "closed" \
    0.0 \
    19.8

# Kia Rio - Т789УФ777 (в использовании)
insert_sensor_data \
    "IOT-KIA-002" \
    "Т789УФ777" \
    65.0 \
    55.7600 \
    37.6250 \
    "open" \
    45.5 \
    24.2

# Toyota Camry - Х123ЦЧ777
insert_sensor_data \
    "IOT-TOYOTA-001" \
    "Х123ЦЧ777" \
    90.1 \
    55.7500 \
    37.6100 \
    "closed" \
    0.0 \
    18.5

# Skoda Octavia - Ш456ЩЫ777
insert_sensor_data \
    "IOT-SKODA-001" \
    "Ш456ЩЫ777" \
    87.4 \
    55.7525 \
    37.6125 \
    "closed" \
    0.0 \
    19.2

# Toyota Camry - Э789ЮЯ777 (зарезервирована)
insert_sensor_data \
    "IOT-TOYOTA-002" \
    "Э789ЮЯ777" \
    82.6 \
    55.7560 \
    37.6140 \
    "locked" \
    0.0 \
    20.8

# BMW 320i - А001БВ777
insert_sensor_data \
    "IOT-BMW-001" \
    "А001БВ777" \
    93.8 \
    55.7570 \
    37.6160 \
    "closed" \
    0.0 \
    17.5

# Mercedes-Benz C200 - В002ГД777
insert_sensor_data \
    "IOT-MERCEDES-001" \
    "В002ГД777" \
    91.5 \
    55.7530 \
    37.6170 \
    "closed" \
    0.0 \
    18.0

# BMW 320i - С003ЕЖ777 (на обслуживании)
insert_sensor_data \
    "IOT-BMW-002" \
    "С003ЕЖ777" \
    45.0 \
    55.7550 \
    37.6180 \
    "locked" \
    0.0 \
    15.0

echo -e "\n${GREEN}✅ Все телематические данные успешно добавлены в Redis!${NC}\n"

# Выводим статистику
echo -e "${YELLOW}📊 Статистика:${NC}"
echo -e "Количество записей с телематическими данными:"
execute_redis "KEYS sensors:*" | wc -l | xargs echo

echo -e "\n${YELLOW}Список VIN в Redis:${NC}"
execute_redis "KEYS sensors:*" | sed 's/sensors://'

echo -e "\n${GREEN}✅ Готово!${NC}\n"

