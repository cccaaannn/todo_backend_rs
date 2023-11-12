# sqlx migration usage

**sqlx cli uses `DATABASE_URL` from environment**

[sqlx cli docs](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli)

Install sqlx cli
```shell
cargo install sqlx-cli
```

Create db
```shell
sqlx database create
```

Add migration
```shell
sqlx migrate add <NAME>
```

Run migration
```shell
sqlx migrate run
```
