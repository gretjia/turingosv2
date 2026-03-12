#![cfg_attr(not(test), allow(dead_code))]

use turingos_core::{ReadView, UniverseSnapshot};

/// WHITEBOX read projection.
///
/// This is the Rust embodiment of:
/// `INPUT_t[WHITEBOX] := <q_t, ReadTool_WHITEBOX(tape_t, HEAD_t)>`.
pub(crate) fn project_read_view<QState: Clone>(
    snapshot: &UniverseSnapshot<QState>,
) -> ReadView<QState> {
    ReadView {
        register: snapshot.register().clone(),
        head: snapshot.head().clone(),
        current_content: snapshot
            .tape()
            .read(snapshot.head().path())
            .map(str::to_owned),
    }
}

#[cfg(test)]
mod tests {
    use turingos_core::{Head, StepIndex, TapeState, TraceHash, UniverseSnapshot};

    use super::project_read_view;

    #[test]
    fn read_view_is_projection_only() {
        let tape = TapeState::default().with_write("root/.ls", "DIR a");
        let snapshot = UniverseSnapshot::new(
            7_u8,
            Head::new("root/.ls"),
            tape,
            TraceHash::genesis(),
            StepIndex(0),
        );
        let view = project_read_view(&snapshot);
        assert_eq!(view.register, 7_u8);
        assert_eq!(view.current_content.as_deref(), Some("DIR a"));
        assert_eq!(snapshot.step(), StepIndex(0));
    }

    #[test]
    fn missing_path_projects_as_none() {
        let snapshot = UniverseSnapshot::new(
            7_u8,
            Head::new("root/.missing"),
            TapeState::default(),
            TraceHash::genesis(),
            StepIndex(0),
        );
        let view = project_read_view(&snapshot);
        assert_eq!(view.current_content, None);
    }
}
