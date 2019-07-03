use std::ops::{Index, Div};
use std::iter::Sum;

pub struct FixedSizePipe<T> {
    data: Box<[Option<T>]>,
    index: usize,
}

impl<T> FixedSizePipe<T> {
    pub fn new(size: usize) -> Self {
        let vec: Vec<Option<T>> = (0..size).map(|_| None).collect();
        Self {
            data: vec.into_boxed_slice(),
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.data
            .iter()
            .filter(|x| x.is_some())
            .count()
    }

    pub fn push(&mut self, value: T) {
        self.data[self.index] = Some(value);
        self.index += 1;
        self.index %= self.data.len()
    }
}

impl FixedSizePipe<f32> {
    pub fn average(&self) -> f32 {
        let sum: f32 = self.data
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .sum();

        sum / self.len() as f32
    }
}

impl<T> Index<usize> for FixedSizePipe<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len());
        match &self.data[index] {
            None => panic!("index {} out of bounds for len {}", index, self.len()),
            Some(x) => x
        }
    }
}