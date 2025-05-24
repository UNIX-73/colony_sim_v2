#[macro_export]
macro_rules! debug_perf {
    ($label:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let result = $code;
        let duration = start.elapsed();

        println!(
            "{} took {} ns ({} µs, {} ms)",
            $label,
            duration.as_nanos(),
            duration.as_micros(),
            duration.as_millis()
        );

        result
    }};
}
