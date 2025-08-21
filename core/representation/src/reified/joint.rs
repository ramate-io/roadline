use roadline_util::dependency::Id;
use serde::{Deserialize, Serialize};
use super::reified_unit::ReifiedUnit;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct JointUnit(ReifiedUnit);

impl JointUnit {
    pub fn new(unit: ReifiedUnit) -> Self {
        Self(unit)
    }

    pub fn value(&self) -> ReifiedUnit {
        self.0
    }
}


/// A point in 2D space using reified coordinates
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ConnectionPoint {
    pub x: ReifiedUnit,
    pub y: ReifiedUnit,
}

impl ConnectionPoint {
    pub fn new(x: ReifiedUnit, y: ReifiedUnit) -> Self {
        Self { x, y }
    }
}

/// A Bezier curve connection between two tasks
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BezierConnection {
    pub start: ConnectionPoint,
    pub end: ConnectionPoint,
    pub control1: ConnectionPoint,
    pub control2: ConnectionPoint,
}

impl BezierConnection {
    pub fn new(start: ConnectionPoint, end: ConnectionPoint, control1: ConnectionPoint, control2: ConnectionPoint) -> Self {
        Self { start, end, control1, control2 }
    }

    /// Create a flowing Bezier curve between two connection points
    pub fn flowing_curve(start: ConnectionPoint, end: ConnectionPoint) -> Self {
        // Calculate control points for a smooth curve
        let horizontal_distance = end.x.value().saturating_sub(start.x.value());
        let curve_distance = (horizontal_distance * 40) / 100; // 40% of horizontal distance

        let control1 = ConnectionPoint::new(
            ReifiedUnit::new(start.x.value() + curve_distance),
            start.y, // Stay at start height
        );

        let control2 = ConnectionPoint::new(
            ReifiedUnit::new(end.x.value().saturating_sub(curve_distance)),
            end.y, // Stay at end height
        );

        Self::new(start, end, control1, control2)
    }
}

/// The joint represents a connection between tasks with its routing information
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Joint {
    dependency: Id,
    bezier_connection: BezierConnection,
}

impl Joint {
    pub fn new(dependency: Id, bezier_connection: BezierConnection) -> Self {
        Self { dependency, bezier_connection }
    }

    pub fn dependency(&self) -> &Id {
        &self.dependency
    }

    pub fn bezier_connection(&self) -> &BezierConnection {
        &self.bezier_connection
    }

    /// Create a joint with a flowing Bezier curve between two connection points
    pub fn flowing_joint(dependency: Id, start: ConnectionPoint, end: ConnectionPoint) -> Self {
        let bezier_connection = BezierConnection::flowing_curve(start, end);
        Self::new(dependency, bezier_connection)
    }
}