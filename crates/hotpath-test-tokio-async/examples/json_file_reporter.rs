use std::time::Duration;

#[cfg_attr(feature = "hotpath", hotpath::measure)]
fn sync_function(sleep: u64) {
    let vec1 = vec![
        1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    std::hint::black_box(&vec1);
    drop(vec1);
    let vec2 = vec![
        1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    std::hint::black_box(&vec2);
    std::thread::sleep(Duration::from_nanos(sleep));
}

#[cfg_attr(feature = "hotpath", hotpath::measure)]
async fn async_function(sleep: u64) {
    let vec1 = vec![1, 2, 3, 5, 6, 7, 8, 9, 10];
    std::hint::black_box(&vec1);
    drop(vec1);
    let vec = vec![1, 2, 3, 5, 6, 7, 8, 9, 10];
    std::hint::black_box(&vec);
    tokio::time::sleep(Duration::from_nanos(sleep)).await;
}

use hotpath::Reporter;

struct JsonFileReporter;

impl Reporter for JsonFileReporter {
    fn report(&self, metrics_provider: &dyn hotpath::MetricsProvider<'_>) {
        if metrics_provider.metric_data().is_empty() {
            println!("No metrics to report");
            return;
        }

        let json = hotpath::MetricsJson::from(metrics_provider);

        match json.save_to_file("hotpath_report.json") {
            Ok(()) => println!("Report saved to hotpath_report.json"),
            Err(e) => eprintln!("Failed to save report: {}", e),
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _hotpath = hotpath::HotPathBuilder::new("main")
        .percentiles(&[50, 90, 95])
        .reporter(Box::new(JsonFileReporter))
        .build();

    for i in 0..100 {
        sync_function(i);
        async_function(i * 2).await;

        #[cfg(feature = "hotpath")]
        hotpath::measure_block!("custom_block", {
            if i == 0 {
                println!("custom_block output");
            }
            std::thread::sleep(Duration::from_nanos(i * 3))
        });
    }

    Ok(())
}
