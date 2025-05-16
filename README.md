# Budget Forecast Calculator

A web application for calculating long-term financial forecasts with inflation adjustments

Server: Amazon EC2, t4g.nano, us-east-1c, Amazon Linux 2023 AMI 64-bit(Arm), 8 GiB gp3.

link: https://evertonprdo.dev

AI Assistance: https://www.linkedin.com/feed/update/urn:li:activity:7328787266158407681/

## Features

- Real-time calculation of financial projections
- Adjustable parameters:
  - Monthly outflow (expenses)
  - Monthly inflow (income)
  - Time range (years)
  - Inflation rate
  - Inflow offset (delayed income start)
- Cumulative calculations considering:
  - Inflation-adjusted outflows
  - Base outflows
  - Total inflows
  - Net flow analysis

## Technical Stack

- Frontend: Vanilla JavaScript, CSS3
- Backend: Rust Rocket HTTP server
  - serde: JSON parsing and generation
  - Static file serving

### Notes

- Front-end: Almost everything AI
- Back-end: Rocket
- README: Almost everything AI

**Motivation**: I was curious to see how Rust would perform in a low-memory environment. Sure, the server doesn’t do much, but I can still gather some metrics on baseline memory usage and draw a few insights.
