"""
this file contains a script for plotting the results of the Monte Carlo simmulation
"""
from os import listdir
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from matplotlib import rcParams

from data_analysis import get_sorted_energies_and_temps, Diffraction

rcParams["font.family"] = "serif"
# rcParams["font.serif"] = ["Noto Serif Regular"]

# files = listdir("out/csv")

# files.sort()

# for i, file in enumerate(files):
#     print(f"({i+1}) {file}")

# match input("choose file number (latest if empty): "):
#     case "":
#         file = files[-1]
#     case num:
#         file = files[int(num) - 1]
run = "2023-11-16_17-01"
file = "2023-11-16_17-01.csv"

df = pd.read_csv(f"out/csv/{file}", header=3)

df.sort_values(["j_prime", "temp"], inplace=True)
width = len(df["j_prime"].unique())
height = len(df["temp"].unique())

energies = df["energy"].values.reshape(width, height)
variances = df["variance"].values
heat_capacity = (variances / df["temp"].values ** 2).reshape(width, height)
temps = df["temp"].values.reshape(width, height)
d_energy = np.gradient(energies, axis=1) / np.gradient(temps, axis=1)

fig, axs = plt.subplots(1, 2, figsize=(12, 6))


example_j_primes = [0.4000001, 2.0, 5.6, 1.1999998, 4.0, 4.8]
example_t_primes = [1.1663113, 5.4320016, 25.299109, 0.46336934, 0.46336934, 0.15784304]
annotations = [chr(i + ord("a")) + ")" for i in range(len(example_j_primes))]


im1 = axs[0].imshow(
    energies.T,
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
axs[0].set_title("Internal Energy U")
fig.colorbar(
    im1,
    ax=axs[0],
    label="U",
)

ln_d_energy = np.log(d_energy)
minimum = np.nanmin(ln_d_energy[np.isfinite(ln_d_energy)])
ln_d_energy[~np.isfinite(ln_d_energy)] = minimum

im2 = axs[1].imshow(
    ln_d_energy.T,
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
axs[1].set_title("ln(dU/dT)")
fig.colorbar(
    im2,
    ax=axs[1],
    label="ln(dU/dT)",
)

for string, point in zip(
    annotations, zip(example_j_primes, [np.log(val) for val in example_t_primes])
):
    for ax in axs:
        ax.text(
            x=point[0],
            y=point[1],
            s=string,
            backgroundcolor="w",
            horizontalalignment="center",
            verticalalignment="center",
        )


plt.tight_layout()
plt.savefig(f"figs/{run}.svg")


e_str, temp_str = get_sorted_energies_and_temps(run)

e_float = [float(val) for val in e_str]
temp_float = [float(val) for val in temp_str]

new_temp = []
for t_prime in example_t_primes:
    for i in range(len(temp_float)):
        if i == len(temp_float) - 1:
            new_temp.append(temp_str[i])
            break
        if abs(t_prime - temp_float[i]) < abs(t_prime - temp_float[i + 1]):
            new_temp.append(temp_str[i])
            break

new_e = []
for t_prime in example_j_primes:
    for i in range(len(e_float)):
        if i == len(e_float) - 1:
            new_e.append(e_str[i])
            break

        if abs(t_prime - e_float[i]) < abs(t_prime - e_float[i + 1]):
            new_e.append(e_str[i])
            break


file_names = [f"j_{val[0]}_t_{val[1]}.h5" for val in zip(new_e, new_temp)]

for i, file in enumerate(file_names):
    diff = Diffraction.read_yell(f"out/h5/{run}/{file}")
    section = np.log(diff.get_averaged_section(0.0))
    plt.imsave(
        f"figs/examples/{chr(i + ord('a'))}.png",
        section,
        cmap="gray_r",
        origin="upper",
        vmin=np.nanmin(section),
        vmax=np.nanmax(section),
    )
