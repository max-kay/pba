"""
this file contains a script for plotting the results of the Monte Carlo simmulation
"""
from os import listdir
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

files = listdir("out/csv")

files.sort()

for i, file in enumerate(files):
    print(f"({i+1}) {file}")

match input("choose file number (latest if empty or 0): "):
    case "":
        file = files[-1]
    case num:
        file = files[int(num) - 1]

df = pd.read_csv(f"out/csv/{file}", header=3)

df.sort_values(["j_prime", "temp"], inplace=True)
width = len(df["j_prime"].unique())
height = len(df["temp"].unique())

energies = df["energy"].values
variances = df["energy"].values
heat_capacity = variances / df["temp"].values ** 2


fig, axs = plt.subplots(1, 2, figsize=(12, 6))


im1 = axs[0].imshow(
    energies.reshape(width, height).T,
    cmap="viridis",
    extent=[
        df["j_prime"].min(),
        df["j_prime"].max(),
        np.min(np.log(df["temp"].values)),
        np.max(np.log(df["temp"].values)),
    ],
    aspect="auto",
    origin="lower",
)
axs[0].set_xlabel("J'")
axs[0].set_ylabel("ln(T')")
axs[0].set_title("Energy")
fig.colorbar(im1, ax=axs[0], label="Energy")


im2 = axs[1].imshow(
    -heat_capacity.reshape(width, height).T,
    cmap="viridis",
    extent=[
        df["j_prime"].min(),
        df["j_prime"].max(),
        np.min(np.log(df["temp"].values)),
        np.max(np.log(df["temp"].values)),
    ],
    aspect="auto",
    origin="lower",
)
axs[1].set_xlabel("J'")
axs[1].set_ylabel("ln(T')")
axs[1].set_title("Heat Capacity")
fig.colorbar(im2, ax=axs[1], label="Heat Capacity")

plt.tight_layout()
plt.show()
