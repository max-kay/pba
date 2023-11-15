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

    pub fn as_flat_slice_mut(&mut self) -> &mut [T] {
        // Safety: the memory layout of [[T; N]; N] is the same as [T]
        // TODO zero sized types
        unsafe { std::slice::from_raw_parts_mut(self.grid.as_mut_ptr().cast(), W * H * D) }
    }
}

impl<const W: usize, const H: usize, const D: usize> Array3d<i8, W, H, D> {
    pub fn as_string(&self) -> String {
        let mut out = String::new();
        for val in self.as_flat_slice() {
            out.push_str(&format!("{} ", val))
        }
        out.pop();
        out
    }

    pub fn from_string(string: &str) -> Result<Self, std::num::ParseIntError> {
        let split = string.split_whitespace();
        let mut grid = Self::new();
        let mut counter = 0;
        for (v, s) in grid.as_flat_slice_mut().iter_mut().zip(split.into_iter()) {
            *v = s.parse()?;
            counter += 1;
        }
        assert_eq!(W * H * D, counter);
        Ok(grid)
    }
}
