use std::time::{Duration, Instant};

use num::ToPrimitive;

pub struct Progress {
    max: f64,
    val: f64,

    width: f64,

    start: Instant,
    last_update: Instant,

    progress_chars: Vec<char>,

    speed: Option<f64>,
    progress: String,
}

impl Progress {
    pub fn new<T>(max: T) -> Self
    where
        T: ToPrimitive,
    {
        Progress {
            max: max.to_f64().unwrap(),
            val: 0_f64,
            width: 50_f64,
            start: Instant::now(),
            last_update: Instant::now() - Duration::from_secs(1),
            progress_chars: "▏▎▍▌▋▊▉█".chars().collect(),
            speed: None,
            progress: String::new(),
        }
    }

    pub fn inc<T>(&mut self, inc: T)
    where
        T: ToPrimitive,
    {
        self.val += inc.to_f64().unwrap(); //NumCast::from(inc).unwrap();
        self.draw();
    }

    pub fn val<T>(&mut self, val: T)
    where
        T: ToPrimitive,
    {
        self.val = val.to_f64().unwrap();
        self.draw();
    }

    pub fn finish(&mut self) {
        self.val = self.max;
        self.draw();
        println!("");
    }

    fn draw(&mut self) {
        // Generate the progress bar
        let progress_increment = (self.val * (self.width * self.progress_chars.len() as f64)
            / self.max)
            .round()
            .to_usize()
            .unwrap();
        let full_blocks = progress_increment / self.progress_chars.len();
        let partial_block_size = progress_increment % self.progress_chars.len();

        let mut vec = vec![*self.progress_chars.last().unwrap(); full_blocks];
        if partial_block_size > 0 {
            vec.push(*self.progress_chars.get(partial_block_size - 1).unwrap());
        }
        let new_progress = vec.iter().collect::<String>();

        // Skip the refresh if there is no visible progress and not enough time has passed
        let time = Instant::now();
        if (time < self.last_update + Duration::from_millis(100)) && (new_progress == self.progress)
        {
            return;
        }
        self.last_update = time;
        self.progress = new_progress;

        // Estimate the speed
        // Do it only if we have enough data to mean something
        // And use a running average to avoid too much variance early on
        let elapsed = (Instant::now() - self.start).as_secs_f64();

        if self.val > (self.max / 100000.0) {
            self.speed = match self.speed {
                Some(speed) => Some(speed * 0.9 + self.val as f64 / elapsed * 0.1),
                None => Some(self.val as f64 / elapsed),
            }
        }

        // Compute the ETA
        let eta = match self.speed {
            Some(speed) if self.val <= self.max => {
                let eta = f64::ceil((self.max - self.val) as f64 / speed) as usize;
                format!("{}:{:02}", eta / 60, eta % 60)
            }
            _ => "unk".to_owned(),
        };

        print!(
            // "background" char only works if we don't use "partial blocks"
            //"\r[{:>2}:{:02}] [{:░<50}] {:>3}% ({})",
            "\r[{:>2}:{:02}] [{:<50}] {:>5.1}% ({})",
            elapsed as usize / 60,
            elapsed as usize % 60,
            self.progress,
            self.val * 100.0 / self.max,
            eta
        );
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut p = Progress::new(50 * 8);
        for i in 0..(50 * 8) {
            p.val(i);
            std::thread::sleep(Duration::from_millis(25));
        }
        p.finish();
    }
}
