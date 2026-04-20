use metrics::{counter, describe_counter};

/// Register default telemetry metrics shared across all services.
pub fn register_default_metrics() {
    describe_counter!("api_calls_total", "Total number of API calls handled");
}

/// Record an API call for the given endpoint.
pub fn record_api_call(endpoint: &str) {
    counter!("api_calls_total", "endpoint" => endpoint.to_owned()).increment(1);
}

/// Install the Prometheus exporter on the given port.
/// Call once during service startup after `appbase::init()`.
pub fn install_prometheus_exporter(port: u16) {
    metrics_exporter_prometheus::PrometheusBuilder::new()
        .with_http_listener(([0, 0, 0, 0], port))
        .install()
        .expect("failed to install prometheus exporter");
}
