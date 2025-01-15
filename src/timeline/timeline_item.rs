pub trait TimelineItem<C>{
    fn get_content(&self) -> C;
    fn set_content(&mut self, content:C);
}