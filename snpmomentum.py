import pandas as pd
import numpy as np
from backtesting import Backtest, Strategy
from backtesting.lib import crossover
from backtesting.test import SMA, GOOG

class SP500MomentumStrategy(Strategy):
    lookback_period = 252  # 12 months (~252 trading days)
    exclude_recent = 21  # Exclude last month (~21 trading days)
    volatility_period = 252  # Standard deviation period
    rebalance_period = 126  # Semi-annual rebalance (~6 months)
    top_quantile = 0.2  # Top 20% of momentum scores
    
    def init(self):
        self.data['momentum'] = self.data.Close.shift(self.exclude_recent) / self.data.Close.shift(self.lookback_period) - 1
        self.data['volatility'] = self.data.Close.pct_change().rolling(self.volatility_period).std()
        self.data['risk_adj_momentum'] = self.data['momentum'] / self.data['volatility']
        self.data.dropna(inplace=True)
        
    def next(self):
        if len(self.data) % self.rebalance_period == 0:
            ranked = self.data['risk_adj_momentum'].iloc[-1]
            selected = ranked >= np.percentile(ranked, 100 * (1 - self.top_quantile))
            
            self.position.close()
            if selected:
                self.buy()

# Load historical data for testing
# Replace GOOG with real S&P 500 data
data = GOOG.copy()
data['Date'] = pd.to_datetime(data.index)
data.set_index('Date', inplace=True)

# Run backtest
bt = Backtest(data, SP500MomentumStrategy, cash=100000, commission=0.001)
results = bt.run()
bt.plot()
