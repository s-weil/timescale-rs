#!/bin/bash
# chmod 774 run.sh
# source run.sh
k6 run --out influxdb=localhost:8086 k6.js
