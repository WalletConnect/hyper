use opentelemetry::metrics::Meter;
use std::sync::Mutex;

pub static GLOBAL_METER: Mutex<Option<&'static Meter>> = Mutex::new(None);

pub fn init_meter(meter: &'static Meter) {
    *GLOBAL_METER.lock().unwrap() = Some(meter);
}

#[macro_export]
macro_rules! counter {
    ($name:expr) => {{
        static METRIC: ::once_cell::sync::Lazy<Option<::opentelemetry::metrics::Counter<u64>>> =
            ::once_cell::sync::Lazy::new(|| {
                $crate::metrics::GLOBAL_METER
                    .lock()
                    .map(|lock| lock.as_deref().map(|meter| meter.u64_counter($name).init()))
                    .unwrap_or(None)
            });

        &METRIC
    }};

    ($name:expr, $value:expr) => {{
        $crate::counter!($name, $value, &[]);
    }};

    ($name:expr, $value:expr, $tags:expr) => {{
        if let Some(counter) = $crate::counter!($name).as_ref() {
            counter.add(&::opentelemetry::Context::new(), $value as u64, $tags);
        }
    }};
}
