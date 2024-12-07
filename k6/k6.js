import http from "k6/http";
import check from "k6";

export let options = {
    stages: [
        // Ramp-up from 1 to 30 VUs in 30s
        {duration: "5s", target: 10},

        // Stay on 30 VUs for 60s
        {duration: "20s", target: 10},

        // Ramp-down from 30 to 0 VUs in 10s
        {duration: "5", target: 0}
    ]
};

export default function () {
    const randomStockId = Math.floor(1 + Math.random() * 499);
    // let res = http.get(`localhost:8000/api/stocks?stockId=${randomStockId}`);
    // const url = `http://localhost:8000/api/stocks/${randomStockId}/time-series`
    const url = `http://localhost:8000/api/stocks/${randomStockId}/time-scale`
    console.log(url);
    let res = http.get(url);
    check(res, {"status is 200": (r) => r.status === 200});
}