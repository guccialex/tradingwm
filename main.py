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

# [*********************100%***********************]  1 of 1 completed
# Price                           Close        High         Low        Open   Volume
# Ticker                           AAPL        AAPL        AAPL        AAPL     AAPL
# Datetime                                                                          
# 2025-01-23 14:30:00+00:00  225.279907  226.259995  224.500000  224.729996  2920044
# 2025-01-23 14:35:00+00:00  225.539993  225.779999  224.682007  225.270004   856159

for index, row in data.iterrows():
    print(f"Time: {index}, Close: {row['Close']}, High: {row['High']}, Low: {row['Low']}, Open: {row['Open']}, Volume: {row['Volume']}")