import matplotlib.pyplot as plt
import pandas
from statsmodels.graphics.mosaicplot import mosaic

df = pandas.read_csv('data/cereals_conf.csv', sep=",")

mosaic(df, ['Fat', 'Protein'])
plt.show()


import pandas
import matplotlib.pyplot as plt
import pandas.plotting as plotting

df = pandas.read_csv('data/cereals_num.csv', sep=",")

plotting.parallel_coordinates(df, 'Cereal')
plt.gca().legend_.remove()
plt.show()



import numpy as np
import matplotlib.pyplot as plt
import matplotlib.cm as cm

inData = np.loadtxt('data/cereals_normed.csv',
                    delimiter=',',
                    dtype=float,
                    skiprows=1)
tData = inData.transpose()

plt.imshow(tData,
           interpolation='nearest',
           cmap=cm.inferno)
plt.axis('off')
plt.show()



import pandas
import matplotlib.pyplot as plt

df = pandas.read_csv('data/cereals_num.csv', sep=",")

plt.scatter(df['Fat'], df['Protein'])
plt.show()


from wordcloud import WordCloud
import matplotlib.pyplot as plt

text = open('data/kjb.txt').read()

wordcloud = WordCloud(background_color='white', width=800, height=400)
wordcloud.generate(text)

plt.imshow(wordcloud, interpolation='bilinear')
plt.axis('off')
plt.show()