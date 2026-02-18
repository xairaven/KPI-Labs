import matplotlib.pyplot as plt
import numpy as np

data = np.genfromtxt('./data/daily-minimum-temperatures-in-me.csv', 
                     delimiter = ",", skip_header = 1)
temps = data[:,1]
mean = np.nanmean(temps)
std = np.nanstd(temps)

plt.plot([0,len(temps)], [mean, mean], 'g-')
plt.plot([0,len(temps)], [mean+std, mean+std], 'y-')
plt.plot([0,len(temps)], [mean, mean], 'g-')
plt.plot([0,len(temps)], [mean+std,mean+std], 'y-')
plt.plot([0,len(temps)], [mean-std, mean-std], 'y-')
plt.plot([0,len(temps)], [mean+2*std, mean+2*std], 'r-')
plt.plot([0,len(temps)], [mean-2*std, mean-2*std], 'r-')
plt.plot([0,len(temps)], [mean, mean], 'g-')
plt.plot([0,len(temps)], [mean+std, mean+std], 'y-')
plt.plot([0,len(temps)], [mean-std, mean-std], 'y-')
plt.plot([0,len(temps)], [mean+2*std, mean+2*std], 'r-')
plt.plot([0,len(temps)], [mean-2*std, mean-2*std], 'r-')
plt.plot(range(len(temps)), temps, 'b-')
plt.show()