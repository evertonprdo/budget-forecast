use rocket::serde::{Serialize, json::Json};

#[derive(FromForm)]
pub struct ForecastRequest {
    outflow: f64,
    inflow: f64,
    range: f64,
    inflation_rate: f64,
    inflow_offset: f64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ForecastResponse {
    outflow_year: f64,
    inflow_year: f64,
    outflow_inflation: Vec<f64>,
    cumulative_outflow: Vec<f64>,
    cumulative_outflow_inflation: Vec<f64>,
    cumulative_inflow: Vec<f64>,
    net_flow: Vec<f64>,
}

pub struct Forecast {
    request: ForecastRequest,
    response: ForecastResponse,
}
impl Forecast {
    pub fn from(params: ForecastRequest) -> Self {
        let range = params.range as usize;
        let response = ForecastResponse {
            outflow_year: 0.0,
            inflow_year: 0.0,
            outflow_inflation: Vec::with_capacity(range),
            cumulative_outflow: Vec::with_capacity(range),
            cumulative_outflow_inflation: Vec::with_capacity(range),
            cumulative_inflow: Vec::with_capacity(range),
            net_flow: Vec::with_capacity(range),
        };

        let mut forecast = Forecast {
            request: params,
            response,
        };

        forecast.init();
        forecast
    }

    pub fn response(self) -> Json<ForecastResponse> {
        Json(self.response)
    }

    fn init(&mut self) {
        self.response.outflow_year = self.request.outflow * 12.0;
        self.response.inflow_year = self.request.inflow * 12.0;

        self.set_outflow_inflation();
        self.set_cumulative_outflow();
        self.set_cumulative_outflow_inflation();
        self.set_cumulative_inflow();
        self.set_net_flow();
    }

    fn set_outflow_inflation(&mut self) {
        let out_inf = &mut self.response.outflow_inflation;
        let mut curr = self.response.outflow_year;

        for _ in 0..self.request.range as usize {
            curr *= 1.0 + self.request.inflation_rate;

            out_inf.push(curr);
        }
    }

    fn set_cumulative_outflow(&mut self) {
        let cm_out = &mut self.response.cumulative_outflow;

        let mut prev = 0.0;

        for _ in 0..self.request.range as usize {
            let curr = prev + self.response.outflow_year;
            cm_out.push(curr);

            prev = curr;
        }
    }

    fn set_cumulative_outflow_inflation(&mut self) {
        let cm_out_inf = &mut self.response.cumulative_outflow_inflation;

        let mut prev = 0.0;

        for year in self.response.outflow_inflation.iter() {
            let curr = prev + year;
            cm_out_inf.push(curr);

            prev = curr;
        }
    }

    fn set_cumulative_inflow(&mut self) {
        let cm_inf = &mut self.response.cumulative_inflow;

        let start = self.request.inflow_offset as usize;
        let end = self.request.range as usize - start;

        for _ in 0..start {
            cm_inf.push(0.0);
        }

        let mut prev = 0.0;
        for _ in 0..end {
            let curr = prev + self.response.inflow_year;
            cm_inf.push(curr);

            prev = curr;
        }
    }

    fn set_net_flow(&mut self) {
        let cm_net = &mut self.response.net_flow;

        let mut i = 0;
        while i < self.request.range as usize {
            let net =
                self.response.cumulative_inflow[i] - self.response.cumulative_outflow_inflation[i];

            cm_net.push(net);
            i += 1;
        }
    }
}
