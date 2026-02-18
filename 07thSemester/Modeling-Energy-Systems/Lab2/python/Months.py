import subprocess
import matplotlib.pyplot as plt

time_values = []
for hour in range(24):
    time_str = f"{hour:02}:00"
    time_values.append(time_str)

all_results = []
day = 1
months = [12, 4, 7, 9]
solar_irradiance = [0.86, 3.96, 5.22, 3.12]

for i in range(len(months)):
    month = months[i]
    irradiance = solar_irradiance[i]
    results = []

    for time in time_values:
        command = ["Lab2.exe", "--day", f"{day:02}", "--month", f"{month:02}", "--time", time,
                   "--solar-irradiance", f"{irradiance}"]
        result = subprocess.run(command, capture_output=True, text=True)
        output_value = float(result.stdout.strip())
        results.append(output_value)

    all_results.append(results)


plt.figure(figsize=(10, 6))
colors = ['b', 'g', 'r', 'c']  # Different colors for each month
for idx, month_results in enumerate(all_results):
    plt.plot(time_values, month_results, marker='o', color=colors[idx], label=f'Month {months[idx]}')

# Adding titles and labels
plt.title(f'Summary Radiation for {day:02}st day of the month.')
plt.xlabel('Time (HH:MM)')
plt.ylabel('kWh / m^2')
plt.xticks(rotation=45)
plt.grid()
plt.legend(title='Months')
plt.tight_layout()

# Save the plot
plt.savefig('months.png')

# Show the plot (optional)
plt.show()