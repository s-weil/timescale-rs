# Test timescale-db

Test project to compare the postgres extension [timescaledb](https://www.timescale.com/) versus (postgres) tables
optimized with respect to a time-series structure.
The comparison is run over an `axum` web server to simulate a real life problem.

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
cargo r --release --bin data_population
```

and run the API to serve requests

```bash
cargo r --release --bin stock-api
```

To test if the API is properly up and running,

```bash
curl -G localhost:8000/api/stocks/41/time-series
curl -G localhost:8000/api/stocks/41/time-scale
```

## Testing for performance

### k6 comparison

Download the `k6` [binaries](https://github.com/grafana/xk6-disruptor/releases) or install via

```bash
sudo dnf install https://dl.k6.io/rpm/repo.rpm
sudo dnf install k6
```

For the setup of Grafana dashboards, follow
this [post](https://medium.com/@nairgirish100/k6-with-docker-compose-influxdb-grafana-344ded339540). In particular
import the dashboard with id `2587` in [Grafana](http://localhost:3000/) and
add the source (Connections > Data sources) “influxdb” (http://influxdb:8086).
Once set up, run

```bash
# chmod 774 ./k6/run.sh
source ./k6/run.sh
```

### criterion comparison

We use [criterion.rs](https://bheisler.github.io/criterion.rs/) for benchmarking the `data_population` project.
Run

```bash
cargo bench -p data_population
```

This can take a while... Adjust the `ts_length` or criterion's sample size to your convenience.
Note that the timings include some noise for the setup.

### TODOs

- finalize criterion; stocks must be populated too
- cluster table vs timescale aggregations