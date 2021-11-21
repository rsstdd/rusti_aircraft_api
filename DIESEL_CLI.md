# CLI Diesel

```bash
cargo install diesel_cli
diesel setup
diesel migration generate aircraft
```

Fill out `up.sql` and `down.sql`

```bash
diesel migration run
diesel migration redo # ensure that dow.sql rolls back correctly
```

```sql
CREATE DATABASE aircraft-dev
```
