# Testing Instructions

```shell
cargo make up # NOTE: This runs docker-compose up -d
cargo make logs # This runs multi tail on the logs files
cargo make down # NOTE: This runs docker-compose down

docker-compose logs -f lib # This will run for a single service
```
