
pub(super) struct LocationLists {
    pub left: Vec<i32>,
    pub right: Vec<i32>,
}

impl IntoIterator for LocationLists {
    type Item = (i32, i32);
    type IntoIter = std::iter::Zip<std::vec::IntoIter<i32>, std::vec::IntoIter<i32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.left.into_iter().zip(self.right)
    }
}

impl<'a> IntoIterator for &'a LocationLists {
    type Item = (&'a i32, &'a i32);
    type IntoIter = std::iter::Zip<std::slice::Iter<'a, i32>, std::slice::Iter<'a, i32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.left.iter().zip(self.right.iter())
    }
}

impl<'a> IntoIterator for &'a mut LocationLists {
    type Item = (&'a mut i32, &'a mut i32);
    type IntoIter = std::iter::Zip<std::slice::IterMut<'a, i32>, std::slice::IterMut<'a, i32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.left.iter_mut().zip(self.right.iter_mut())
    }
}
