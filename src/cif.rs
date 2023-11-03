use std::{collections::HashMap, fs::File, io::Write, path::Path};

use crate::array3d::Array3d;

pub struct CifWriter<'a, const S: usize> {
    pub cell_a: f32,
    pub cell_b: f32,
    pub cell_c: f32,
    pub naming: HashMap<i8, Option<Ion>>,
    pub grid: &'a Array3d<i8, S, S, S>,
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
    pub fn write_to_file(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        writeln!(file, "{}", self.get_header())?;
        for i in 0..(S as isize) {
            for j in 0..(S as isize) {
                for k in 0..(S as isize) {
                    let val = self.grid[(i, j, k)];
                    match self.naming.get(&val) {
                        Some(opt) => {
                            if let Some(ion) = opt {
                                ion.place_in_cif(
                                    &mut file,
                                    self.index_to_armstong(i, j, k),
                                    |x, y, z| self.armstrong_to_rel(x, y, z),
                                )?
                            }
                        }
                        None => eprintln!("failed to get name for {}", val),
                    }
                }
            }
        }
        Ok(())
    }

    pub fn index_to_armstong(&self, i: isize, j: isize, k: isize) -> (f32, f32, f32) {
        (
            i as f32 / S as f32 * self.cell_a,
            j as f32 / S as f32 * self.cell_b,
            k as f32 / S as f32 * self.cell_c,
        )
    }

    pub fn armstrong_to_rel(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        (x / self.cell_a, y / self.cell_b, z / self.cell_c)
    }
}

pub enum Ion {
    Singlet(&'static str),
}

impl Ion {
    pub fn place_in_cif(
        &self,
        file: &mut File,
        coord: (f32, f32, f32),
        coord_conversion: impl Fn(f32, f32, f32) -> (f32, f32, f32),
    ) -> std::io::Result<()> {
        let (x, y, z) = coord;
        match self {
            Self::Singlet(name) => {
                let (r_x, r_y, r_z) = coord_conversion(x, y, z);
                writeln!(file, "{} {} {} {}", name, r_x, r_y, r_z)
            }
        }
    }
}

// fn write_co_nc_complex(c_x: f32, c_y: f32, c_z: f32, file: &mut File) -> std::io::Result<()> {
//     writeln!(
//         file,
//         "Co {} {} {}",
//         angstrom_to_rel(c_x),
//         angstrom_to_rel(c_y),
//         angstrom_to_rel(c_z),
//     )?;

//     // x-direction
//     writeln!(
//         file,
//         "C {} {} {}",
//         angstrom_to_rel(c_x + C_CO),
//         angstrom_to_rel(c_y),
//         angstrom_to_rel(c_z)
//     )?;
//     writeln!(
//         file,
//         "N {} {} {}",
//         angstrom_to_rel(c_x + C_CO + C_N_BOND),
//         angstrom_to_rel(c_y),
//         angstrom_to_rel(c_z)
//     )?;
//     writeln!(
//         file,
//         "C {} {} {}",
//         angstrom_to_rel(c_x - C_CO),
//         angstrom_to_rel(c_y),
//         angstrom_to_rel(c_z)
//     )?;
//     writeln!(
//         file,
//         "N {} {} {}",
//         angstrom_to_rel(c_x - C_CO - C_N_BOND),
//         angstrom_to_rel(c_y),
//         angstrom_to_rel(c_z)
//     )?;

//     // y-direction
//     writeln!(
//         file,
//         "C {} {} {}",
//         angstrom_to_rel(c_x),
//         angstrom_to_rel(c_y + C_CO),
//         angstrom_to_rel(c_z)
//     )?;
//     writeln!(
//         file,
//         "N {} {} {}",
//         angstrom_to_rel(c_x),
//         angstrom_to_rel(c_y + C_CO + C_N_BOND),
//         angstrom_to_rel(c_z)
//     )?;
//     writeln!(
//         file,
//         "C {} {} {}",
//         angstrom_to_rel(c_x),
//         angstrom_to_rel(c_y - C_CO),
//         angstrom_to_rel(c_z)
//     )?;
//     writeln!(
//         file,
//         "N {} {} {}",
//         angstrom_to_rel(c_x),
//         angstrom_to_rel(c_y - C_CO - C_N_BOND),
//         angstrom_to_rel(c_z)
//     )?;

//     // z-direction
//     writeln!(
//         file,
//         "C {} {} {}",
//         angstrom_to_rel(c_x),
//         angstrom_to_rel(c_y),
//         angstrom_to_rel(c_z + C_CO),
//     )?;
//     writeln!(
//         file,
//         "N {} {} {}",
//         angstrom_to_rel(c_x),
//         angstrom_to_rel(c_y),
//         angstrom_to_rel(c_z + C_CO + C_N_BOND),
//     )?;
//     writeln!(
//         file,
//         "C {} {} {}",
//         angstrom_to_rel(c_x),
//         angstrom_to_rel(c_y),
//         angstrom_to_rel(c_z - C_CO),
//     )?;
//     writeln!(
//         file,
//         "N {} {} {}",
//         angstrom_to_rel(c_x),
//         angstrom_to_rel(c_y),
//         angstrom_to_rel(c_z - C_CO - C_N_BOND),
//     )?;

//     Ok(())
// }

// #[inline(always)]
// fn angstrom_to_rel(d: f32, cell_length: f32) -> f32 {
//     d / cell_length
// }
