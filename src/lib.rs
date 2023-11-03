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
    grid: Array3d<i8, S, S, S>,
    j_1: f32,
    j_2: f32,
    rng: StdRng,
    hamiltonian: f32,
    good_moves: u32,
    bad_moves: u32,
    rejected_moves: u32,
}

impl<const S: usize> Model<S> {
    pub fn new(j_1: f32, j_2: f32, fill_frac: f64, seed: Option<&'static str>) -> Self {
        assert!(S % 2 == 0, "grid need to have length 2*N");
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
            j_2,
            rng,
            hamiltonian: 0.0,
            good_moves: 0,
            bad_moves: 0,
            rejected_moves: 0,
        };
        out.calc_hamiltonian();
        out
    }

    pub fn get_hamiltonian(&self) -> f32 {
        self.hamiltonian
    }
    pub fn calc_hamiltonian(&mut self) -> f32 {
        let mut res = 0.0;
        for i in 0..(S as isize) {
            for j in 0..(S as isize) {
                for k in 0..((S / 2) as isize) {
                    let idx = (i, j, 2 * k + i % 2 + j % 2);
                    res += self.j_1 / 2.0 * self.diags_around(idx) as f32;
                    res += self.j_2 * self.axis_through(idx) as f32;
                }
            }
        }
        self.hamiltonian = res;
        res
    }

    pub fn print_counters(&self) {
        println!("good moves: {}", self.good_moves);
        println!("bad moves: {}", self.bad_moves);
        println!("rejected moves: {}", self.rejected_moves);
    }

    // vacancy position !!!!
    pub fn energy_around(&self, idx: Index) -> f32 {
        self.j_1 * self.diags_from(idx) as f32 + self.j_2 * self.axis_from(idx) as f32
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

    #[inline]
    fn diags_around(&self, idx: Index) -> i8 {
        let (i, j, k) = idx;
        let mut sum = self.grid[(i, j + 1, k)] * self.grid[(i, j, k + 1)]
            + self.grid[(i, j, k + 1)] * self.grid[(i, j - 1, k)]
            + self.grid[(i, j - 1, k)] * self.grid[(i, j, k - 1)]
            + self.grid[(i, j, k - 1)] * self.grid[(i, j + 1, k)];

        sum += self.grid[(i + 1, j, k)] * self.grid[(i, j, k + 1)]
            + self.grid[(i, j, k + 1)] * self.grid[(i - 1, j, k)]
            + self.grid[(i - 1, j, k)] * self.grid[(i, j, k - 1)]
            + self.grid[(i, j, k - 1)] * self.grid[(i + 1, j, k)];

        sum += self.grid[(i + 1, j, k)] * self.grid[(i, j + 1, k)]
            + self.grid[(i, j + 1, k)] * self.grid[(i - 1, j, k)]
            + self.grid[(i - 1, j, k)] * self.grid[(i, j - 1, k)]
            + self.grid[(i, j - 1, k)] * self.grid[(i + 1, j, k)];
        sum
    }

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

    #[inline]
    fn axis_through(&self, idx: Index) -> i8 {
        let (i, j, k) = idx;
        self.grid[(i - 1, j, k)] * self.grid[(i + 1, j, k)]
            + self.grid[(i, j - 1, k)] * self.grid[(i, j + 1, k)]
            + self.grid[(i, j, k - 1)] * self.grid[(i, j, k + 1)]
    }
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
        let e_before = self.energy_around(idx_1) + self.energy_around(idx_2);
        self.swap(idx_1, idx_2);
        let delta_e = e_before - (self.energy_around(idx_1) + self.energy_around(idx_2));
        if delta_e <= 0.0 {
            self.hamiltonian += delta_e;
            self.good_moves += 1;
        } else if self.rng.gen::<f32>() < (-beta * delta_e).exp() {
            self.hamiltonian += delta_e;
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
                Some(Ion::CyanoMetal {
                    name: "Co",
                    c_offset: C_CO,
                    n_offset: C_CO + C_N_BOND,
                }),
            ),
            (
                1,
                Some(Ion::CyanoMetal {
                    name: "Co",
                    c_offset: C_CO,
                    n_offset: C_CO + C_N_BOND,
                }),
            ),
        ]);
        write_cif(&self.grid, side, side, side, naming, path)
    }
}
