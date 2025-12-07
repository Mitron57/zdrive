.PHONY: help build up down logs restart clean test

help: ## Показать справку
	@echo "Доступные команды:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Собрать Docker образ
	docker-compose build

up: ## Запустить сервисы
	docker-compose up -d

down: ## Остановить сервисы
	docker-compose down

logs: ## Показать логи
	docker-compose logs -f users-service

restart: ## Перезапустить сервисы
	docker-compose restart

clean: ## Удалить контейнеры и volumes
	docker-compose down -v
	docker rmi users-users-service 2>/dev/null || true

test: ## Запустить тесты
	cargo test --package users

.PHONY: db-shell
db-shell: ## Подключиться к PostgreSQL
	docker-compose exec postgres psql -U zdrive -d zdrive_users

.PHONY: shell
shell: ## Войти в контейнер сервиса
	docker-compose exec users-service /bin/bash

