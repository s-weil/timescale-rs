# Test timescale-db

Test project to compare [timescaledb](todo-link) versus tables optimized with respect to a time-series structure.

### SetUp

Run

```bash 
cp ./.env.example ./.env
```

Adjust the `docker-compose.yml` and `.env` file, then run

```bash
docker-compose up -d
```

The migration scripts need to be run manually too (for now).

Finally run

```bash
cargo r --release --bin data-population
```

and

```bash
cargo r --release --bin stock-api
```

### TODO

k6 comparison