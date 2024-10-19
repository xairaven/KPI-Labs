import subprocess
import matplotlib.pyplot as plt

day = 1
month = 7
time = "12:00"
solar_irradiance = 5.22

azimuths = []
for value in range(-179, 181, 10):
    string = f"{value}"
    azimuths.append(string)
results = []
for value in azimuths:
    command = ["Lab2.exe", "--day", f"{day:02}", "--month", f"{month:02}", "--time", time,
               "--solar-irradiance", f"{solar_irradiance}", "--azimuth", f"'{value}'"]
    result = subprocess.run(command, capture_output=True, text=True)
    output_value = float(result.stdout.strip())
    results.append(output_value)
plt.figure(figsize=(10, 6))
plt.plot(azimuths, results, marker='o')
plt.title(f'Radiation by changing azimuth. Date & Time: {day:02}.{month:02} {time}.')
plt.xlabel('Azimuth')
plt.ylabel('kWh / m^2')
plt.xticks(rotation=45)
plt.grid()
plt.tight_layout()
plt.savefig('azimuth.png')
plt.show()


tilts = []
for value in range(0, 91, 5):
    string = f"{value}"
    tilts.append(string)
results = []
for value in tilts:
    command = ["Lab2.exe", "--day", f"{day:02}", "--month", f"{month:02}", "--time", time,
               "--solar-irradiance", f"{solar_irradiance}", "--tilt-angle", f"'{value}'"]
    result = subprocess.run(command, capture_output=True, text=True)
    output_value = float(result.stdout.strip())
    results.append(output_value)
plt.figure(figsize=(10, 6))
plt.plot(tilts, results, marker='o')
plt.title(f'Radiation by changing tilt. Date & Time: {day:02}.{month:02} {time}.')
plt.xlabel('Tilt')
plt.ylabel('kWh / m^2')
plt.xticks(rotation=45)
plt.grid()
plt.tight_layout()
plt.savefig('tilt.png')
plt.show()