use std::time::Duration;

use hotpath::Reporter;

/// Run with:
/// cargo test -p hotpath-test-tokio-async --example unit_test --features hotpath -- --nocapture --test-threads=1

#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn sync_function(sleep: u64) {
    let vec1 = vec![1, 2, 3, 5, 6];
    std::hint::black_box(&vec1);
    drop(vec1);
    let vec2 = vec![1, 2, 3, 5, 6];
    std::hint::black_box(&vec2);
    std::thread::sleep(Duration::from_nanos(sleep));
}

#[allow(unused)]
struct UnitTestReporter;

impl Reporter for UnitTestReporter {
    fn report(
        &self,
        metrics_provider: &dyn hotpath::MetricsProvider<'_>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if metrics_provider.metric_data().is_empty() {
            println!("No metrics to report");
            return Ok(());
        }

        let metric_data = metrics_provider.metric_data();

        let sync_function_metrics = metric_data.get("unit_test::sync_function").unwrap();
        dbg!(sync_function_metrics);

        let alloc_count = &sync_function_metrics[1];
        if let hotpath::MetricType::AllocCount(count) = alloc_count {
            assert!(*count < 3, "AllocCount is not less than 3: {}", count);
        } else {
            panic!("Expected AllocCount metric, got {:?}", alloc_count);
        }
        Ok(())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..100 {
        sync_function(i);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_function() {
        #[cfg(feature = "hotpath")]
        let _hotpath = hotpath::GuardBuilder::new("test_sync_function")
            .reporter(Box::new(UnitTestReporter))
            .build();

        sync_function(100);
    }
}
