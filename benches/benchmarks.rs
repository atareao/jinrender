use jinrender::*;
use std::collections::HashMap;
use std::time::Instant;

#[cfg(test)]
mod benchmarks {
    use super::*;

    #[test]
    fn bench_simple_template_rendering() {
        let mut env_vars = HashMap::new();
        env_vars.insert("NAME".to_string(), "Benchmark Test".to_string());
        env_vars.insert("VALUE".to_string(), "12345".to_string());
        
        let template_content = "Hello {{ env.NAME }}, your value is {{ env.VALUE }}";
        
        // Warmup
        for _ in 0..10 {
            let _ = render_template("bench_template", template_content, &env_vars).unwrap();
        }
        
        // Benchmark
        let start = Instant::now();
        let iterations = 1000;
        
        for _ in 0..iterations {
            let _ = render_template("bench_template", template_content, &env_vars).unwrap();
        }
        
        let duration = start.elapsed();
        let avg_time = duration / iterations;
        
        println!("Simple template rendering:");
        println!("  Total time: {:?}", duration);
        println!("  Average time per render: {:?}", avg_time);
        println!("  Renders per second: {:.0}", 1_000_000_000.0 / avg_time.as_nanos() as f64);
        
        // Basic performance assertion - should render simple templates quickly
        assert!(avg_time.as_millis() < 10, "Simple template rendering is too slow");
    }
}
