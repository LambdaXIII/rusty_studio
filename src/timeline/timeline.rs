use super::{TimeRange,TimelineItem,Track,TrackManager};

pub struct Timeline{
    tracks:Box<dyn TrackManager<TimelineItem>>,
    
}