use cif::{write_cif, Ion};
use rand::prelude::*;
use rand::rngs::StdRng;
use rand_seeder::Seeder;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

mod array3d;
use array3d::Array3d;
mod cif;

type Index = (isize, isize, isize);

// TODO get correct values
// these values are all in Angstrom
const DIST_MN_MN: f32 = 10.1;
const C_N_BOND: f32 = 1.3;
const C_CO: f32 = 1.2; // this value is invented.

// let M be the unmodifiable value of the array 0_i8
// let M' be the of the array 1_i8
// let M' be the vacancy of the array -1_i8
#[derive(Debug)]
pub struct Model<const S: usize> {
    /// The grid where the Ions are stored.
    /// 0 corresponds to the fixed metal ion
    /// 1 to the cyanometalate
    /// -1 to the vacancy at a cyanometalate site
    grid: Array3d<i8, S, S, S>,
    /// The interaction energy of nearest neighbours
    j_1: f32,
    /// TODO
    nearest_neighbours: i64,
    /// The interaction energy of nect nearest neighbours
    j_2: f32,
    /// TODO
    next_nearest_neighbours: i64,
    /// The hamiltonian of the system
    hamiltonian: f32,
    /// The random number generator
    rng: StdRng,
    /// The number of moves where the difference in energy was negative
    good_moves: u32,
    /// The number of accepted moves with the difference in energy >= 0
    bad_moves: u32,
    /// The number of rejected moves
    rejected_moves: u32,
}

impl<const S: usize> Model<S> {
    pub fn new(j_1: f32, j_2: f32, fill_frac: f64, seed: Option<&'static str>) -> Self {
        assert!(S % 2 == 0, "grid need to have side length 2*N");
        let mut rng = if let Some(seed) = seed {
            Seeder::from(seed).make_rng()
        } else {
            StdRng::from_entropy()
        };
        let mut out = Self {
            grid: Array3d::filled(|(x, y, z)| {
                if (x + y + z) % 2 == 0 {
                    0
                } else {
                    if rng.gen_bool(fill_frac) {
                        1
                    } else {
                        -1
                    }
                }
            }),
            j_1,
            nearest_neighbours: 0,
            j_2,
            next_nearest_neighbours: 0,
            hamiltonian: 0.0,
            good_moves: 0,
            bad_moves: 0,
            rejected_moves: 0,
            rng,
        };
        out.calc_hamiltonian();
        out
    }

    pub fn calc_hamiltonian(&mut self) -> f32 {
        self.nearest_neighbours = 0;
        self.next_nearest_neighbours = 0;
        for i in 0..(S as isize) {
            for j in 0..(S as isize) {
                for k in 0..((S / 2) as isize) {
                    let idx = (i, j, 2 * k + i % 2 + j % 2);
                    self.nearest_neighbours += self.diags_around(idx) as i64;
                    self.next_nearest_neighbours += self.axis_through(idx) as i64;
                }
            }
        }
        self.hamiltonian = self.nearest_neighbours as f32 * self.j_1
            + self.next_nearest_neighbours as f32 * self.j_2;
        self.hamiltonian
    }

    pub fn print_hamiltonian(&self) {
        println!("progressive hamiltonian: {}", self.hamiltonian);
        println!(
            "from neighbours: {}",
            self.nearest_neighbours as f32 * self.j_1
                + self.next_nearest_neighbours as f32 * self.j_2
        )
    }

    pub fn print_neighbours(&self) {
        println!("nearest neighbours sum: {}", self.nearest_neighbours);
        println!(
            "next nearest neighbours sum: {}",
            self.next_nearest_neighbours
        );
    }

    pub fn print_counters(&self) {
        println!("good moves: {}", self.good_moves);
        println!("bad moves: {}", self.bad_moves);
        println!("rejected moves: {}", self.rejected_moves);
    }

    pub fn fill_frac(&self) -> f64 {
        let mut counter = 0;
        for val in self.grid.as_flat_slice() {
            if *val == 1 {
                counter += 1;
            }
        }
        counter as f64 / (S * S * S / 2) as f64
    }
}

impl<const S: usize> Model<S> {
    /// The sum over the nearest neigbours around a metal ion.
    /// To remove over counting in each plane the first of the two indexes
    /// is only allowed to increase or stay the same
    #[inline]
    fn diags_around(&self, idx: Index) -> i8 {
        let (i, j, k) = idx;
        // next nearest neighbours in the jk plane
        let mut sum = self.grid[(i, j + 1, k)] * self.grid[(i, j, k + 1)]
            + self.grid[(i, j, k - 1)] * self.grid[(i, j + 1, k)];

        // next nearest neighbours in the ik plane
        sum += self.grid[(i + 1, j, k)] * self.grid[(i, j, k + 1)]
            + self.grid[(i, j, k - 1)] * self.grid[(i + 1, j, k)];

        // next nearest neighbours in the ij plane
        sum += self.grid[(i + 1, j, k)] * self.grid[(i, j + 1, k)]
            + self.grid[(i, j - 1, k)] * self.grid[(i + 1, j, k)];
        sum
    }

    /// The sum over the next nearest neighbours around a metal ion.
    #[inline]
    fn axis_through(&self, idx: Index) -> i8 {
        let (i, j, k) = idx;
        self.grid[(i - 1, j, k)] * self.grid[(i + 1, j, k)]
            + self.grid[(i, j - 1, k)] * self.grid[(i, j + 1, k)]
            + self.grid[(i, j, k - 1)] * self.grid[(i, j, k + 1)]
    }

    /// The sum over all nearest neighbours to idx
    #[inline]
    fn diags_from(&self, idx: Index) -> i8 {
        let (i, j, k) = idx;
        let mut sum = self.grid[(i, j, k)] * self.grid[(i, j + 1, k + 1)]
            + self.grid[(i, j, k)] * self.grid[(i, j + 1, k - 1)]
            + self.grid[(i, j, k)] * self.grid[(i, j - 1, k - 1)]
            + self.grid[(i, j, k)] * self.grid[(i, j - 1, k + 1)];
        sum += self.grid[(i, j, k)] * self.grid[(i + 1, j, k + 1)]
            + self.grid[(i, j, k)] * self.grid[(i + 1, j, k - 1)]
            + self.grid[(i, j, k)] * self.grid[(i - 1, j, k - 1)]
            + self.grid[(i, j, k)] * self.grid[(i - 1, j, k + 1)];
        sum += self.grid[(i, j, k)] * self.grid[(i + 1, j + 1, k)]
            + self.grid[(i, j, k)] * self.grid[(i + 1, j - 1, k)]
            + self.grid[(i, j, k)] * self.grid[(i - 1, j - 1, k)]
            + self.grid[(i, j, k)] * self.grid[(i - 1, j + 1, k)];
        sum
    }

    /// The sum over all nest neares neighbours to idx
    #[inline]
    fn axis_from(&self, idx: Index) -> i8 {
        let (i, j, k) = idx;
        self.grid[(i, j, k)] * self.grid[(i + 2, j, k)]
            + self.grid[(i, j, k)] * self.grid[(i - 2, j, k)]
            + self.grid[(i, j, k)] * self.grid[(i, j + 2, k)]
            + self.grid[(i, j, k)] * self.grid[(i, j - 2, k)]
            + self.grid[(i, j, k)] * self.grid[(i, j, k + 2)]
            + self.grid[(i, j, k)] * self.grid[(i, j, k - 2)]
    }
}

impl<const S: usize> Model<S> {
    fn uniform_idx(&mut self) -> Index {
        let i = self.rng.gen_range(0..S as isize);
        let j = self.rng.gen_range(0..S as isize);
        let k = 2 * self.rng.gen_range(0..(S / 2) as isize) + i % 2 + j % 2 + 1;
        (i, j, k)
    }

    fn choose_swap_pos(&mut self) -> (Index, Index) {
        let idx_1 = self.uniform_idx();
        let mut idx_2 = self.uniform_idx();
        while self.grid[idx_1] == self.grid[idx_2] {
            idx_2 = self.uniform_idx()
        }
        (idx_1, idx_2)
    }

    pub fn monte_carlo_step(&mut self, beta: f32) {
        let (idx_1, idx_2) = self.choose_swap_pos();
        let old_n_neighbours = self.diags_from(idx_1) + self.diags_from(idx_2);
        let old_n_n_neighbours = self.axis_from(idx_1) + self.axis_from(idx_2);
        let old_energy = self.j_1 * old_n_neighbours as f32 + self.j_2 * old_n_n_neighbours as f32;
        self.swap(idx_1, idx_2);
        let new_n_neighbours = self.diags_from(idx_1) + self.diags_from(idx_2);
        let new_n_n_neighbours = self.axis_from(idx_1) + self.axis_from(idx_2);
        let new_energy = self.j_1 * new_n_neighbours as f32 + self.j_2 * new_n_n_neighbours as f32;
        let delta_e = new_energy - old_energy;
        if delta_e <= 0.0 {
            self.hamiltonian += delta_e;
            self.nearest_neighbours += (new_n_neighbours - old_n_neighbours) as i64;
            self.next_nearest_neighbours += (new_n_n_neighbours - old_n_n_neighbours) as i64;
            self.good_moves += 1;
        } else if self.rng.gen::<f32>() < (-beta * delta_e).exp() {
            self.hamiltonian += delta_e;
            self.nearest_neighbours += (new_n_neighbours - old_n_neighbours) as i64;
            self.next_nearest_neighbours += (new_n_n_neighbours - old_n_n_neighbours) as i64;
            self.bad_moves += 1;
        } else {
            self.swap(idx_1, idx_2);
            self.rejected_moves += 1;
        }
    }

    fn swap(&mut self, idx_1: Index, idx_2: Index) {
        let temp = self.grid[idx_1];
        self.grid[idx_1] = self.grid[idx_2];
        self.grid[idx_2] = temp;
    }

    pub fn write_to_cif(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let side = (S / 2) as f32 * DIST_MN_MN;

        let naming = HashMap::from([
            (0, Some(Ion::Singlet("Mn"))),
            (
                -1,
                Some(Ion::Cyanometalate {
                    name: "Co",
                    c_offset: C_CO,
                    n_offset: C_CO + C_N_BOND,
                }),
            ),
            (
                1,
                Some(Ion::Cyanometalate {
                    name: "Co",
                    c_offset: C_CO,
                    n_offset: C_CO + C_N_BOND,
                }),
            ),
        ]);
        write_cif(&self.grid, side, side, side, naming, path)
    }
}
