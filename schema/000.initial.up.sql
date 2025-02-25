CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
-- select * from pg_extension;

CREATE SCHEMA stocks;

SET search_path TO stocks;

CREATE TABLE STOCK_DEFINITIONS
(
    id     SERIAL NOT NULL,
    ticker TEXT   not null,
    PRIMARY KEY (id),
    UNIQUE (ticker)
);

-- Create a table without timescaledb...
CREATE TABLE STOCK_TIMESERIES
(
    stock_id INTEGER NOT NULL,
    dt       DATE    NOT NULL,
    close    NUMERIC NOT NULL,
    PRIMARY KEY (stock_id, dt),
    CONSTRAINT stock_id_fk FOREIGN KEY (stock_id) REFERENCES STOCK_DEFINITIONS (id)
    -- TODO consider dedicated index on stock_id (though partially contained in PK); consider dt sorted
);

-- ... and a timescaledb table for comparison
CREATE TABLE IF NOT EXISTS STOCK_TIMESCALE
(
    stock_id INTEGER NOT NULL,
    dt       DATE    NOT NULL,
    close    NUMERIC NOT NULL,
    PRIMARY KEY (stock_id, dt),
    CONSTRAINT stock_id_fk FOREIGN KEY (stock_id) REFERENCES STOCK_DEFINITIONS (id)
);

-- SELECT create_hypertable('stocks.STOCK_TIMESCALE', by_range('dt'));
SELECT create_hypertable('stocks.STOCK_TIMESCALE', 'dt');