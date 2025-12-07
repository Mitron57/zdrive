-- SQL запросы для вставки новых машин
-- Перед выполнением убедитесь, что в таблице tariffs есть записи
-- 
-- ВАЖНО: Замените 'YOUR-TARIFF-UUID-HERE' на реальные UUID тарифов из вашей базы данных
-- Чтобы получить список тарифов, выполните: SELECT id, price_per_minute, minimal_rating, minimal_experience FROM tariffs;

-- ============================================
-- ВАРИАНТ 1: Использование первого доступного тарифа (для тестирования)
-- ============================================

-- Эконом класс (Lada Granta, Lada Vesta)
INSERT INTO cars (id, model, license_plate, iot_serial_number, state, tariff_id, base_price, created_at, updated_at)
VALUES 
    (gen_random_uuid(), 'Lada Granta', 'А123БВ777', 'IOT-LADA-001', 'available', 
     (SELECT id FROM tariffs LIMIT 1), 150.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Lada Vesta', 'В456ГД777', 'IOT-LADA-002', 'available',
     (SELECT id FROM tariffs LIMIT 1), 180.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Lada Granta', 'С789ЕЖ777', 'IOT-LADA-003', 'available',
     (SELECT id FROM tariffs LIMIT 1), 150.0, NOW(), NOW());

-- Комфорт класс (Kia Rio, Hyundai Solaris)
INSERT INTO cars (id, model, license_plate, iot_serial_number, state, tariff_id, base_price, created_at, updated_at)
VALUES 
    (gen_random_uuid(), 'Kia Rio', 'М123НП777', 'IOT-KIA-001', 'available',
     (SELECT id FROM tariffs LIMIT 1), 250.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Hyundai Solaris', 'О456РС777', 'IOT-HYUNDAI-001', 'available',
     (SELECT id FROM tariffs LIMIT 1), 270.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Kia Rio', 'Т789УФ777', 'IOT-KIA-002', 'in_use',
     (SELECT id FROM tariffs LIMIT 1), 250.0, NOW(), NOW());

-- Бизнес класс (Toyota Camry, Skoda Octavia)
INSERT INTO cars (id, model, license_plate, iot_serial_number, state, tariff_id, base_price, created_at, updated_at)
VALUES 
    (gen_random_uuid(), 'Toyota Camry', 'Х123ЦЧ777', 'IOT-TOYOTA-001', 'available',
     (SELECT id FROM tariffs LIMIT 1), 400.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Skoda Octavia', 'Ш456ЩЫ777', 'IOT-SKODA-001', 'available',
     (SELECT id FROM tariffs LIMIT 1), 380.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Toyota Camry', 'Э789ЮЯ777', 'IOT-TOYOTA-002', 'reserved',
     (SELECT id FROM tariffs LIMIT 1), 400.0, NOW(), NOW());

-- Премиум класс (BMW 3 Series, Mercedes-Benz C-Class)
INSERT INTO cars (id, model, license_plate, iot_serial_number, state, tariff_id, base_price, created_at, updated_at)
VALUES 
    (gen_random_uuid(), 'BMW 320i', 'А001БВ777', 'IOT-BMW-001', 'available',
     (SELECT id FROM tariffs LIMIT 1), 600.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Mercedes-Benz C200', 'В002ГД777', 'IOT-MERCEDES-001', 'available',
     (SELECT id FROM tariffs LIMIT 1), 650.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'BMW 320i', 'С003ЕЖ777', 'IOT-BMW-002', 'maintenance',
     (SELECT id FROM tariffs LIMIT 1), 600.0, NOW(), NOW());

-- ============================================
-- ВАРИАНТ 2: Использование конкретных tariff_id (рекомендуется)
-- ============================================
-- Раскомментируйте и замените UUID на реальные значения из вашей базы

/*
-- Сначала получите список тарифов:
-- SELECT id, price_per_minute, minimal_rating, minimal_experience FROM tariffs;

-- Затем используйте конкретные UUID:
INSERT INTO cars (id, model, license_plate, iot_serial_number, state, tariff_id, base_price, created_at, updated_at)
VALUES 
    (gen_random_uuid(), 'Lada Granta', 'А123БВ777', 'IOT-LADA-001', 'available', 
     'YOUR-TARIFF-UUID-HERE'::uuid, 150.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Lada Vesta', 'В456ГД777', 'IOT-LADA-002', 'available',
     'YOUR-TARIFF-UUID-HERE'::uuid, 180.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Kia Rio', 'М123НП777', 'IOT-KIA-001', 'available',
     'YOUR-TARIFF-UUID-HERE'::uuid, 250.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Hyundai Solaris', 'О456РС777', 'IOT-HYUNDAI-001', 'available',
     'YOUR-TARIFF-UUID-HERE'::uuid, 270.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Toyota Camry', 'Х123ЦЧ777', 'IOT-TOYOTA-001', 'available',
     'YOUR-TARIFF-UUID-HERE'::uuid, 400.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Skoda Octavia', 'Ш456ЩЫ777', 'IOT-SKODA-001', 'available',
     'YOUR-TARIFF-UUID-HERE'::uuid, 380.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'BMW 320i', 'А001БВ777', 'IOT-BMW-001', 'available',
     'YOUR-TARIFF-UUID-HERE'::uuid, 600.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Mercedes-Benz C200', 'В002ГД777', 'IOT-MERCEDES-001', 'available',
     'YOUR-TARIFF-UUID-HERE'::uuid, 650.0, NOW(), NOW());
*/

-- ============================================
-- Проверка вставленных данных
-- ============================================
-- SELECT c.id, c.model, c.license_plate, c.state, c.base_price, 
--        t.price_per_minute, t.minimal_rating, t.minimal_experience
-- FROM cars c 
-- JOIN tariffs t ON c.tariff_id = t.id 
-- ORDER BY c.created_at DESC;

-- ============================================
-- Дополнительные примеры с разными состояниями
-- ============================================
/*
-- Машины в разных состояниях
INSERT INTO cars (id, model, license_plate, iot_serial_number, state, tariff_id, base_price, created_at, updated_at)
VALUES 
    (gen_random_uuid(), 'Lada Granta', 'Д123ЕЖ777', 'IOT-LADA-004', 'available', 
     (SELECT id FROM tariffs LIMIT 1), 150.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Kia Rio', 'Ж456ЗИ777', 'IOT-KIA-003', 'in_use',
     (SELECT id FROM tariffs LIMIT 1), 250.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'Toyota Camry', 'И789КЛ777', 'IOT-TOYOTA-003', 'reserved',
     (SELECT id FROM tariffs LIMIT 1), 400.0, NOW(), NOW()),
    
    (gen_random_uuid(), 'BMW 320i', 'К123ЛМ777', 'IOT-BMW-003', 'maintenance',
     (SELECT id FROM tariffs LIMIT 1), 600.0, NOW(), NOW());
*/

