use super::Time;


pub trait TimeRange{
    fn start(&self) -> Time;
    fn duration(&self) -> Time;
    fn end(&self) -> Time{
        self.start() + self.duration()
    }
    
    fn contains(&self, time: &Time) -> bool{
        self.start() <= *time && *time <= self.end()
    }
    
    fn overlaps(&self, other: &dyn TimeRange) -> bool{
        // self.contains(&other.start()) || self.contains(&other.end()) || other.contains(&self.start()) || other.contains(&self.end())
    self.start() <= other.end() && self.end() >= other.start()
    }
    
    
}