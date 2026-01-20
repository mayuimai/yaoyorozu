// src/appdoc/orichy_timer.rs

//--------------------------------------//
// オリちゃんがお知らせするタイマー機能
//--------------------------------------//

use std::time::{Duration, Instant};

pub struct OriTimer {
    開始時間: Instant,
    休憩が必要: bool,
}

impl OriTimer {
    pub fn new() -> Self {
        Self {
            開始時間: Instant::now(),
            休憩が必要: false,
        }
    }

    pub fn 更新(&mut self) {
        // 1時間（3600秒）経ったかチェック
        if self.開始時間.elapsed() >= Duration::from_secs(10) {
            self.休憩が必要 = true;
        }
    }

    pub fn 休憩した(&mut self) {
        self.開始時間 = Instant::now();
        self.休憩が必要 = false;
    }

    pub fn 状態を教える(&self) -> bool {
        self.休憩が必要
    }
}