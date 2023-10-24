-include .env

sqlite:
	rm -f $(SQLITE_FILE)
	touch $(SQLITE_FILE)
	chmod 775 $(SQLITE_FILE)

migrate:
	sqlx migrate run

unmigrate:
	sqlx migrate revert

docker-up:
	cargo build --release
	docker-compose build --no-cache
	docker-compose up -d

docker-down:
	docker-compose down
	docker-compose rm -f
	docker rmi dennis_lawter_resume_api_server:latest

docker-ps:
	docker ps -a -f "name=dennis_lawter_resume_api_server"

docker-bash:
	docker exec -it api-resume_dennis_lawter_resume_api_server_1 /bin/bash
