import pandas as pd
import geopandas as gpd
import matplotlib.pyplot as plt

import warnings
warnings.filterwarnings('ignore')


world = gpd.read_file(
    gpd.datasets.get_path(
        'naturalearth_lowres'))

world = world[
    (world.pop_est > 0) &
    (world.name != "Antarctica")]

world['gdp_per_cap'] = (
        world.gdp_md_est / 
        world.pop_est)

world.plot(
    column='gdp_per_cap')

plt.show()


world = gpd.read_file(
    gpd.datasets.get_path(
        'naturalearth_lowres'))

world = world[
    (world.pop_est > 0) &
    (world.name != "Antarctica")]

data = gpd.read_file(
    gpd.datasets.get_path(
        'naturalearth_lowres'))

data = data[
    (data.pop_est > 0) &
    (data.name != 'Antarctica')]
data['centroid_column'] = data.centroid
data['gdp_per_cap'] = (data.gdp_md_est 
                       / data.pop_est)

centroids = list(data['centroid_column'])
df = pd.DataFrame({
    'y':[centroids[i].y for i in
         range(len(centroids))],
    'x':[centroids[i].x for i in
         range(len(centroids))],
    'data':list(data['gdp_per_cap'])})

base = world.plot(
    color='white',
    edgecolor='black')
df.plot(
    kind='scatter',
    x='x', y='y',
    s=df['data']*1000,
    ax=base)
plt.show()



import libpysal
import numpy as np
from esda.moran import Moran

f = libpysal.io.open(libpysal.examples.get_path("stl_hom.txt"))
y = np.array(f.by_col['HR8893'])
w = libpysal.io.open(
    libpysal.examples.get_path('stl.gal')).read()

mi = Moran(y, w, two_tailed=False)
print(mi.I)