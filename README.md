
# start up

1. create .env first. refer to .env.example file as an example
2. then run following commands

```Shell
docker network create subscription_network
docker compose up --build -d # this will create a new database
docker compose logs -f
```

# clean up when finished

```Shell
docker compose down
docker network rm subscription_network
```

# migration related commands

- command for generating new table. the migration file will be created under /migration/src

```Shell
sea-orm-cli migrate generate create_user_table
```

- migrate

```Shell
sea-orm-cli migrate up -u postgres://postgres@localhost:5432/subscription_db
```
