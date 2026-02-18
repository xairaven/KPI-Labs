import matplotlib.pyplot as plt


# Arguments: 3 arrays (T, Y1, Y2)
def show(t, y1, y2):
    # Set id for plot
    plt.figure(0)

    # Plot functions
    plt.plot(t, y1, color="red")
    plt.plot(t, y2, color="blue")

    # Legend
    plt.legend(['y₁', 'y₂'])
    plt.xlabel("T", fontsize=16, loc="center")
    plt.ylabel("Results", fontsize=16, loc="center")

    # Show plot
    plt.show()


def show4(x, y1, y2, y3, y4):
    # Set id for plot
    plt.figure(0)

    # Plot functions
    plt.plot(x, y1, color="red", marker=".")
    plt.plot(x, y2, color="purple", marker=".")

    plt.plot(x, y3, color="blue", marker=".")
    plt.plot(x, y4, color="cyan", marker=".")

    # Legend
    plt.legend(["Model Y1", "Model Y2", "Experimental Y1", "Experimental Y2"])
    plt.xlabel("X", fontsize=16, loc="center")
    plt.ylabel("Results", fontsize=16, loc="center")

    # Show plot
    plt.show()
