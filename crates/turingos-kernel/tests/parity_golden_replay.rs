#[path = "../../../tests/support/parity_golden.rs"]
mod parity_golden;

use parity_golden::{
    assert_phase20_provenance, fixture, initial_snapshot, GoldenAdapter, GoldenTask,
};
use turingos_core::StepIndex;
use turingos_kernel::{HaltStatus, KernelConfig, KernelEngine, RunReportStop, RunStop};

#[test]
fn deterministic_parity_golden_replay_matches_python_reference() {
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

    assert_phase20_provenance(&fixture);
    assert_eq!(outcome.attempted_steps, fixture.expected_steps);
    assert_eq!(outcome.committed_steps.len(), fixture.history.len());

    match &outcome.stop {
        RunStop::Halted {
            final_snapshot,
            status,
        } => {
            assert_eq!(status, &HaltStatus::Success);
            assert_eq!(final_snapshot.step(), expected_terminal_step);
            assert_eq!(final_snapshot.head().path(), "result.md");
            assert_eq!(
                final_snapshot.tape().read("result.md"),
                Some(fixture.expected_result.as_str())
            );
        }
        other => panic!("expected halted success, got {other:?}"),
    }

    assert_eq!(report.attempted_steps(), fixture.expected_steps);
    assert_eq!(report.committed_steps().len(), fixture.history.len());
    assert_eq!(
        report.stop(),
        RunReportStop::Halted {
            status: &HaltStatus::Success
        }
    );

    for (step, golden) in fixture.history.iter().enumerate() {
        assert_eq!(golden.step, step);

        let committed = &outcome.committed_steps[step];
        assert_eq!(
            committed.observation.provenance,
            format!("golden:{}", fixture.selected_agent)
        );
        assert_eq!(committed.record.from_step, StepIndex(step as u64));
        assert_eq!(committed.record.to_step, StepIndex(step as u64 + 1));
        assert_eq!(committed.record.next_head.path(), golden.next_path);
        assert_eq!(committed.record.notes, "");
    }
}
