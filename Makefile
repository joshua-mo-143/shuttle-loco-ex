docker:
	docker run -d -t -p 8500:5432 -e POSTGRES_PASSWORD=postgres postgres
