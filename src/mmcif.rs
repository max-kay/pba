use nalgebra::{Matrix3, Vector3};
use std::{collections::HashMap, fs::File, io::Write, path::Path, usize};

use crate::array3d::Array3d;

/// A struct containing all information required to write a mmcif file
struct MmCifWriter<'a, const S: usize> {
    cell_a: f32,
    cell_b: f32,
    cell_c: f32,
    armstrong_to_rel: Matrix3<f32>,
    naming: HashMap<i8, Option<Ion>>,
    grid: &'a Array3d<i8, S, S, S>,
    file: File,
    counter: u32,
}

impl<'a, const S: usize> MmCifWriter<'a, S> {
    /// Constructor
    fn new(
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
            counter: 0,
        })
    }
}

impl<const S: usize> MmCifWriter<'_, S> {
    /// Make the Header for the mmcif file
    fn get_header(&self) -> String {
        format!(
            "\
data_struct
_entry.id struct

_cell.entry_id struct
_cell.length_a {}
_cell.length_b {}
_cell.length_c {}
_cell.angle_alpha 90
_cell.angle_beta 90
_cell.angle_gamma 90

_symmetry.entry_id struct
_symmetry.space_group_name_H-M 'P 1'
_symmetry.Int_Tables_number 1




loop_
_chem_comp.id
_chem_comp.type
'' .


",
            self.cell_a, self.cell_b, self.cell_c
        )
    }

    fn get_symbols(&self) -> Vec<String> {
        let mut vec = Vec::new();
        for ion in self.naming.values().into_iter().filter_map(|x| x.as_ref()) {
            vec.append(&mut ion.get_uppercase_names())
        }
        vec
    }

    /// Write the mmcif file
    fn write_to_file(&mut self) -> std::io::Result<()> {
        writeln!(self.file, "{}", self.get_header())?;

        writeln!(
            self.file,
            "\
loop_
_atom_type.symbol"
        )?;
        for name in self.get_symbols() {
            writeln!(self.file, "{}", name)?;
        }

        writeln!(
            self.file,
            "\
loop_
_atom_site.group_PDB
_atom_site.id
_atom_site.type_symbol
_atom_site.label_atom_id
_atom_site.label_alt_id
_atom_site.label_comp_id
_atom_site.label_asym_id
_atom_site.label_entity_id
_atom_site.label_seq_id
_atom_site.pdbx_PDB_ins_code
_atom_site.Cartn_x
_atom_site.Cartn_y
_atom_site.Cartn_z
_atom_site.occupancy
_atom_site.B_iso_or_equiv
_atom_site.pdbx_formal_charge
_atom_site.auth_seq_id
_atom_site.auth_asym_id
_atom_site.pdbx_PDB_model_num"
        )?;
        for i in 0..(S as isize) {
            for j in 0..(S as isize) {
                for k in 0..(S as isize) {
                    let val = self.grid[(i, j, k)];
                    match self.naming.get(&val) {
                        Some(opt) => {
                            if let Some(ion) = opt {
                                self.place_ion(*ion, self.index_to_armstong(i, j, k))?
                            }
                        }
                        None => eprintln!("failed to get name for {}", val),
                    }
                }
            }
        }
        Ok(())
    }

    /// Place an ion into a mmcif file
    fn place_ion(&mut self, ion: Ion, coord_armstrong: Vector3<f32>) -> std::io::Result<()> {
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

    /// Place a named atom into a mmcif file at the position in armstong
    fn place(&mut self, name: &'static str, pos_armstrong: Vector3<f32>) -> std::io::Result<()> {
        self.counter += 1;
        let rel_coords = self.armstrong_to_rel * pos_armstrong;
        writeln!(
            self.file,
            "ATOM {} {} {} . '' . . . ? {} {} {} 1 0 ? ? A 1",
            self.counter,
            name,
            name.to_ascii_uppercase(),
            rel_coords.x,
            rel_coords.y,
            rel_coords.z
        )
    }

    /// convert an index to coordinates in armstong
    fn index_to_armstong(&self, i: isize, j: isize, k: isize) -> Vector3<f32> {
        [
            i as f32 / S as f32 * self.cell_a, // TODO check this logic!!!
            j as f32 / S as f32 * self.cell_b,
            k as f32 / S as f32 * self.cell_c,
        ]
        .into()
    }
}

/// A type for Ions
#[derive(Clone, Copy)]
pub enum Ion {
    Singlet(&'static str),
    Cyanometalate {
        name: &'static str,
        c_offset: f32,
        n_offset: f32,
    },
}

impl Ion {
    fn get_uppercase_names(&self) -> Vec<String> {
        match self {
            Ion::Singlet(name) => vec![name.to_ascii_uppercase()],
            Ion::Cyanometalate { name, .. } => {
                vec![name.to_ascii_uppercase(), "C".to_string(), "N".to_string()]
            }
        }
    }
}

/// Create a mmcif file from the grid.
/// Note that $\alpha = \beta = \gamma = 90 \degrees$
/// The naming provides a translation from i8 to an ion
/// If the ion is None it is just ignored.
pub fn write_mmcif<const S: usize>(
    grid: &Array3d<i8, S, S, S>,
    cell_a: f32,
    cell_b: f32,
    cell_c: f32,
    naming: HashMap<i8, Option<Ion>>,
    path: impl AsRef<Path>,
) -> std::io::Result<()> {
    let mut writer = MmCifWriter::new(grid, cell_a, cell_b, cell_c, naming, path)?;
    writer.write_to_file()
}
