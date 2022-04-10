#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
    array: Vec<T>,
    width: usize,
    height: usize,
}

//Should this be the greater impl ?
impl<T: Clone> Array2<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self {
        let array = vec![value; width * height];
        Array2 {
            width,
            height,
            array,
        }
    }

    ///Return height
    pub fn height(&self) -> usize{
        self.height
    }

    ///Return width
    pub fn width(&self) -> usize{
        self.width
    }

    
    pub fn get(&self, c: usize, r: usize) -> Option<&T> {
        self.get_index(c, r).map(|index| &self.array[index])
    }

    pub fn get_mut(&mut self, c: usize, r: usize) -> Option<&mut T> {
        self.get_index(c, r)
            .map(move |index| &mut self.array[index])
    }

    pub fn get_index(&self, c: usize, r: usize) -> Option<usize> {
        if c < self.width && r < self.height {
            Some(r * self.width + c)
        } else {
            None
        }
    }
    // }
    // impl<T> Array2<T>{
    ///Take in a vector, place elements in array2 by rows first (dimenions are passed in as well), outputs an array2
    pub fn from_row_major(vec: Vec<T>, dimensions: (u32, u32)) -> Array2<T> {
        Array2 {
            width: dimensions.0 as usize,
            height: dimensions.1 as usize,
            array: vec,
        }
    }
    ///Take in a vector, place elements in array2 by columns first (dimensions are passed in as well), outputs an array2
    pub fn from_column_major(vec: Vec<T>, dimensions: (u32, u32)) -> Array2<T> {
        Array2 {
            width: dimensions.0 as usize,
            height: dimensions.1 as usize,
            array: vec,
        }
    }
    ///Returns an element at a given point
    pub fn position(self, coordinates: (usize, usize)) -> T {
        return self.array[self.width * (coordinates.0 + coordinates.1)].clone();
    }

    ///Iterates through rows first
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        //row, column, value
        self.array
            .iter()
            .enumerate()
            .map(move |(i, v)| (i % self.width, i / self.width, v))
    }
    ///Iterates through columns first
    pub fn iter_column_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.width)
            .map(move |c| (c, self.array.iter().skip(c)))
            .flat_map(move |(c, col)| {
                col.step_by(self.width)
                    .enumerate()
                    .map(move |(r, val)| (c, r, val))
            })
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn create_and_acess() {
        use super::Array2;
        let a = Array2::from_row_major( vec![1, 2, 3, 4, 5, 6], (3, 2,));
        assert_eq!(*a.get(2, 0).unwrap(), 3)
    }
    #[test]
    fn modify_and_acess() {
        use super::Array2;
        let mut a = Array2::from_row_major( vec![1, 2, 3, 4, 5, 6], (3, 2),);
        let i = a.get_mut(2, 0).unwrap();
        *i = 99;
        assert_eq!(*a.get(2, 0).unwrap(), 99);
    }
    #[test]
    fn access_out_of_bounds() {
        use super::Array2;
        let a = Array2::from_row_major( vec![1, 2, 3, 4, 5, 6], (3, 2,));
        let v = a.get(3, 0);
        assert_eq!(v, None);
    }
}
