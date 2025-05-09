struct ForecastInput {
    outflow: f64,
    inflow: f64,
    range: f64,
    inflation_rate: f64,
    inflow_offset: f64,
}
impl ForecastInput {
    fn from(raw: &[u8]) -> Option<Self> {
        // AI
        let json = std::str::from_utf8(raw).ok()?.trim();

        fn extract_value(json: &str, key: &str) -> Option<f64> {
            let pattern = format!("\"{}\":", key);
            let start = json.find(&pattern)? + pattern.len();
            let rest = &json[start..];
            let end = rest
                .find(|c: char| c == ',' || c == '}')
                .unwrap_or(rest.len());
            let value_str = &rest[..end].trim();
            value_str.parse::<f64>().ok()
        }

        Some(ForecastInput {
            outflow: extract_value(json, "outflow")?,
            inflow: extract_value(json, "inflow")?,
            range: extract_value(json, "range")?,
            inflation_rate: extract_value(json, "inflation_rate")?,
            inflow_offset: extract_value(json, "inflow_offset")?,
        })
    }
}

pub struct Forecast {
    input: ForecastInput,
    outflow_year: f64,
    inflow_year: f64,
    outflow_inflation: Vec<f64>,
    cumulative_outflow: Vec<f64>,
    cumulative_outflow_inflation: Vec<f64>,
    cumulative_inflow: Vec<f64>,
    net_flow: Vec<f64>,
}
impl Forecast {
    pub fn from(body: &[u8]) -> Self {
        let input = ForecastInput::from(body).unwrap();
        let range = input.range as usize;

        let mut forecast = Forecast {
            input,
            outflow_year: 0.0,
            inflow_year: 0.0,
            outflow_inflation: Vec::with_capacity(range),
            cumulative_outflow: Vec::with_capacity(range),
            cumulative_outflow_inflation: Vec::with_capacity(range),
            cumulative_inflow: Vec::with_capacity(range),
            net_flow: Vec::with_capacity(range),
        };

        forecast.init();
        forecast
    }

    fn init(&mut self) {
        self.outflow_year = self.input.outflow * 12.0;
        self.inflow_year = self.input.inflow * 12.0;

        self.set_outflow_inflation();
        self.set_cumulative_outflow();
        self.set_cumulative_outflow_inflation();
        self.set_cumulative_inflow();
        self.set_net_flow();
    }

    pub fn response(&self) -> String {
        format!(
            "{{\"outflow_year\":{},\"inflow_year\":{},\"outflow_inflation\":{:?},\"cumulative_outflow\": {:?},\"cumulative_outflow_inflation\": {:?},\"cumulative_inflow\": {:?},\"net_flow\": {:?}}}",
            self.outflow_year,
            self.inflow_year,
            self.outflow_inflation,
            self.cumulative_outflow,
            self.cumulative_outflow_inflation,
            self.cumulative_inflow,
            self.net_flow,
        )
    }

    fn set_outflow_inflation(&mut self) {
        let out_inf = &mut self.outflow_inflation;

        for i in 1..=self.input.range as usize {
            let rate = (1.0 + self.input.inflation_rate).powf(i as f64) - 1.0;
            let curr = self.outflow_year / (1.0 - rate);

            out_inf.push(curr);
        }
    }

    fn set_cumulative_outflow(&mut self) {
        let cm_out = &mut self.cumulative_outflow;

        let mut prev = 0.0;

        for _ in 0..self.input.range as usize {
            let curr = prev + self.outflow_year;
            cm_out.push(curr);

            prev = curr;
        }
    }

    fn set_cumulative_outflow_inflation(&mut self) {
        let cm_out_inf = &mut self.cumulative_outflow_inflation;

        let mut prev = 0.0;

        for year in self.outflow_inflation.iter() {
            let curr = prev + year;
            cm_out_inf.push(curr);

            prev = curr;
        }
    }

    fn set_cumulative_inflow(&mut self) {
        let cm_inf = &mut self.cumulative_inflow;

        let start = self.input.inflow_offset as usize;
        let end = self.input.range as usize - start;

        for _ in 0..start {
            cm_inf.push(0.0);
        }

        let mut prev = 0.0;
        for _ in 0..end {
            let curr = prev + self.inflow_year;
            cm_inf.push(curr);

            prev = curr;
        }
    }

    fn set_net_flow(&mut self) {
        let cm_net = &mut self.net_flow;

        let mut i = 0;
        while i < self.input.range as usize {
            let net = self.cumulative_inflow[i] - self.cumulative_outflow_inflation[i];
            cm_net.push(net);
            i += 1;
        }
    }
}
