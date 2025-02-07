use super::StaticSubtitle;
use std::io::BufRead;



pub trait SubtitleLoader<'a> 
where Self:Iterator<Item=StaticSubtitle>{
    fn new(source:&'a mut (dyn BufRead + 'a)) -> Self;
}
