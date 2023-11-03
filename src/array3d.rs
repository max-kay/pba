use std::ops::{Index, IndexMut};
/// A 3D grid type that is Copy and allows indexes to "wrap around"
#[derive(Clone, Debug)]
pub struct Array3d<T, const W: usize, const H: usize, const D: usize> {
    pub grid: Box<[[[T; W]; H]; D]>,
}

impl<T, const W: usize, const H: usize, const D: usize> Array3d<T, W, H, D>
where
    T: Copy + Default + Clone,
{
    pub fn new() -> Self {
        Self {
            grid: Box::new([[[T::default(); W]; H]; D]),
        }
    }

    pub fn filled(mut func: impl FnMut((usize, usize, usize)) -> T) -> Self {
        let mut grid = Box::new([[[T::default(); W]; H]; D]);
        for i in 0..D {
            for j in 0..H {
                for k in 0..W {
                    grid[k][j][i] = func((i, j, k))
                }
            }
        }
        Self { grid }
    }
}

impl<T, const W: usize, const H: usize, const D: usize> Index<(isize, isize, isize)>
    for Array3d<T, W, H, D>
{
    type Output = T;

    fn index(&self, index: (isize, isize, isize)) -> &Self::Output {
        let (x, y, z) = index;
        let x = x.rem_euclid(W as isize);
        let y = y.rem_euclid(H as isize);
        let z = z.rem_euclid(D as isize);
        // Safety this is safe because of the above rem
        unsafe {
            (*self.grid)
                .get_unchecked(z as usize)
                .get_unchecked(y as usize)
                .get_unchecked(x as usize)
        }
    }
}

impl<T, const W: usize, const H: usize, const D: usize> IndexMut<(isize, isize, isize)>
    for Array3d<T, W, H, D>
{
    fn index_mut(&mut self, index: (isize, isize, isize)) -> &mut Self::Output {
        let (x, y, z) = index;
        let x = x.rem_euclid(W as isize);
        let y = y.rem_euclid(H as isize);
        let z = z.rem_euclid(D as isize);
        // Safety this is safe because of the above rem
        unsafe {
            (*self.grid)
                .get_unchecked_mut(z as usize)
                .get_unchecked_mut(y as usize)
                .get_unchecked_mut(x as usize)
        }
    }
}

impl<T: Default + Copy, const W: usize, const H: usize, const D: usize> Default
    for Array3d<T, W, H, D>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const W: usize, const H: usize, const D: usize> Array3d<T, W, H, D> {
    pub fn as_flat_slice(&self) -> &[T] {
        // Safety: the memory layout of [[T; N]; N] is the same as [T]
        // TODO zero sized types
        unsafe { std::slice::from_raw_parts(self.grid.as_ptr().cast(), W * H * D) }
    }

    // pub fn as_flat_slice_mut(&mut self) -> &mut [T] {
    //     // Safety: the memory layout of [[T; N]; N] is the same as [T]
    //     // TODO zero sized types
    //     unsafe { std::slice::from_raw_parts_mut(self.grid.as_mut_ptr().cast(), W * H * D) }
    // }
}
