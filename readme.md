# S&P 500 Data Analysis and Momentum Evaluation


## Roadmap

- [ ] **Get function that gives the S&P 500 list at any given time**  

- [ ] **Check S&P list generation given a certain time is correct**  

- [ ] **Fetch and save data for each stock using Yahoo Finance, Retrieve the stock price and market capitalization for each S&P 500 stock at the start of each quarter over the last 20 years and save the data in JSON format**

- [ ] **Figure out and implement the momentum algorithm to identify the top 20% of S&P 500 stocks based on momentum for each year.**

- [ ] **Implement simple evaluation metric using the fetched stock price data, to evaluate the returns on the portfolio suggested by the S&P 500 + momentum strategy.**

- [ ] **Hypertune the hardcoded values. Use the evaluation metric and momentum selection function to optimize the parameters for maximum return.**

- [ ] **Figure out how to use backtest as our evaluation metric, so we can hypertune to reality and validate the strategy better.**

## Installation

1. Clone the repository.
2. Ensure you have Rust installed.
3. Install any required dependencies.

## Usage

Run the main executable to generate the S&P 500 list, fetch stock data, and evaluate the momentum strategy:

```bash
cargo run --release
