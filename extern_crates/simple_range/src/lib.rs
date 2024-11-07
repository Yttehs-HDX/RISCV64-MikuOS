#![no_std]
#![no_main]

pub trait StepByOne {
    fn step(&mut self);
}

// region SimpleRange begin
#[derive(Clone, Copy)]
pub struct SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd,
{
    start: T,
    end: T,
}

impl<T> IntoIterator for SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd,
{
    type Item = T;
    type IntoIter = SimpleRangeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleRangeIterator {
            range: self,
            current: self.start,
        }
    }
}

impl<T> SimpleRange<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd,
{
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> T {
        self.start
    }

    pub fn get_start_mut(&mut self) -> &mut T {
        &mut self.start
    }

    pub fn end(&self) -> T {
        self.end
    }

    pub fn get_end_mut(&mut self) -> &mut T {
        &mut self.end
    }
}
// region SimpleRange end

// region SimpleRangeIterator begin
pub struct SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd,
{
    range: SimpleRange<T>,
    current: T,
}

impl<T> Iterator for SimpleRangeIterator<T>
where
    T: StepByOne + Copy + PartialEq + PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.range.end() {
            return None;
        }

        let current = self.current;
        self.current.step();
        Some(current)
    }
}
// region SimpleRangeIterator end
