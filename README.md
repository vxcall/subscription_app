thanks to docker network, postgres host name can be docker compose service

```Shell
DATABASE_URL=postgres://postgres@subscription_postgres:5432/subscription_db
# my_postgres is the service name in compose.yml
# rust_actix_db is created in create_db.sql
```

## migration


first time only

```Shell
sea-orm-cli migrate init
```

generate new table. the migration file will be created under /migration/src

```Shell
sea-orm-cli migrate generate create_user_table
```

migrate
```Shell
sea-orm-cli migrate up -u postgres://postgres@localhost:5432/subscription_db
```
