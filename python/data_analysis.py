"""
This module contains function to read and work with cif files gemmi outputs and the yell format
"""
import re
import os
import subprocess
import multiprocessing
import numpy as np
import h5py
from matplotlib import pyplot as plt


class Diffraction:
    """
    A class for holding, accessing and converting Diffraction data
    """

    def __init__(
        self,
        frac_hkl_max: np.ndarray,
        intensities: np.ndarray,
        super_cells: np.ndarray,
    ):
        self.frac_hkl_max = np.array(frac_hkl_max)
        self.intensities = intensities
        self.super_cells = super_cells

    @classmethod
    def add_from_files(cls, paths: list[str], supercells: np.ndarray):
        """
        reads the list of sf files provided and adds the values
        failes if the (hkl) have different ranges
        """
        first_int_hkl, summed_intensities = read_sf_file(paths[0])
        for path in paths[1:]:
            int_hkl, intensities = read_sf_file(path)
            assert (first_int_hkl == int_hkl).all()
            summed_intensities += intensities

        int_hkl_max, intensities = Diffraction.bring_into_shape(
            first_int_hkl, summed_intensities
        )
        return cls(int_hkl_max / supercells, intensities, supercells)

    @classmethod
    def read_sf(cls, path: str, supercells: np.ndarray):
        """
        reads an .sf file
        """
        int_hkl, intensities = read_sf_file(path)
        int_hkl_max, intensities = Diffraction.bring_into_shape(int_hkl, intensities)
        return cls(int_hkl_max / supercells, intensities, supercells)

    @staticmethod
    def bring_into_shape(
        integer_hkls: np.ndarray, intensities: np.ndarray
    ) -> tuple[np.ndarray, np.ndarray]:
        """
        Takes the integer hkl and intensitied as generated by gemmi
        and puts the intensities into the correct shape
        """
        int_hkl_max = np.amax(np.abs(integer_hkls), axis=0)

        int_h_max, int_k_max, int_l_max = int_hkl_max
        new_intensities = np.empty(int_hkl_max * 2 + 1)
        new_intensities[:] = np.nan
        for (h, k, l), intensitiy in zip(integer_hkls, intensities):
            new_intensities[int_h_max + h, int_k_max + k, int_l_max + l] = intensitiy
            new_intensities[int_h_max - h, int_k_max - k, int_l_max - l] = intensitiy
        return np.array([int_h_max, int_k_max, int_l_max]), new_intensities

    def has_nan(self) -> bool:
        """
        checks if the diffraction data has nan
        """
        return np.isnan(self.intensities).all()

    def get_averaged_section(self, value) -> np.ndarray:
        """
        get a section at value where all directions +- are averaged
        """
        # TODO should I apply mirror and rotational symmetries?
        # this pattern is inversion symmetric. Can I use this?
        sum = self.get_h_section(value)
        sum += self.get_h_section(-value)
        sum += self.get_k_section(value)
        sum += self.get_k_section(-value)
        sum += self.get_l_section(value)
        sum += self.get_l_section(-value)
        return sum / 6

    def get_h_section(self, h_value) -> np.ndarray:
        """
        get a section of the diffraction pattern at the specified h_value
        """
        int_h_value = int(round(h_value * self.super_cells[0]))
        int_h_max, _int_k_max, _int_l_max = np.round(
            self.frac_hkl_max * self.super_cells
        ).astype(np.int32)
        return self.intensities[int_h_max + int_h_value, :, :]

    def get_k_section(self, k_value) -> np.ndarray:
        """
        get a section of the diffraction pattern at the specified l_value
        """
        int_k_value = int(round(k_value * self.super_cells[1]))
        _int_h_max, int_k_max, _int_l_max = np.round(
            self.frac_hkl_max * self.super_cells
        ).astype(np.int32)
        return self.intensities[:, int_k_max + int_k_value, :]

    def get_l_section(self, l_value) -> np.ndarray:
        """
        get a section of the diffraction pattern at the specified l_value
        """
        int_l_value = int(round(l_value * self.super_cells[2]))
        _int_h_max, _int_k_max, int_l_max = np.round(
            self.frac_hkl_max * self.super_cells
        ).astype(np.int32)
        return self.intensities[:, :, int_l_max + int_l_value]

    def plot_l_section(self, fig, ax, l_value):
        """
        plots an l-section if the diffraction data
        """
        section = self.get_l_section(l_value)
        int_h_max, int_k_max, _int_l_max = np.round(
            self.frac_hkl_max * self.super_cells
        ).astype(np.int32)
        im = ax.imshow(
            section,
            extent=[
                -int_h_max,
                int_h_max,
                -int_k_max,
                int_k_max,
            ],
            cmap="gray",
        )
        fig.colorbar(im)
        fig.tight_layout()

    def save_l_section(self, l_value: float, path: str):
        """
        save a section of the specified l_value
        """
        section = self.get_l_section(l_value)
        plt.imsave(
            path,
            section,
            cmap="gray_r",
            vmin=np.nanmin(section),
            vmax=np.nanmax(section),
        )

    def save_yell(self, path):
        """
        saves the diffraction pattern into the yell format (a .h5 file)
        """
        output = h5py.File(path, "w")
        output["data"] = self.intensities  # the data
        output["format"] = b"Yell 1.0"  # formatting string
        # whether the data is in real or reciprocal space. Scattering data is in reciprocal space
        output["is_direct"] = 0
        # the smallest hkl index for this dataset respecting supercells fractional values
        output["lower_limits"] = -self.frac_hkl_max
        output["step_sizes"] = 1 / self.super_cells
        output["unit_cell"] = [10.0003, 10.0003, 10.0003, 90, 90, 90]
        output.close()

    @classmethod
    def open_yell(cls, path):
        """
        reads the data from a .h5 file using the Yell format
        """
        file = h5py.File(path, "r")
        intensities = np.array(file["data"])
        frac_hkl_max = -np.array(file["lower_limits"])
        super_cells = np.round(1 / np.array(file["step_sizes"]))
        file.close()
        return cls(frac_hkl_max, intensities, super_cells)

    @classmethod
    def generate_from_mmcif(cls, path: str, supercells: np.ndarray):
        """
        runs gemmi on the file provided and parses the input
        """
        string = get_sf_string(path)
        if not string:
            return
        int_hkl_max, intesities = Diffraction.bring_into_shape(*parse_sf_string(string))
        return cls(int_hkl_max / supercells, intesities, supercells)


def read_sf_file(path: str) -> tuple[np.ndarray, np.ndarray]:
    """
    reads a .sf file
    """
    with open(path, "r", encoding="utf8") as file:
        string = file.read()
    return parse_sf_string(string)


def parse_sf_string(string: str) -> tuple[np.ndarray, np.ndarray]:
    """
    parses the string from an .sf file
    """
    lines = string.splitlines()

    hkls = []
    intensities = []

    for line in lines:
        numbers = re.split(r"\)?\s+", line.strip()[1:])
        hkl = [int(n) for n in numbers[:3]]
        abs_val = float(numbers[3])
        # phase = float(numbers[4])
        intensities.append(abs_val)
        hkls.append(hkl)

    return np.array(hkls, dtype=np.int32), np.array(intensities) ** 2


def get_sf_string(mmcif_file: str) -> str:
    """
    runs gemmi sfcalc on the provided file and returns the output as a string
    """
    try:
        print("running gemmi sfcalc on", mmcif_file)
        result = subprocess.run(
            ["gemmi", "sfcalc", "--dmin=1", mmcif_file],
            stdout=subprocess.PIPE,
            check=True,
        )
        print("ran successfully")
        return result.stdout.decode()
    except subprocess.CalledProcessError:
        print(f"failed to run gemmi on {mmcif_file}")
        return None


def get_runs() -> list[str]:
    """
    returns a list of the names of the runs
    """
    names = []
    for name in os.listdir("out/mmcif"):
        if os.path.isdir(f"out/mmcif/{name}"):
            names.append(name)
    return names


def get_file_names(run: str) -> list[str]:
    """
    returns a list of the values used in the run
    """
    names = os.listdir(f"out/mmcif/{run}")
    return list(map(lambda string: string.removesuffix(".mmcif"), names))


def analyze_mmcif(run: str, name: str, supercells: str):
    """
    calculates all diffraction patterns of a run and saves them as .h5 files in the yell format
    """
    if not os.path.exists(f"out/h5/{run}/{name}.h5"):
        diffraction = Diffraction.generate_from_mmcif(
            f"out/mmcif/{run}/{name}.mmcif", supercells
        )
        diffraction.save_yell(f"out/h5/{run}/{name}.h5")
    else:
        print(f"{run} {name} is already analyzed")


def analyze_run(run: str):
    """
    calculates all diffraction patterns of a run and saves them as .h5 files in the yell format
    """
    with open(f"out/csv/{run}.csv", mode="r", encoding="utf8") as file:
        file.readline()
        supercells = int(file.readline().split(" ")[0])
    supercells = np.array([supercells] * 3)
    os.makedirs(f"out/h5/{run}", exist_ok=True)
    names = get_file_names(run)
    lenght = len(names)
    for i, name in enumerate(names):
        print()
        print(f"file {i+1} of {lenght}")
        analyze_mmcif(run, name, supercells)


def test_analyze_mmcif():
    """
    test analyze_mmcif
    """
    run = "2023-11-16_15-21"
    name = "j_0_t_0.3807861"
    analyze_mmcif(run, name, np.array([4] * 3))


def analyze_mmcif_wrapper(args):
    """
    Wrapper function for analyze_mmcif to be used with multiprocessing
    """
    run, name, supercells = args
    analyze_mmcif(run, name, supercells)


def analyze_run_parallel(run: str):
    """
    Calculates all diffraction patterns of a run using multiprocessing
    and saves them as .h5 files in the yell format
    Additionally, saves the hk0 section as pngs
    """
    with open(f"out/csv/{run}.csv", mode="r", encoding="utf8") as file:
        file.readline()
        supercells = int(file.readline().split(" ")[0])
    supercells = np.array([supercells] * 3)
    os.makedirs(f"out/h5/{run}", exist_ok=True)
    os.makedirs(f"out/hk0/{run}", exist_ok=True)
    names = get_file_names(run)

    num_processes = multiprocessing.cpu_count()
    pool = multiprocessing.Pool(processes=num_processes)
    args_list = [(run, name, supercells) for name in names]
    pool.map(analyze_mmcif_wrapper, args_list)
    pool.close()
    pool.join()


def make_map_from_yells(
    run: str,
    value: float,
    energy_values: int | None = None,
    temp_values: int | None = None,
):
    temps, energies = get_sorted_energies_and_temps(run)

    t_step = 1
    if temp_values:
        t_step = len(temps) // temp_values
    temps = temps[::t_step]

    energy_step = 1
    if energy_values:
        energy_step = len(energies) // energy_values
    energies = energies[::energy_step]

    diffraction = Diffraction.open_yell(
        f"out/h5/{run}/j_{energies[-1]}_t_{temps[-1]}.h5"
    )
    w, h = find_biggest_non_nan_square(diffraction.get_averaged_section(value)).shape
    del diffraction
    assert w == h
    img = np.empty((h * len(temps), w * len(energies)), dtype=np.float64)
    img[:] = np.nan

    for x, j in enumerate(energies):
        for y, temp in enumerate(temps):
            try:
                diffraction = Diffraction.open_yell(f"out/h5/{run}/j_{j}_t_{temp}.h5")
                img[
                    y * h : (y + 1) * h,
                    x * w : (x + 1) * w,
                ] = find_biggest_non_nan_square(diffraction.get_averaged_section(value))
                del diffraction
            except FileNotFoundError:
                pass
    os.makedirs(f"out/maps/{run}", exist_ok=True)
    img = np.log(img)
    plt.imsave(
        f"out/maps/{run}/hk{value}.png",
        img,
        cmap="gray",
        origin="upper",
        vmin=np.nanmin(img),
        vmax=np.nanmax(img),
    )


def get_sorted_energies_and_temps(run):
    """
    returns the sorted temperatures and Js aviable for this run
    """
    temps = []
    energies = []
    for name in get_file_names(run):
        _, j, _, t = name.split("_")
        if t not in temps:
            temps.append(t)
        if j not in energies:
            energies.append(j)

    f_temps = list(zip([float(temp) for temp in temps], temps))
    f_temps.sort(key=lambda tup: tup[1], reverse=True)
    f_energies = list(zip([float(j) for j in energies], energies))
    f_energies.sort(key=lambda tup: tup[0])
    temps = [tup[1] for tup in f_temps]
    energies = [tup[1] for tup in f_energies]
    return temps, energies


def shrink_to_fit_non_nan(array: np.ndarray) -> np.ndarray:
    """
    takes an array which has a border of NaN values surrounding the concrete values
    and returns an array shrunk to the size such that all non nan values are contained and the shape
    of the values is preserved
    """
    non_nan_indices = np.argwhere(~np.isnan(array))
    print(non_nan_indices)
    (min_row, min_col), (max_row, max_col) = non_nan_indices.min(
        0
    ), non_nan_indices.max(0)

    # Extract the inner shape preserving the contents
    return array[min_row : max_row + 1, min_col : max_col + 1]


def find_biggest_non_nan_square(array: np.ndarray) -> np.ndarray:
    """
    shrinks the array to the biggest possible square that only contains non nan values
    assumes the nan values are 4m symmetric and the shape formed by the non nan vallues is convex
    """
    w, h = array.shape
    assert w == h
    offset = 0
    for i in range(w):
        if not np.isnan(array[i, i]):
            offset = i
            break
    return array[offset : w - offset, offset : w - offset]


# if __name__ == "__main__":
#     # run = "2023-11-16_16-54"
#     run = "2023-11-16_17-01"
#     # analyze_run_parallel(run)
#     for value in range(10):
#         make_map_from_yells(run, value)
