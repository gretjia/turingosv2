use turingos_adapter::{AdapterOutcome, IntentAdapter};
use turingos_core::{
    Head, IntentEnvelope, PredicateVerdict, ReadView, StepIndex, StepObservation, TapeState,
    TraceHash, UniverseSnapshot, WorldState, WriteMode,
};
use turingos_kernel::{HaltStatus, KernelConfig, KernelEngine, RunStop, TaskContract};
use turingos_observe::{
    RunExport, RunPulseTerminalClass, RunPulseTransitionClass, RunSummaryStopClass,
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum FixturePhase {
    Boot,
    Finalize,
    Done,
}

struct FixtureTask;
struct SuccessAdapter;
struct RejectingAdapter;

impl TaskContract<FixturePhase> for FixtureTask {
    fn evaluate(
        &self,
        world: &WorldState<FixturePhase>,
        intent: &IntentEnvelope<FixturePhase>,
    ) -> PredicateVerdict {
        let expected = match world.register() {
            FixturePhase::Boot => IntentEnvelope {
                proposed_register: FixturePhase::Finalize,
                action_payload: None,
                proposed_head: Head::new("root/result.txt"),
                write_mode: WriteMode::Overwrite,
                write_content: Some("prepared".to_owned()),
                halt: false,
                notes: "seed->prepared".to_owned(),
            },
            FixturePhase::Finalize => IntentEnvelope {
                proposed_register: FixturePhase::Done,
                action_payload: None,
                proposed_head: Head::new("root/result.txt"),
                write_mode: WriteMode::Overwrite,
                write_content: Some("ok".to_owned()),
                halt: true,
                notes: "write-result".to_owned(),
            },
            FixturePhase::Done => IntentEnvelope {
                proposed_register: FixturePhase::Done,
                action_payload: None,
                proposed_head: Head::new("root/result.txt"),
                write_mode: WriteMode::Keep,
                write_content: None,
                halt: true,
                notes: "already-done".to_owned(),
            },
        };

        if intent == &expected {
            PredicateVerdict::pass()
        } else {
            PredicateVerdict::fail(vec!["intent mismatch".to_owned()])
        }
    }

    fn is_success(&self, world: &WorldState<FixturePhase>) -> bool {
        world.register() == &FixturePhase::Done && world.tape().read("root/result.txt") == Some("ok")
    }
}

impl IntentAdapter<FixturePhase> for SuccessAdapter {
    fn propose(&self, view: &ReadView<FixturePhase>) -> AdapterOutcome<FixturePhase> {
        let intent = match view.register {
            FixturePhase::Boot => {
                assert_eq!(view.head.path(), "root/input.txt");
                assert_eq!(view.current_content.as_deref(), Some("seed"));
                IntentEnvelope {
                    proposed_register: FixturePhase::Finalize,
                    action_payload: None,
                    proposed_head: Head::new("root/result.txt"),
                    write_mode: WriteMode::Overwrite,
                    write_content: Some("prepared".to_owned()),
                    halt: false,
                    notes: "seed->prepared".to_owned(),
                }
            }
            FixturePhase::Finalize => {
                assert_eq!(view.head.path(), "root/result.txt");
                assert_eq!(view.current_content, None);
                IntentEnvelope {
                    proposed_register: FixturePhase::Done,
                    action_payload: None,
                    proposed_head: Head::new("root/result.txt"),
                    write_mode: WriteMode::Overwrite,
                    write_content: Some("ok".to_owned()),
                    halt: true,
                    notes: "write-result".to_owned(),
                }
            }
            FixturePhase::Done => panic!("halted snapshot must short-circuit before adapter"),
        };

        AdapterOutcome::Intent {
            observation: StepObservation {
                provenance: "fixture-success".to_owned(),
            },
            intent,
        }
    }
}

impl IntentAdapter<FixturePhase> for RejectingAdapter {
    fn propose(&self, view: &ReadView<FixturePhase>) -> AdapterOutcome<FixturePhase> {
        assert_eq!(view.head.path(), "root/input.txt");
        AdapterOutcome::Intent {
            observation: StepObservation {
                provenance: "fixture-reject".to_owned(),
            },
            intent: IntentEnvelope {
                proposed_register: FixturePhase::Finalize,
                action_payload: None,
                proposed_head: Head::new("root/wrong.txt"),
                write_mode: WriteMode::Overwrite,
                write_content: Some("bad".to_owned()),
                halt: false,
                notes: "wrong-path".to_owned(),
            },
        }
    }
}

fn initial_snapshot() -> UniverseSnapshot<FixturePhase> {
    UniverseSnapshot::new(
        FixturePhase::Boot,
        Head::new("root/input.txt"),
        TapeState::default().with_write("root/input.txt", "seed"),
        TraceHash::genesis(),
        StepIndex(0),
    )
}

#[test]
fn real_success_run_projects_through_export_summary_and_frame() {
    let engine = KernelEngine::new(KernelConfig::default());
    let outcome = engine.run(initial_snapshot(), &SuccessAdapter, &FixtureTask, 4);

    match &outcome.stop {
        RunStop::Halted {
            final_snapshot,
            status,
        } => {
            assert_eq!(status, &HaltStatus::Success);
            assert_eq!(final_snapshot.step(), StepIndex(2));
            assert_eq!(final_snapshot.register(), &FixturePhase::Done);
            assert_eq!(final_snapshot.head().path(), "root/result.txt");
            assert_eq!(
                final_snapshot.tape().read("root/input.txt"),
                Some("prepared")
            );
            assert_eq!(final_snapshot.tape().read("root/result.txt"), Some("ok"));
        }
        other => panic!("expected halted success, got {other:?}"),
    }

    assert_eq!(outcome.attempted_steps, 2);
    assert_eq!(outcome.committed_steps.len(), 2);
    assert_eq!(outcome.committed_steps[0].record.notes, "seed->prepared");
    assert_eq!(outcome.committed_steps[1].record.notes, "write-result");

    let report = outcome.report();
    let export = RunExport::from(outcome.report());
    let summary = export.summary();
    let frame = export.pulse_frame();

    assert_eq!(report.attempted_steps(), 2);
    assert_eq!(report.committed_steps().len(), 2);
    assert_eq!(
        report.committed_steps()[0].observation.provenance,
        "fixture-success"
    );
    assert_eq!(report.committed_steps()[1].record.notes, "write-result");
    assert_eq!(report.terminal_snapshot().step(), StepIndex(2));
    assert_eq!(
        report.terminal_snapshot().tape().read("root/result.txt"),
        Some("ok")
    );
    assert_eq!(
        report.stop(),
        turingos_kernel::RunReportStop::Halted {
            status: &HaltStatus::Success
        }
    );

    assert_eq!(export.attempted_steps, 2);
    assert_eq!(export.committed_steps.len(), 2);
    assert_eq!(
        export.committed_steps[0].observation.provenance,
        "fixture-success"
    );
    assert_eq!(export.committed_steps[0].record.notes, "seed->prepared");
    assert_eq!(
        export.committed_steps[1].observation.provenance,
        "fixture-success"
    );
    assert_eq!(export.committed_steps[1].record.notes, "write-result");
    assert_eq!(
        export.terminal_snapshot.tape().read("root/result.txt"),
        Some("ok")
    );
    assert_eq!(
        export.stop,
        turingos_observe::RunExportStop::Halted {
            status: HaltStatus::Success
        }
    );

    assert_eq!(summary.attempted_steps(), 2);
    assert_eq!(summary.committed_step_count(), 2);
    assert_eq!(summary.terminal_step(), StepIndex(2));
    assert_eq!(summary.stop_class(), RunSummaryStopClass::Halted);
    assert_eq!(summary.halt_status(), Some(&HaltStatus::Success));
    assert!(summary.is_success());

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

#[test]
fn real_abort_run_projects_through_export_summary_and_frame() {
    let engine = KernelEngine::new(KernelConfig::default());
    let outcome = engine.run(initial_snapshot(), &RejectingAdapter, &FixtureTask, 4);

    match &outcome.stop {
        RunStop::Abort {
            preserved_snapshot,
            observation,
            abort,
        } => {
            assert_eq!(preserved_snapshot.step(), StepIndex(0));
            assert_eq!(preserved_snapshot.register(), &FixturePhase::Boot);
            assert_eq!(
                preserved_snapshot.tape().read("root/input.txt"),
                Some("seed")
            );
            assert_eq!(observation.provenance, "fixture-reject");
            assert_eq!(abort.reject.step, StepIndex(0));
            assert_eq!(abort.rejected_intent.notes, "wrong-path");
        }
        other => panic!("expected abort, got {other:?}"),
    }

    assert_eq!(outcome.attempted_steps, 1);
    assert!(outcome.committed_steps.is_empty());

    let report = outcome.report();
    let export = RunExport::from(outcome.report());
    let summary = export.summary();
    let frame = export.pulse_frame();

    assert_eq!(report.attempted_steps(), 1);
    assert_eq!(report.committed_steps().len(), 0);
    assert_eq!(report.terminal_snapshot().step(), StepIndex(0));
    assert_eq!(
        report.terminal_snapshot().tape().read("root/input.txt"),
        Some("seed")
    );
    match report.stop() {
        turingos_kernel::RunReportStop::Abort { observation, abort } => {
            assert_eq!(observation.provenance, "fixture-reject");
            assert_eq!(abort.reject.step, StepIndex(0));
            assert_eq!(abort.rejected_intent.notes, "wrong-path");
        }
        other => panic!("expected report abort, got {other:?}"),
    }

    assert_eq!(export.attempted_steps, 1);
    assert_eq!(export.committed_steps.len(), 0);
    assert_eq!(export.terminal_snapshot.step(), StepIndex(0));
    assert_eq!(export.terminal_snapshot.register(), &FixturePhase::Boot);
    assert_eq!(
        export.terminal_snapshot.tape().read("root/input.txt"),
        Some("seed")
    );
    match &export.stop {
        turingos_observe::RunExportStop::Abort { observation, abort } => {
            assert_eq!(observation.provenance, "fixture-reject");
            assert_eq!(abort.reject.step, StepIndex(0));
            assert_eq!(abort.rejected_intent.notes, "wrong-path");
        }
        other => panic!("expected export abort, got {other:?}"),
    }

    assert_eq!(summary.attempted_steps(), 1);
    assert_eq!(summary.committed_step_count(), 0);
    assert_eq!(summary.terminal_step(), StepIndex(0));
    assert_eq!(summary.stop_class(), RunSummaryStopClass::Abort);
    assert_eq!(summary.halt_status(), None);
    assert!(!summary.is_success());

    assert_eq!(frame.basis(), &export.basis());
    assert_eq!(
        frame.class().transition(),
        RunPulseTransitionClass::CurrentOnly
    );
    assert_eq!(frame.class().terminal(), RunPulseTerminalClass::Abort);
}

#[test]
fn cross_run_basis_carry_preserves_regressed_abort_projection() {
    let engine = KernelEngine::new(KernelConfig::default());
    let success_export = RunExport::from(
        engine
            .run(initial_snapshot(), &SuccessAdapter, &FixtureTask, 4)
            .report(),
    );
    let abort_export = RunExport::from(
        engine
            .run(initial_snapshot(), &RejectingAdapter, &FixtureTask, 4)
            .report(),
    );

    let frame = abort_export.pulse_frame_after_basis(&success_export.basis());

    assert_eq!(frame.basis(), &abort_export.basis());
    assert_eq!(
        frame.class().transition(),
        RunPulseTransitionClass::Regressed
    );
    assert_eq!(frame.class().terminal(), RunPulseTerminalClass::Abort);
}
