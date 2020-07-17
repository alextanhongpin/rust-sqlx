## rust-sqlx

```bash
# Start postgres.
$ make up

# Start application.
$ make start

# Stop postgres.
$ make down
```

Run this before running `make start`:

```sql
CREATE EXTENSION pgcrypto;
CREATE TABLE IF NOT EXISTS job (
	id uuid NOT NULL DEFAULT gen_random_uuid(),
	name text NOT NULL
);

INSERT INTO job(name) VALUES ('Software Engineer');
```

Output:

```
got 150
got job: Job { name: "Software Engineer", id: c8cc9e13-43c2-4fd5-a678-0d92dd1adb21  }
```
