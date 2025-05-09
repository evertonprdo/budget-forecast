# Budget Forecast Calculator

A web application for calculating long-term financial forecasts with inflation adjustments.

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
- Backend: Rust HTTP Server (no external crates)
  - Custom thread pool implementation
  - JSON parsing and generation
  - Static file serving

## Architecture

- Modular backend with separation of concerns
- Responsive single-page frontend
- RESTful API for forecast calculations
- Thread pool for handling concurrent requests

### Notes

Front-end: Almost everything AI
Back-end: A little bit of AI
README: Almost everything AI

motivation: Just some experiments