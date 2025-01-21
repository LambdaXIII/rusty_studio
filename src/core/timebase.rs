#![allow(dead_code)]

/**
Timebase 时一个简单的结构体，保存了帧速率和是否丢帧的时基信息。

多媒体工作流程中常用的时基信息有很多的表示方法，例如：“24p”、“24.000p”、“23.976p”等等；
它们的含义具有微妙的区别，而且无法统一表示方式，所以在 Timebase 中，这些信息被拆分成了两部分。

其中帧速率使用 u8 类型表示，也就是说，在本工具集中使用 Timebase 只能处理较小的整数帧速率。
即使是常见的“23.976”或“59.94”这样的帧速率，实际处理时也是按照整数近似值处理的，
只是在输出时会按照固定的规范丢弃画面。所以，在 Timebase 中使用整数保存帧速率信息。

鉴于此工具集的定位就是简单、快速、易用，所以诸如小于1的帧速率或超高帧速率之类的情况暂时不提供支持。

Timebase is a simple struct that stores the frame rate and drop frame information.
The frame rate information used in multimedia workflows is represented in many ways,
such as: "24p", "24.000p", "23.976p" and so on.
The meaning of these representations is subtle and cannot be unified.
Therefore, in the Timebase, these representations are divided into two parts.

The frame rate is represented by u8 type, which means that in this tool set,
only small integer frame rates are processed.
Even if the common frame rates such as "23.976" or "59.94" are used,
they are processed as integer approximations and are only discarded in the output.

Since this tool set is designed to be simple, fast and easy to use,
it does not provide support for frame rates less than 1 or high frame rates.
*/
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Timebase {
    pub fps: u8,
    pub drop_frame: bool,
}

impl Timebase {
    ///直接指定帧速率以构造一个新的 Timebase。
    pub fn new(fps: u8) -> Self {
        Timebase {
            fps,
            drop_frame: false,
        }
    }

    /**
    从一个浮点数自动识别时基信息。

    原理非常简单，如果输入的数字四舍五入之后仍然相同，那么就认为它不丢帧，否则认为它丢帧。
    而帧速率则直接使用四舍五入的近似值。
    
    Automatically identify timebase information from a floating point number.

    The principle is very simple. If the rounded number after rounding is still the same,
    then it is not dropped, otherwise it is dropped.
    
    Example:
    ```rust
    # use rusty_studio::core::Timebase;
    let timebase = Timebase::from_real_fps(23.976);
    assert_eq!(timebase.fps,24);
    assert_eq!(timebase.drop_frame,true);
    ```

    ```rust
    # use rusty_studio::core::Timebase;
    let timebase = Timebase::from_real_fps(24.0);
    assert_eq!(timebase.fps,24);
    assert_eq!(timebase.drop_frame,false);
    ```
    */
    pub fn from_real_fps(fps: f64) -> Self {
        let base_fps = (fps * 100.0) as i32;
        let rounded = (fps.round() as i32) * 100;
        let drop_frame = base_fps != rounded;
        Self {
            fps: (rounded / 100) as u8,
            drop_frame,
        }
    }

    /**
    根据 fps 统计帧数占用的毫秒数。
    Calculate the number of milliseconds of a mount of frames, depending on fps.

    Example:
    ```rust
    # use rusty_studio::core::Timebase;
    let timebase = Timebase::new(24);
    let frames = 100;
    let ms = timebase.milliseconds_from_frames(frames);
    assert_eq!(ms,4167);
    ```
    */
    pub fn milliseconds_from_frames(&self, frames: u64) -> i128 {
        ((frames as f64 / self.fps as f64) * 1000.0).round() as i128
    }

    /**
    Calculate frames from milliseconds.
    Example:
    ```rust
    # use rusty_studio::core::Timebase;
    let timebase = Timebase::new(24);
    let ms = 4166;
    let frames = timebase.frames_from_milliseconds(ms);
    assert_eq!(frames,100);
    ```
    */
    pub fn frames_from_milliseconds(&self, ms: i128) -> u64 {
        let seconds = ms as f64 / 1000.0;
        (seconds * self.fps as f64).round() as u64
    }
}

impl Default for Timebase {
    fn default() -> Self {
        Self {
            fps: 24,
            drop_frame: false,
        }
    }
}
