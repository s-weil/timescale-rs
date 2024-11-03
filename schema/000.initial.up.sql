CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
-- select * from pg_extension;

CREATE SCHEMA stocks;

SET search_path TO stocks;

CREATE TABLE STOCK_DEFINITIONS (
    id SERIAL NOT NULL,
    ticker TEXT not null,
    PRIMARY KEY (id)
);

-- Create a table without timescaledb...
CREATE TABLE STOCK_TIMESERIES (
    stock_id INTEGER NOT NULL,
    dt TIMESTAMP NOT NULL,
    close NUMERIC NOT NULL,
    PRIMARY KEY (stock_id, dt),
    CONSTRAINT stock_id_fk FOREIGN KEY (stock_id) REFERENCES STOCK_DEFINITIONS(id)
);

-- ... and a timescaledb table for comparison
CREATE TABLE IF NOT EXISTS STOCK_TIMESCALE (
    stock_id INTEGER NOT NULL,
    dt TIMESTAMPTZ NOT NULL,
    close NUMERIC NOT NULL,
    PRIMARY KEY (stock_id, dt),
    CONSTRAINT stock_id_fk FOREIGN KEY (stock_id) REFERENCES STOCK_DEFINITIONS(id)
);

-- SELECT create_hypertable('stocks.STOCK_TIMESCALE', by_range('dt'));
SELECT create_hypertable('stocks.STOCK_TIMESCALE', 'dt');