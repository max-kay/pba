use nalgebra::{Matrix3, Vector3};
use std::{collections::HashMap, fs::File, io::Write, path::Path, usize};

use crate::array3d::Array3d;

struct CifWriter<'a, const S: usize> {
    cell_a: f32,
    cell_b: f32,
    cell_c: f32,
    armstrong_to_rel: Matrix3<f32>,
    naming: HashMap<i8, Option<Ion>>,
    grid: &'a Array3d<i8, S, S, S>,
    file: File,
}

impl<'a, const S: usize> CifWriter<'a, S> {
    pub fn new(
        grid: &'a Array3d<i8, S, S, S>,
        cell_a: f32,
        cell_b: f32,
        cell_c: f32,
        naming: HashMap<i8, Option<Ion>>,
        path: impl AsRef<Path>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            cell_a,
            cell_b,
            cell_c,
            armstrong_to_rel: Matrix3::from_diagonal(
                &[1.0 / cell_a, 1.0 / cell_b, 1.0 / cell_c].into(),
            ),
            naming,
            grid,
            file: File::create(path)?,
        })
    }
}

impl<const S: usize> CifWriter<'_, S> {
    fn get_header(&self) -> String {
        format!(
            "\
    data_struct
    _symmetry_space_group_name_H-M   'P 1'
    _cell_length_a                   {}
    _cell_length_b                   {}
    _cell_length_c                   {}
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
",
            self.cell_a, self.cell_b, self.cell_c
        )
    }

    pub fn write_to_file(&mut self) -> std::io::Result<()> {
        writeln!(self.file, "{}", self.get_header())?;
        for i in 0..(S as isize) {
            for j in 0..(S as isize) {
                for k in 0..(S as isize) {
                    let val = self.grid[(i, j, k)];
                    match self.naming.get(&val) {
                        Some(opt) => {
                            if let Some(ion) = opt {
                                self.place_ion(*ion, self.index_to_armstong(i, j, k))?
                            } else {
                                println!("omiited {}", val)
                            }
                        }
                        None => eprintln!("failed to get name for {}", val),
                    }
                }
            }
        }
        Ok(())
    }

    pub fn place_ion(&mut self, ion: Ion, coord_armstrong: Vector3<f32>) -> std::io::Result<()> {
        match ion {
            Ion::Singlet(name) => self.place(name, coord_armstrong)?,
            Ion::Cyanometalate {
                name,
                c_offset,
                n_offset,
            } => {
                self.place(name, coord_armstrong)?;

                // find nicer way to generate these unit vecs
                let all_axis: Vec<Vector3<f32>> = (0..3)
                    .map(|i| {
                        let mut e = [0.0, 0.0, 0.0];
                        e[i] = 1.0;
                        e.into()
                    })
                    .collect();
                for dir in all_axis {
                    let c_offset = dir.scale(c_offset);
                    self.place("C", coord_armstrong + c_offset)?;
                    self.place("C", coord_armstrong - c_offset)?;
                    let n_offset = dir.scale(n_offset);
                    self.place("N", coord_armstrong + n_offset)?;
                    self.place("N", coord_armstrong - n_offset)?;
                }
            }
        }
        Ok(())
    }

    fn place(&mut self, name: &'static str, pos_armstrong: Vector3<f32>) -> std::io::Result<()> {
        let rel_coords = self.armstrong_to_rel * pos_armstrong;
        writeln!(
            self.file,
            "{} {} {} {}",
            name, rel_coords.x, rel_coords.y, rel_coords.z
        )
    }

    pub fn index_to_armstong(&self, i: isize, j: isize, k: isize) -> Vector3<f32> {
        [
            i as f32 / S as f32 * self.cell_a,
            j as f32 / S as f32 * self.cell_b,
            k as f32 / S as f32 * self.cell_c,
        ]
        .into()
    }
}

#[derive(Clone, Copy)]
pub enum Ion {
    Singlet(&'static str),
    Cyanometalate {
        name: &'static str,
        c_offset: f32,
        n_offset: f32,
    },
}

pub fn write_cif<const S: usize>(
    grid: &Array3d<i8, S, S, S>,
    cell_a: f32,
    cell_b: f32,
    cell_c: f32,
    naming: HashMap<i8, Option<Ion>>,
    path: impl AsRef<Path>,
) -> std::io::Result<()> {
    let mut writer = CifWriter::new(grid, cell_a, cell_b, cell_c, naming, path)?;
    writer.write_to_file()
}
