use lazy_static::lazy_static;
use std::sync::Arc;

const STATIC_FRAME_STR: &str = include_str!("../frames/1/01.txt");

const ANIMATE1_FRAMES_STR: [&str; 11] = [
    include_str!("../frames/1/01.txt"),
    include_str!("../frames/1/02.txt"),
    include_str!("../frames/1/03.txt"),
    include_str!("../frames/1/04.txt"),
    include_str!("../frames/1/05.txt"),
    include_str!("../frames/1/06.txt"),
    include_str!("../frames/1/07.txt"),
    include_str!("../frames/1/08.txt"),
    include_str!("../frames/1/09.txt"),
    include_str!("../frames/1/10.txt"),
    include_str!("../frames/1/11.txt"),
];

const ANIMATE2_FRAMES_STR: [&str; 10] = [
    include_str!("../frames/2/01.txt"),
    include_str!("../frames/2/02.txt"),
    include_str!("../frames/2/03.txt"),
    include_str!("../frames/2/04.txt"),
    include_str!("../frames/2/05.txt"),
    include_str!("../frames/2/06.txt"),
    include_str!("../frames/2/07.txt"),
    include_str!("../frames/2/08.txt"),
    include_str!("../frames/2/09.txt"),
    include_str!("../frames/2/10.txt"),
];

const ANIMATE3_FRAMES_STR: [&str; 5] = [
    include_str!("../frames/3/01.txt"),
    include_str!("../frames/3/02.txt"),
    include_str!("../frames/3/03.txt"),
    include_str!("../frames/3/04.txt"),
    include_str!("../frames/3/05.txt"),
];

#[derive(Debug, Clone)]
pub struct Frame {
    pub lines: Arc<[&'static str]>,
}

#[derive(Debug, Clone)]
pub struct AnimatedFrames {
    pub frames: Arc<[Frame]>,
    pub interval_ms: Arc<[u64]>,
}

impl AnimatedFrames {
    pub fn iter(&self) -> AnimatedFramesIterator {
        AnimatedFramesIterator {
            frames: self.frames.clone(),
            interval_ms: self.interval_ms.clone(),
            current_frame: 0,
        }
    }
}

pub struct AnimatedFramesIterator {
    frames: Arc<[Frame]>,
    interval_ms: Arc<[u64]>,
    current_frame: usize,
}

impl Iterator for AnimatedFramesIterator {
    type Item = (Frame, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.frames.is_empty() || self.interval_ms.is_empty() {
            return None;
        }
        let max_index = self.frames.len().max(self.interval_ms.len()) - 1;
        if self.current_frame >= max_index {
            return None;
        }
        let frame = self.frames[self.current_frame].clone();
        let interval = self.interval_ms[self.current_frame];
        self.current_frame += 1;
        Some((frame, interval))
    }
}

fn make_frames(strs: &'static [&'static str], interval_ms: u64) -> AnimatedFrames {
    let frames = strs
        .iter()
        .map(|s| Frame {
            lines: s.lines().map(|line| &line[0..line.len() - 1]).collect(),
        })
        .collect::<Box<[Frame]>>();
    AnimatedFrames {
        frames: frames.into(),
        interval_ms: vec![interval_ms; strs.len()].into(),
    }
}

lazy_static! {
    pub static ref STATIC_FRAME: Frame = Frame {
        lines: STATIC_FRAME_STR
            .lines()
            .map(|line| &line[0..line.len() - 1])
            .collect(),
    };
    pub static ref ANIMATE1_FRAMES: AnimatedFrames = make_frames(&ANIMATE1_FRAMES_STR, 60);
    pub static ref ANIMATE2_FRAMES: AnimatedFrames = make_frames(&ANIMATE2_FRAMES_STR, 100);
    pub static ref ANIMATE3_FRAMES: AnimatedFrames = make_frames(&ANIMATE3_FRAMES_STR, 40);
}
