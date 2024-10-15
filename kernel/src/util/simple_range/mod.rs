// region StepByOne begin
pub trait StepByOne {
    fn step(&mut self);
}
// region StepByOne end

// region SimpleRange begin
#[derive(Clone, Copy)]
pub struct SimpleRange<T>
where T: StepByOne + Copy + PartialEq + PartialOrd
{
    start: T,
    end: T,
}

impl<T> IntoIterator for SimpleRange<T>
where T: StepByOne + Copy + PartialEq + PartialOrd
{
    type Item = T;
    type IntoIter = SimpleRangeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        SimpleRangeIterator { current: self.start, end: self.end }
    }
}

impl<T> SimpleRange<T>
where T: StepByOne + Copy + PartialEq + PartialOrd
{
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end, "SimpleRange: start must be less than or equal to end");
        SimpleRange { start, end }
    }

    pub fn get_start(&self) -> T { self.start }

    pub fn get_end(&self) -> T { self.end }
}
// region SimpleRange end

// region SimpleRangeIterator begin
pub struct SimpleRangeIterator<T>
where T: StepByOne + Copy + PartialEq + PartialOrd
{
    current: T,
    end: T,
}

impl<T> Iterator for SimpleRangeIterator<T>
where T: StepByOne + Copy + PartialEq + PartialOrd
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let result = self.current;
            self.current.step();
            Some(result)
        }
    }
}
// endregion SimpleRangeIterator end