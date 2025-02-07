#![allow(dead_code)]
use super::{StaticSubtitle, SubtitleLoader};
use crate::core::TimeRangeSupport;
use crate::timeline::TimeRange;
use regex::Regex;
use std::io::BufRead;

pub struct SrtReader<'a> {
    source: &'a mut dyn BufRead,
    sequence_number_pat: Regex,
    time_range_pat: Regex,
}

impl Iterator for SrtReader<'_> {
    type Item = StaticSubtitle;
    fn next(&mut self) -> Option<Self::Item> {
        // 使用枚举来表示解析状态
        enum ParseState {
            SequenceNumber,
            TimeRange,
            Content,
            Done,
        }

        let mut state = ParseState::SequenceNumber;
        let time_range: TimeRange = TimeRange::from_millisecond(0, 0);
        let mut contents: Vec<String> = Vec::new();
        let mut line: String = String::new();

        loop {
            match self.source.read_line(&mut line) {
                Ok(0) => return None, // 文件结束
                Ok(_) => match state {
                    ParseState::SequenceNumber => {
                        if self.sequence_number_pat.is_match(&line) {
                            state = ParseState::TimeRange;
                        }
                    }
                    ParseState::TimeRange => {
                        if self.time_range_pat.is_match(&line) {
                            state = ParseState::Content;
                        }
                    }
                    ParseState::Content => {
                        if line.is_empty() {
                            state = ParseState::Done;
                        } else {
                            contents.push(line.clone());
                        }
                    }
                    ParseState::Done => break,
                },
                Err(_) => return None, // 读取错误
            }
        }

        Some(StaticSubtitle {
            start: time_range.start(),
            duration: time_range.duration(),
            content: contents.join("\n"),
        })
    }
}

impl<'a> SubtitleLoader<'a> for SrtReader<'a> {
    fn new(source: &'a mut (dyn BufRead + 'a)) -> Self {
        Self {
            source,
            sequence_number_pat: Regex::new(r"^(\d+)$").unwrap(),
            time_range_pat: Regex::new(
                r"^(\d\d:\d\d:\d\d[:;.,]\d\d\d) --> (\d\d:\d\d:\d\d[:;.,]\d\d\d)",
            )
            .unwrap(),
        }
    }
}
