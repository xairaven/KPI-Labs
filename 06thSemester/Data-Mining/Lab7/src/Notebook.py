import folium
from folium.plugins import MarkerCluster
import pandas as pd

df_incidents = pd.read_csv('./data/PoliceDepartmentIncidents2016.csv')
limit = 1000
df_incidents = df_incidents.iloc[0:limit, :]
df_incidents.shape

# San Francisco latitude and longitude values

latitude = 37.77
longitude = -122.42


sanfran_map = folium.Map(location = [latitude, longitude], zoom_start = 12)
incidents = folium.map.FeatureGroup()

for lat, lng, in zip(df_incidents.Y, df_incidents.X):
    incidents.add_child(folium.features.CircleMarker(
        [lat, lng], radius = 5, color = "yellow", fill_color = "blue", fill_opacity=0.6)
    )

sanfran_map.add_child(incidents)
sanfran_map = folium.Map(location =[latitude, longitude], zoom_start = 12)
incidents = MarkerCluster().add_to(sanfran_map)

for lat, lng, label, in zip(df_incidents.Y, df_incidents.X, df_incidents.Category):
    folium.Marker(location = [lat, lng], icon = None, popup = label).add_to(incidents)

sanfran_map