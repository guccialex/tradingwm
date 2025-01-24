
import yfinance as yf
import pandas as pd



class StockDataDownloader:
    """
    Downloads and stores 5-minute interval stock data for the past year using yfinance.
    Provides a method to retrieve the start-of-day bar for any given date.
    """

    def __init__(self, ticker="AAPL"):
        """
        Initializes the downloader with a default ticker symbol of "AAPL".
        Downloads the past year's 5-minute interval data and stores it in a DataFrame.
        """
        self.ticker = ticker
        # Download last 1 year (365 days) of 5-minute data
        self.data = yf.download(
            tickers=self.ticker,
            period="60d",
            interval="5m"
        )
       
        
        # # Convert the index to a standard DatetimeIndex (just in case)
        self.data.index = pd.to_datetime(self.data.index)

    def get_average_volume_over_last_n_days(self, date_start_str, n=14):

        date_data = self.data.loc[date_start_str]
        row = date_data.iloc[0]

        datetime = row.name
        close = row['Close']['AAPL']
        high = row['High']['AAPL']
        low = row['Low']['AAPL']
        open = row['Open']['AAPL']
        volume = row['Volume']['AAPL']

        print(f"Open: {open}, High: {high}, Low: {low}, Close: {close}, Volume: {volume}, Date: {datetime}")
        

    def get_day_start_interval(self, date_str):
        """
        Given a date string in 'YYYY-MM-DD' format, returns the first bar of that day's trading session.
        By default, for U.S. equities, we assume the open occurs at 9:30 AM Eastern,
        which is typically 14:30 UTC in winter (13:30 UTC in summer).
        
        Returns a dictionary with open, high, low, close, and volume if found. Otherwise, None.
        """
        # Filter the data to just that date
        # (this works if your index is in UTC and each row's date is the trade date)
        # 'date_str' must be in 'YYYY-MM-DD'
        date_data = self.data.loc[date_str]
        row = date_data.iloc[0]

        
        datetime = row.name
        close = row['Close']['AAPL']
        high = row['High']['AAPL']
        low = row['Low']['AAPL']
        open = row['Open']['AAPL']
        volume = row['Volume']['AAPL']

        print(f"Open: {open}, High: {high}, Low: {low}, Close: {close}, Volume: {volume}, Date: {datetime}")
        


if __name__ == "__main__":
    # Example usage
    downloader = StockDataDownloader(ticker="AAPL")
    
    # Get the first bar of a specific date (e.g., January 23, 2025)
    day_start = downloader.get_day_start_interval("2025-01-22")
    if day_start:
        print("Day Start Interval:")
        print(day_start)
    else:
        print("No data found for the requested date/time.")