# Kubernetes Deployment для ZDrive

Этот каталог содержит Kubernetes манифесты для развертывания всех сервисов ZDrive.

## Структура

```
k8s/
├── base/              # Базовые манифесты
│   ├── namespace.yaml
│   ├── secrets.yaml
│   ├── postgres-*.yaml
│   ├── redis.yaml
│   ├── rabbitmq.yaml
│   ├── *-migrate.yaml
│   ├── *-service.yaml
│   ├── ingress.yaml
│   └── kustomization.yaml
└── overlays/          # Оверлеи для разных окружений (опционально)
```

## Предварительные требования

1. Kubernetes кластер (версия 1.24+)
2. kubectl настроен для работы с кластером
3. kustomize (опционально, для использования kustomization)
4. Docker образы для всех сервисов

## Подготовка образов

Перед развертыванием необходимо собрать и загрузить Docker образы в registry:

```bash
# Сборка образов
docker build -t zdrive-users:latest -f users/Dockerfile .
docker build -t zdrive-cars:latest -f cars/Dockerfile .
docker build -t zdrive-trips:latest -f trips/Dockerfile .
docker build -t zdrive-telematics:latest -f telematics/Dockerfile .
docker build -t zdrive-billing:latest -f billing/Dockerfile .
docker build -t zdrive-dispatcher:latest -f dispatcher/Dockerfile .
docker build -t zdrive-client-frontend:latest -f frontend/client/Dockerfile .
docker build -t zdrive-admin-frontend:latest -f frontend/admin/Dockerfile .

# Если используете registry (например, Docker Hub или private registry):
docker tag zdrive-users:latest your-registry/zdrive-users:latest
docker push your-registry/zdrive-users:latest
# ... и так далее для всех образов
```

После этого обновите имена образов в манифестах (замените `zdrive-*:latest` на `your-registry/zdrive-*:latest`).

## Развертывание

### Вариант 1: Использование kubectl напрямую

```bash
# Применить все манифесты
kubectl apply -f k8s/base/

# Проверить статус
kubectl get all -n zdrive

# Посмотреть логи
kubectl logs -n zdrive deployment/users-service
```

### Вариант 2: Использование kustomize

```bash
# Применить с kustomize
kubectl apply -k k8s/base/

# Или с kustomize напрямую
kubectl kustomize k8s/base/ | kubectl apply -f -
```

## Порядок развертывания

Рекомендуемый порядок развертывания:

1. **Инфраструктура:**
   ```bash
   kubectl apply -f k8s/base/namespace.yaml
   kubectl apply -f k8s/base/secrets.yaml
   kubectl apply -f k8s/base/postgres-users.yaml
   kubectl apply -f k8s/base/postgres-cars.yaml
   kubectl apply -f k8s/base/postgres-trips.yaml
   kubectl apply -f k8s/base/postgres-billing.yaml
   kubectl apply -f k8s/base/redis.yaml
   kubectl apply -f k8s/base/rabbitmq.yaml
   ```

2. **Дождаться готовности баз данных:**
   ```bash
   kubectl wait --for=condition=ready pod -l app=postgres-users -n zdrive --timeout=300s
   kubectl wait --for=condition=ready pod -l app=postgres-cars -n zdrive --timeout=300s
   kubectl wait --for=condition=ready pod -l app=postgres-trips -n zdrive --timeout=300s
   kubectl wait --for=condition=ready pod -l app=postgres-billing -n zdrive --timeout=300s
   ```

3. **Запустить миграции:**
   ```bash
   kubectl apply -f k8s/base/users-migrate.yaml
   kubectl apply -f k8s/base/cars-migrate.yaml
   kubectl apply -f k8s/base/trips-migrate.yaml
   kubectl apply -f k8s/base/billing-migrate.yaml
   
   # Дождаться завершения миграций
   kubectl wait --for=condition=complete job/users-migrate -n zdrive --timeout=300s
   kubectl wait --for=condition=complete job/cars-migrate -n zdrive --timeout=300s
   kubectl wait --for=condition=complete job/trips-migrate -n zdrive --timeout=300s
   kubectl wait --for=condition=complete job/billing-migrate -n zdrive --timeout=300s
   ```

4. **Развернуть сервисы:**
   ```bash
   kubectl apply -f k8s/base/users-service.yaml
   kubectl apply -f k8s/base/cars-service.yaml
   kubectl apply -f k8s/base/trips-service.yaml
   kubectl apply -f k8s/base/telematics-service.yaml
   kubectl apply -f k8s/base/billing-service.yaml
   ```

5. **Развернуть dispatcher:**
   ```bash
   kubectl apply -f k8s/base/dispatcher-service.yaml
   ```

6. **Развернуть фронтенды:**
   ```bash
   kubectl apply -f k8s/base/client-frontend.yaml
   kubectl apply -f k8s/base/admin-frontend.yaml
   ```

7. **Настроить Ingress:**
   ```bash
   # Убедитесь, что Ingress Controller установлен (например, nginx-ingress)
   kubectl apply -f k8s/base/ingress.yaml
   ```

## Проверка развертывания

```bash
# Проверить все ресурсы
kubectl get all -n zdrive

# Проверить поды
kubectl get pods -n zdrive

# Проверить сервисы
kubectl get svc -n zdrive

# Проверить логи
kubectl logs -n zdrive deployment/users-service -f
kubectl logs -n zdrive deployment/dispatcher -f

# Проверить события
kubectl get events -n zdrive --sort-by='.lastTimestamp'
```

## Доступ к приложению

После развертывания приложение будет доступно через Ingress:

- **Client Frontend:** http://localhost/client
- **Admin Frontend:** http://localhost/admin
- **API (Dispatcher):** http://localhost/api

Если Ingress не настроен, можно использовать port-forward:

```bash
# Dispatcher
kubectl port-forward -n zdrive svc/dispatcher 8080:8080

# Client Frontend
kubectl port-forward -n zdrive svc/client-frontend 3000:80

# Admin Frontend
kubectl port-forward -n zdrive svc/admin-frontend 3001:80
```

## Масштабирование

Для масштабирования сервисов:

```bash
kubectl scale deployment users-service -n zdrive --replicas=3
kubectl scale deployment dispatcher -n zdrive --replicas=3
```

## Обновление

Для обновления образа сервиса:

```bash
# Обновить образ в deployment
kubectl set image deployment/users-service users-service=zdrive-users:v2.0 -n zdrive

# Или отредактировать deployment
kubectl edit deployment users-service -n zdrive
```

## Удаление

Для удаления всех ресурсов:

```bash
kubectl delete namespace zdrive
```

Или удалить по отдельности:

```bash
kubectl delete -f k8s/base/
```

## Примечания

1. **Secrets:** В production окружении используйте более безопасные методы хранения секретов (например, External Secrets Operator, Sealed Secrets, или cloud-native решения).

2. **Health Checks:** Убедитесь, что ваши сервисы имеют эндпоинты `/health` для liveness и readiness проб.

3. **Resource Limits:** Добавьте resource limits и requests в production:

```yaml
resources:
  requests:
    memory: "128Mi"
    cpu: "100m"
  limits:
    memory: "256Mi"
    cpu: "200m"
```

4. **Persistent Volumes:** Убедитесь, что в кластере настроен StorageClass для PersistentVolumeClaims.

5. **Ingress Controller:** Для работы Ingress необходим установленный Ingress Controller (например, nginx-ingress).

6. **Образы:** Замените `zdrive-*:latest` на реальные имена образов из вашего registry.

