#[path = "../../../tests/support/parity_golden.rs"]
mod parity_golden;

use parity_golden::{fixture, initial_snapshot, GoldenAdapter, GoldenTask};
use turingos_core::StepIndex;
use turingos_kernel::{HaltStatus, KernelConfig, KernelEngine, RunReportStop};
use turingos_observe::{
    RunExport, RunExportStop, RunPulseTerminalClass, RunPulseTransitionClass, RunSummaryStopClass,
};

#[test]
fn deterministic_parity_golden_projects_through_observer_surfaces() {
    let fixture = fixture();
    let task = GoldenTask { fixture: &fixture };
    let adapter = GoldenAdapter::new(&fixture);
    let engine = KernelEngine::new(KernelConfig::default());

    let outcome = engine.run(
        initial_snapshot(&fixture),
        &adapter,
        &task,
        fixture.expected_steps,
    );
    let report = outcome.report();
    let expected_terminal_step = StepIndex(fixture.expected_steps as u64);

    assert_eq!(outcome.attempted_steps, fixture.expected_steps);
    assert_eq!(outcome.committed_steps.len(), fixture.history.len());

    assert_eq!(report.attempted_steps(), fixture.expected_steps);
    assert_eq!(report.committed_steps().len(), fixture.history.len());
    assert_eq!(report.terminal_snapshot().step(), expected_terminal_step);
    assert_eq!(
        report.terminal_snapshot().tape().read("result.md"),
        Some(fixture.expected_result.as_str())
    );
    assert_eq!(
        report.stop(),
        RunReportStop::Halted {
            status: &HaltStatus::Success
        }
    );

    let export = RunExport::from(outcome.report());
    let summary = export.summary();
    let frame = export.pulse_frame();

    assert_eq!(export.attempted_steps, fixture.expected_steps);
    assert_eq!(export.committed_steps.len(), fixture.history.len());
    assert_eq!(
        export.committed_steps[0].observation.provenance,
        format!("golden:{}", fixture.selected_agent)
    );
    assert_eq!(
        export.terminal_snapshot.tape().read("result.md"),
        Some(fixture.expected_result.as_str())
    );
    assert_eq!(
        export.stop,
        RunExportStop::Halted {
            status: HaltStatus::Success
        }
    );

    assert_eq!(summary.attempted_steps(), fixture.expected_steps);
    assert_eq!(summary.committed_step_count(), fixture.history.len());
    assert_eq!(summary.terminal_step(), expected_terminal_step);
    assert_eq!(summary.stop_class(), RunSummaryStopClass::Halted);
    assert_eq!(summary.halt_status(), Some(&HaltStatus::Success));
    assert!(summary.is_success());
    assert_eq!(
        summary.terminal_snapshot().tape().read("result.md"),
        Some(fixture.expected_result.as_str())
    );

    assert_eq!(frame.basis(), &export.basis());
    assert_eq!(
        frame.class().transition(),
        RunPulseTransitionClass::CurrentOnly
    );
    assert_eq!(
        frame.class().terminal(),
        RunPulseTerminalClass::HaltedSuccess
    );
}
