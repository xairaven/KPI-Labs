import matplotlib.pyplot as plt
import numpy as np
from scipy.cluster.hierarchy import linkage, dendrogram
import sqlite3

db_filename = 'data/cereals-1.db'
conn = sqlite3.connect(db_filename)
c = conn.cursor()


c.execute('''SELECT Protein, 
                    Fat, 
                    Carbohydrates 
             FROM cereals''')
results = c.fetchall()

X = (np.array(results)
     .reshape(-1,3))

links = linkage(X, 'centroid')

dendrogram(links)
plt.show()


from scipy.cluster.vq import whiten

Y = whiten(X)

links = linkage(Y, 'centroid')

dendrogram(links)
plt.show()