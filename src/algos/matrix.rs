use std::ops::{Index, IndexMut};

pub struct Matrix<T> {
    width: usize,
    height: usize,
    data: Box<[T]>,
}

impl<T: Copy> Matrix<T> {
    #[inline]
    pub fn fill(value: T, width: usize, height: usize) -> Self {
        assert_ne!(width, 0);
        assert_ne!(height, 0);

        Self {
            width,
            height,
            data: vec![value; width * height].into_boxed_slice(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[x + y * self.width])
        } else {
            None
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(
            index.0 < self.width,
            "x index {} out of bounds {}",
            index.0,
            self.width
        );
        assert!(
            index.1 < self.height,
            "x index {} out of bounds {}",
            index.1,
            self.height
        );

        &self.data[index.0 + index.1 * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(
            index.0 < self.width,
            "x index {} out of bounds {}",
            index.0,
            self.width
        );
        assert!(
            index.1 < self.height,
            "x index {} out of bounds {}",
            index.1,
            self.height
        );

        &mut self.data[index.0 + index.1 * self.width]
    }
}
