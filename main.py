import yfinance as yf

# Example ticker symbol (Apple Inc.):
ticker_symbol = "AAPL"

# Use 'period="1d"' to get the past 1 trading day
# Use 'interval="5m"' to request 5-minute bars
data = yf.download(
    tickers=ticker_symbol,
    period="1d",
    interval="5m"
)

print(data)
