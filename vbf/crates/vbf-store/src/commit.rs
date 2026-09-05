use crate::{TransactionError, WorldState, transaction::apply_mutations};
use std::error::Error;
use std::fmt;
use vbf_event::EventEnvelope;
use vbf_types::{EventSequence, EventUid, SimTime};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitError {
    DuplicateEvent(EventUid),
    SequenceMismatch {
        expected: EventSequence,
        actual: EventSequence,
    },
    SequenceOverflow,
    TimeRegression {
        current: SimTime,
        event: SimTime,
    },
    Mutation(TransactionError),
}
impl fmt::Display for CommitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateEvent(uid) => write!(f, "event already committed: {uid}"),
            Self::SequenceMismatch { expected, actual } => write!(
                f,
                "event sequence mismatch: expected {}, received {}",
                expected.get(),
                actual.get()
            ),
            Self::SequenceOverflow => write!(f, "event sequence overflowed"),
            Self::TimeRegression { current, event } => write!(
                f,
                "event time regressed from {} ms to {} ms",
                current.as_millis(),
                event.as_millis()
            ),
            Self::Mutation(error) => write!(f, "event mutation failed: {error}"),
        }
    }
}
impl Error for CommitError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Mutation(error) => Some(error),
            _ => None,
        }
    }
}
impl From<TransactionError> for CommitError {
    fn from(value: TransactionError) -> Self {
        Self::Mutation(value)
    }
}
pub(crate) fn commit_event(
    state: &mut WorldState,
    history: &mut Vec<EventEnvelope>,
    event: EventEnvelope,
) -> Result<(), CommitError> {
    if history.iter().any(|committed| committed.uid == event.uid) {
        return Err(CommitError::DuplicateEvent(event.uid));
    }
    let expected_sequence = match history.last() {
        None => EventSequence::ZERO,
        Some(last) => last.sequence.next().ok_or(CommitError::SequenceOverflow)?,
    };
    if event.sequence != expected_sequence {
        return Err(CommitError::SequenceMismatch {
            expected: expected_sequence,
            actual: event.sequence,
        });
    }
    if event.sim_time < state.sim_time {
        return Err(CommitError::TimeRegression {
            current: state.sim_time,
            event: event.sim_time,
        });
    }
    let mut candidate = state.clone();
    apply_mutations(&mut candidate, &event.mutations)?;
    candidate.sim_time = event.sim_time;
    *state = candidate;
    history.push(event);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use vbf_event::{EventOrigin, EventTypeRef};
    use vbf_types::{EventUid, Key};
    fn event(sequence: u64, sim_time_ms: i64) -> EventEnvelope {
        EventEnvelope {
            uid: EventUid::new(),
            sequence: EventSequence::new(sequence),
            sim_time: SimTime::from_millis(sim_time_ms),
            event_type: EventTypeRef::new(Key::new("event.test").expect("valid test key"), 1)
                .expect("valid event type version"),
            origin: EventOrigin::System,
            cause: None,
            correlation: None,
            participants: Vec::new(),
            payload: Default::default(),
            mutations: Vec::new(),
        }
    }
    #[test]
    fn committed_event_advances_time_revision_and_history() {
        let mut state = WorldState::default();
        let initial_revision = state.revision;
        let mut history = Vec::new();
        commit_event(&mut state, &mut history, event(0, 1_250)).expect("first event should commit");
        assert_eq!(state.sim_time, SimTime::from_millis(1_250));
        assert!(state.revision > initial_revision);
        assert_eq!(history.len(), 1);
    }
    #[test]
    fn sequence_gap_is_rejected_without_mutating_state() {
        let mut state = WorldState::default();
        let initial_revision = state.revision;
        let mut history = Vec::new();
        let result = commit_event(&mut state, &mut history, event(1, 1_250));
        assert!(matches!(result, Err(CommitError::SequenceMismatch { .. })));
        assert_eq!(state.revision, initial_revision);
        assert!(history.is_empty());
    }
    #[test]
    fn simulation_time_cannot_regress_after_a_commit() {
        let mut state = WorldState::default();
        let mut history = Vec::new();
        commit_event(&mut state, &mut history, event(0, 1_250)).expect("first event should commit");
        let result = commit_event(&mut state, &mut history, event(1, 1_249));
        assert!(matches!(result, Err(CommitError::TimeRegression { .. })));
        assert_eq!(state.sim_time, SimTime::from_millis(1_250));
        assert_eq!(history.len(), 1);
    }
}
