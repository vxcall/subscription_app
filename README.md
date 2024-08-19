thanks to docker network, postgres host name can be docker compose service

```Shell
DATABASE_URL=postgres://postgres@my_postgres:5432/rust_actix_db
# my_postgres is the service name in compose.yml
# rust_actix_db is created in create_db.sql
```

## migration

```
sea-orm-cli migrate init
sea-orm-cli migrate generate create_user_table
sea-orm-cli migrate up -u postgres://postgres@localhost:5432/rust_actix_db
```