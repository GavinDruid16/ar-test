use crate::WorldState;
use std::error::Error;
use std::fmt;
use vbf_event::StateMutation;
use vbf_types::{EntityUid, RelationshipUid};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransactionError {
    DuplicateEntity(EntityUid),
    MissingEntity(EntityUid),
    DuplicateRelationship(RelationshipUid),
    MissingRelationship(RelationshipUid),
    RevisionOverflow,
}
impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateEntity(uid) => write!(f, "entity already exists: {uid}"),
            Self::MissingEntity(uid) => write!(f, "entity does not exist: {uid}"),
            Self::DuplicateRelationship(uid) => write!(f, "relationship already exists: {uid}"),
            Self::MissingRelationship(uid) => write!(f, "relationship does not exist: {uid}"),
            Self::RevisionOverflow => write!(f, "state revision overflowed"),
        }
    }
}
impl Error for TransactionError {}
/// Apply a set of primitive mutations atomically.
///
/// This initial implementation clones the candidate state, applies all changes,
/// and commits only if every primitive mutation succeeds. Schema/domain
/// validation will wrap this transaction boundary as Layer 0 deepens.
pub fn apply_mutations(
    state: &mut WorldState,
    mutations: &[StateMutation],
) -> Result<(), TransactionError> {
    let mut candidate = state.clone();
    for mutation in mutations {
        match mutation {
            StateMutation::AddEntity { entity } => {
                if candidate.entities.contains_key(&entity.uid) {
                    return Err(TransactionError::DuplicateEntity(entity.uid));
                }
                candidate.entities.insert(entity.uid, entity.clone());
            }
            StateMutation::RemoveEntity { entity } => {
                if candidate.entities.remove(entity).is_none() {
                    return Err(TransactionError::MissingEntity(*entity));
                }
                candidate.positions.remove(entity);
                candidate.orientations.remove(entity);
                candidate.linear_velocities.remove(entity);
                candidate.angular_velocities.remove(entity);
            }
            StateMutation::SetComponent { entity, component } => {
                let target = candidate
                    .entities
                    .get_mut(entity)
                    .ok_or(TransactionError::MissingEntity(*entity))?;
                target.set_component(component.clone());
            }
            StateMutation::RemoveComponent { entity, component } => {
                let target = candidate
                    .entities
                    .get_mut(entity)
                    .ok_or(TransactionError::MissingEntity(*entity))?;
                target.components.remove(component);
            }
            StateMutation::AddRelationship { relationship } => {
                if candidate.relationships.contains_key(&relationship.uid) {
                    return Err(TransactionError::DuplicateRelationship(relationship.uid));
                }
                candidate
                    .relationships
                    .insert(relationship.uid, relationship.clone());
            }
            StateMutation::EndRelationship { relationship } => {
                if candidate.relationships.remove(relationship).is_none() {
                    return Err(TransactionError::MissingRelationship(*relationship));
                }
            }
            StateMutation::SetPosition { entity, position } => {
                if !candidate.entities.contains_key(entity) {
                    return Err(TransactionError::MissingEntity(*entity));
                }
                candidate.positions.insert(*entity, position.clone());
            }
            StateMutation::SetOrientation {
                entity,
                orientation,
            } => {
                if !candidate.entities.contains_key(entity) {
                    return Err(TransactionError::MissingEntity(*entity));
                }
                candidate.orientations.insert(*entity, *orientation);
            }
            StateMutation::SetLinearVelocity { entity, velocity } => {
                if !candidate.entities.contains_key(entity) {
                    return Err(TransactionError::MissingEntity(*entity));
                }
                candidate.linear_velocities.insert(*entity, *velocity);
            }
            StateMutation::SetAngularVelocity { entity, velocity } => {
                if !candidate.entities.contains_key(entity) {
                    return Err(TransactionError::MissingEntity(*entity));
                }
                candidate.angular_velocities.insert(*entity, *velocity);
            }
        }
    }
    candidate.revision = candidate
        .revision
        .next()
        .map_err(|_| TransactionError::RevisionOverflow)?;
    *state = candidate;
    Ok(())
}
