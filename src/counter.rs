use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    sync::atomic::{AtomicUsize, Ordering},
    time::UNIX_EPOCH,
};

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};

pub struct Counter {
    inner: AtomicUsize,
}

impl Counter {
    const FILE_NAME: &'static str = "counter";

    pub fn new() -> Self {
        Self::ensure_file_exists();
        Self {
            inner: Self::read_counter(),
        }
    }

    pub fn get(&self) -> usize {
        self.inner.load(Ordering::SeqCst)
    }

    pub fn update(&self) {
        let from = Self::last_modified();
        let to = Utc::now().naive_local().date();
        let amount = month_diff(from, to) as usize;
        self.increase(amount);
    }

    pub fn increase(&self, amount: usize) {
        let new_value = self
            .inner
            .fetch_add(amount, Ordering::SeqCst)
            .checked_add(amount)
            .expect("amount plus current count value less or equal to usize max");
        Self::write_counter(new_value);
    }

    pub fn decrease(&self, amount: usize) {
        let new_value = self
            .inner
            .fetch_sub(amount, Ordering::SeqCst)
            .checked_sub(amount)
            .expect("amount less or equal to the current count value");
        Self::write_counter(new_value);
    }

    fn last_modified() -> NaiveDate {
        let path = Path::new(Self::FILE_NAME);
        let last_modfied = path.metadata().unwrap().modified().unwrap();
        let millis = last_modfied.duration_since(UNIX_EPOCH).unwrap().as_millis();
        NaiveDateTime::from_timestamp_millis(i64::try_from(millis).unwrap())
            .unwrap()
            .date()
    }

    fn ensure_file_exists() {
        let path = Path::new(Self::FILE_NAME);
        if !path.exists() {
            let mut file = File::create(path).expect("counter file");
            file.write_all(b"0").expect("write counter");
            file.flush().expect("write file to disk");
        }
    }

    fn read_counter() -> AtomicUsize {
        let path = Path::new(Self::FILE_NAME);
        let mut file = File::open(path).expect("counter file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("file contents");
        AtomicUsize::new(contents.parse::<usize>().expect("valid counter"))
    }

    fn write_counter(value: usize) {
        let path = Path::new(Self::FILE_NAME);
        let mut file = File::create(path).expect("counter file");
        file.write_all(value.to_string().as_bytes())
            .expect("write counter");
        file.flush().expect("write file to disk");
    }
}

fn month_diff(from: NaiveDate, to: NaiveDate) -> u32 {
    debug_assert!(from <= to);
    let from_month = from.month();
    let to_month = to.month();
    if from_month == 12u32 {
        to_month
    } else {
        to_month - from_month
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        path::Path,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use chrono::{Months, NaiveDate, NaiveDateTime, NaiveTime};

    use super::{month_diff, Counter};

    #[allow(clippy::cast_sign_loss)]
    #[test]
    fn test_counter() {
        let now = SystemTime::now();
        let now = now.duration_since(UNIX_EPOCH).unwrap().as_millis();
        let now = NaiveDateTime::from_timestamp_millis(i64::try_from(now).unwrap())
            .unwrap()
            .date();
        let month_before = now.checked_sub_months(Months::new(1)).expect("valid date");
        let month_before = month_before.and_time(NaiveTime::MIN).and_utc().timestamp();
        let month_before = UNIX_EPOCH
            .checked_add(Duration::from_secs(month_before as u64))
            .expect("valid date");
        let path = Path::new(Counter::FILE_NAME);
        if path.exists() {
            std::fs::remove_file(path).expect("delete file");
        }
        let counter = Counter::new();
        let file = File::create(path).expect("counter file");
        file.set_modified(month_before)
            .expect("set mdofication date");

        assert_eq!(counter.get(), 0);
        counter.update();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_month_diff() {
        let from = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2023, 2, 23).unwrap();
        assert_eq!(month_diff(from, to), 1);

        let from = NaiveDate::from_ymd_opt(2023, 2, 3).unwrap();
        let to = NaiveDate::from_ymd_opt(2023, 6, 19).unwrap();
        assert_eq!(month_diff(from, to), 4);

        let from = NaiveDate::from_ymd_opt(2023, 12, 13).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 1, 6).unwrap();
        assert_eq!(month_diff(from, to), 1);

        let from = NaiveDate::from_ymd_opt(2023, 12, 30).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 2, 11).unwrap();
        assert_eq!(month_diff(from, to), 2);
    }
}
