import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


data = np.genfromtxt('./data/daily-minimum-temperatures-in-me.csv', 
                     delimiter = ",", skip_header = 1)
temps = data[:,1]

mean = np.nanmean(temps)
std = np.nanstd(temps)

window_size = 50

plt.plot(np.convolve(temps,
                     np.ones(window_size,)/
                     window_size, 'same'))
plt.show()



data = np.genfromtxt('./data/daily-minimum-temperatures-in-me.csv',
                     delimiter = ",", skip_header = 1)
temps = data[:,1]

span = 50
df = pd.DataFrame({'temps':temps})
df_exp = (df.ewm(span = 50)
          .mean())
plt.plot(df_exp)
plt.show()