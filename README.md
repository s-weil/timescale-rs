# Test timescale-db

Test project to compare [timescaledb](todo-link) versus tables optimized with respect to a time-series structure.
The comparison is run over an `axum` web server to simulate real life problem.

### SetUp

Run

```bash 
cp ./.env.example ./.env
```

Adjust the `docker-compose.yml` and `.env` file, then run

```bash
docker-compose up -d

# or alternatively
# docker run --name postgres_db --env POSTGRES_USER=pgadmin POSTGRES_PASSWORD=pw --volume pg_data:/var/lib/postgresql/data -p 5432:5432 -d postgres

```

The migration scripts need to be run manually too (for now).

Finally populate the stock test data

```bash
cargo r --release --bin data-population
```

and run the API to serve requests

```bash
cargo r --release --bin stock-api
```

### TODO

k6 comparison
criterion comparison