use std::ops::Index;

pub struct FixedSizePipe<T> {
    data: Box<[Option<T>]>,
    index: usize,
}

impl<T> FixedSizePipe<T> {
    pub fn new(len: usize) -> Self {
        let vec: Vec<Option<T>> = (0..len).map(|_| None).collect();
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

    pub fn push_right(&mut self, value: T) {
        self.data[self.index] = Some(value);
        self.index = (self.index + 1) % self.data.len();
    }

    pub fn push_left(&mut self, value: T) {
        self.data[self.index] = Some(value);
        let len = self.data.len();
        self.index = (self.index + len - 1) % len;
    }

    pub fn as_vec(&self) -> Vec<&T> {
        let mut vec = self.data.as_ref().iter().collect::<Vec<_>>();
        vec.rotate_left(self.index);
        vec.iter().filter_map(|o| o.as_ref()).collect::<Vec<_>>()
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