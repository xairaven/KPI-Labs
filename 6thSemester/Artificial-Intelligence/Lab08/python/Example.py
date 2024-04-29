import matplotlib.pyplot as plt
import numpy as np

# Main Variables
center = {'x': 2, 'y': 3}
radius1 = 3
radius2 = 5

# Theta for circles
theta = np.linspace(0, 2*np.pi, 100)

# First Circle
circle1_x = center['x'] + radius1 * np.cos(theta)
circle1_y = center['y'] + radius1 * np.sin(theta)

# Second circle
circle2_x = center['x'] + radius2 * np.cos(theta)
circle2_y = center['y'] + radius2 * np.sin(theta)

# Building plot
plt.figure(figsize=(8, 5))

plt.plot(circle1_x, circle1_y, label='Circle 1 (R = 3)')
plt.plot(circle2_x, circle2_y, label='Circle 2 (R = 5)')

plt.scatter(4, 4, color='red', label='A(4; 4)')
plt.scatter(2, 5, color='orange', label='B(2; 5)')
plt.scatter(2, 8, color='green', label='C(2; 8)')
plt.scatter(5, 1, color='purple', label='D(5; 1)')
plt.scatter(3, 6, color='cyan', label='E(3; 6)')

plt.xlabel('X')
plt.ylabel('Y')
plt.title('Example')

fill_x = np.append(circle1_x, np.flip(circle2_x))
fill_y = np.append(circle1_y, np.flip(circle2_y))
plt.fill(fill_x, fill_y, color='gray', alpha=0.2)

plt.axis('equal')
plt.legend(loc='center left', bbox_to_anchor=(0.8, 0.1), framealpha=1, fancybox=True, shadow=True)
plt.grid(True)

plt.show()
