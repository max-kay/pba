use rand::prelude::*;
use rand::rngs::StdRng;
use rand_seeder::Seeder;
use std::fmt::Debug;
use std::io::Write;
use std::path::Path;

mod array3d;
use array3d::Array3d;

type Index = (isize, isize, isize);

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
        };
        out.get_tot_energy();
        out
    }

    pub fn get_tot_energy(&mut self) -> f32 {
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
        let (x, y, z) = idx;
        let mut sum = self.grid[(x, y + 1, z)] * self.grid[(x, y, z + 1)]
            + self.grid[(x, y, z + 1)] * self.grid[(x, y - 1, z)]
            + self.grid[(x, y - 1, z)] * self.grid[(x, y, z - 1)]
            + self.grid[(x, y, z - 1)] * self.grid[(x, y + 1, z)];

        sum += self.grid[(x + 1, y, z)] * self.grid[(x, y, z + 1)]
            + self.grid[(x, y, z + 1)] * self.grid[(x - 1, y, z)]
            + self.grid[(x - 1, y, z)] * self.grid[(x, y, z - 1)]
            + self.grid[(x, y, z - 1)] * self.grid[(x + 1, y, z)];

        sum += self.grid[(x + 1, y, z)] * self.grid[(x, y + 1, z)]
            + self.grid[(x, y + 1, z)] * self.grid[(x - 1, y, z)]
            + self.grid[(x - 1, y, z)] * self.grid[(x, y - 1, z)]
            + self.grid[(x, y - 1, z)] * self.grid[(x + 1, y, z)];
        sum
    }

    #[inline]
    fn diags_from(&self, idx: Index) -> i8 {
        let (x, y, z) = idx;
        let mut sum = self.grid[(x, y, z)] * self.grid[(x, y + 1, z + 1)]
            + self.grid[(x, y, z)] * self.grid[(x, y + 1, z - 1)]
            + self.grid[(x, y, z)] * self.grid[(x, y - 1, z - 1)]
            + self.grid[(x, y, z)] * self.grid[(x, y - 1, z + 1)];
        sum += self.grid[(x, y, z)] * self.grid[(x + 1, y, z + 1)]
            + self.grid[(x, y, z)] * self.grid[(x + 1, y, z - 1)]
            + self.grid[(x, y, z)] * self.grid[(x - 1, y, z - 1)]
            + self.grid[(x, y, z)] * self.grid[(x - 1, y, z + 1)];
        sum += self.grid[(x, y, z)] * self.grid[(x + 1, y + 1, z)]
            + self.grid[(x, y, z)] * self.grid[(x + 1, y - 1, z)]
            + self.grid[(x, y, z)] * self.grid[(x - 1, y - 1, z)]
            + self.grid[(x, y, z)] * self.grid[(x - 1, y + 1, z)];
        sum
    }

    #[inline]
    fn axis_through(&self, idx: Index) -> i8 {
        let (x, y, z) = idx;
        self.grid[(x - 1, y, z)] * self.grid[(x + 1, y, z)]
            + self.grid[(x, y - 1, z)] * self.grid[(x, y + 1, z)]
            + self.grid[(x, y, z - 1)] * self.grid[(x, y, z + 1)]
    }
    #[inline]
    fn axis_from(&self, idx: Index) -> i8 {
        let (x, y, z) = idx;
        self.grid[(x, y, z)] * self.grid[(x + 2, y, z)]
            + self.grid[(x, y, z)] * self.grid[(x - 2, y, z)]
            + self.grid[(x, y, z)] * self.grid[(x, y + 2, z)]
            + self.grid[(x, y, z)] * self.grid[(x, y - 2, z)]
            + self.grid[(x, y, z)] * self.grid[(x, y, z + 2)]
            + self.grid[(x, y, z)] * self.grid[(x, y, z - 2)]
    }
}
impl<const S: usize> Model<S> {
    fn uniform_idx(&mut self) -> Index {
        let x = self.rng.gen_range(0..S as isize);
        let y = self.rng.gen_range(0..S as isize);
        let z = 2 * self.rng.gen_range(0..(S / 2) as isize) + x % 2 + y % 2 + 1;
        (x, y, z)
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
        let delta_e = e_before - self.energy_around(idx_1) - self.energy_around(idx_2);
        if delta_e <= 0.0 || (self.rng.gen::<f32>() < (-beta * delta_e).exp()) {
            self.hamiltonian += delta_e
        } else {
            self.swap(idx_1, idx_2)
        }
    }

    fn swap(&mut self, idx_1: Index, idx_2: Index) {
        let temp = self.grid[idx_1];
        self.grid[idx_1] = self.grid[idx_2];
        self.grid[idx_2] = temp;
    }
}

impl<const S: usize> Model<S> {
    pub fn save_cif_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        writeln!(file, "{}", CIF_HEADER)?;
        let len = 1.0 / S as f32;
        for i in 0..(S as isize) {
            for j in 0..(S as isize) {
                for k in 0..(S as isize) {
                    match self.grid[(i, j, k)] {
                        0 => writeln!(
                            file,
                            "Mn {} {} {}",
                            i as f32 * len,
                            j as f32 * len,
                            k as f32 * len
                        )?,
                        1 => writeln!(
                            file,
                            "Co {} {} {}",
                            i as f32 * len,
                            j as f32 * len,
                            k as f32 * len
                        )?,
                        -1 => (),
                        _ => unreachable!(),
                    };
                }
            }
        }
        Ok(())
    }
}

const CIF_HEADER: &'static str = "\
    data_struct
    _symmetry_space_group_name_H-M   'P 1'
    _cell_length_a                   51.70000000
    _cell_length_b                   51.70000000
    _cell_length_c                   51.70000000
    _cell_angle_alpha                90.000000
    _cell_angle_beta                 90.000000
    _cell_angle_gamma                90.000000
    loop_
    _symmetry_equiv_pos_as_xyz
    x,y,z
    loop_
    _atom_site_label
    _atom_site_fract_x
    _atom_site_fract_y
    _atom_site_fract_z
";
