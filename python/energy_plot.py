import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

df = pd.read_csv("out.csv", header=1)

df.sort_values(["temp", "j_prime"], inplace=True)
width = len(df["j_prime"].unique())
height = len(df["temp"].unique())

energies = df["energy"].values
variances = df["energy"].values
heat_capacity = variances / df["temp"].values ** 2


fig, axs = plt.subplots(1, 2, figsize=(12, 6))


im1 = axs[0].imshow(
    energies.reshape(width, height),
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
    heat_capacity.reshape(width, height),
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
