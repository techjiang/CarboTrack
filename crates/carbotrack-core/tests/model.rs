use carbotrack_core::{
    CarboTrack, CarboTrackConfig, CarbonIntensity, NetworkScope, Operation, ReportFormat,
};

#[test]
fn cpu_emission_matches_formula() {
    let emission = carbotrack_core::carbon::calculate(
        &Operation::Compute {
            operation: "api".into(),
            duration_ms: 60_000.0,
            cores: 2,
        },
        CarbonIntensity::USA,
        Default::default(),
    );
    assert!((emission.compute - 0.3032).abs() < 1e-6);
    assert_eq!(emission.total().round(), 0.0);
}

#[test]
fn network_emission_uses_cross_region_factor() {
    let emission = carbotrack_core::carbon::calculate(
        &Operation::Network {
            operation: "upload".into(),
            gb: 1.0,
            scope: NetworkScope::CrossRegion,
        },
        CarbonIntensity::CHINA,
        Default::default(),
    );
    assert!((emission.network - 3.336).abs() < 1e-6);
}

#[test]
fn tracker_records_and_reports() {
    let tracker = CarboTrack::new(CarboTrackConfig {
        region: "us-east-1".into(),
        budget_gco2e_per_day: Some(0.01),
        ..Default::default()
    });
    let record = tracker.cpu("api_request", 60_000.0, 1);
    assert_eq!(tracker.records().len(), 1);
    assert_eq!(record.id, tracker.records()[0].id);
    assert!(tracker.budget_exceeded());
    let report = tracker.report(ReportFormat::GhgProtocol, 0, u128::MAX);
    assert_eq!(report.record_count, 1);
}

#[test]
fn hotspot_analysis_reports_highest_operation_first() {
    use carbotrack_core::analysis::analyze;
    let tracker = CarboTrack::new(CarboTrackConfig::default());
    tracker.cpu("small", 1000.0, 1);
    tracker.network("large", 10.0, NetworkScope::CrossRegion);
    let hotspots = analyze(&tracker.records());
    assert_eq!(hotspots[0].operation, "large");
}

#[test]
fn cooling_ratio_uses_total_emissions() {
    let emission = carbotrack_core::carbon::calculate(
        &Operation::Compute {
            operation: "api".into(),
            duration_ms: 60_000.0,
            cores: 2,
        },
        CarbonIntensity::USA,
        Default::default(),
    );
    assert!((emission.cooling_emissions() - 0.0330).abs() < 0.001);
    assert!((emission.cooling_ratio() - 0.0909).abs() < 0.001);
}
