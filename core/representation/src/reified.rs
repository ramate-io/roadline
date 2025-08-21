pub mod down_stretch;
pub mod down_lane;
pub mod down_cell;
pub mod joint;
pub mod reified_unit;

pub use down_cell::DownCell;
pub use down_lane::{DownLane, DownLanePadding};
pub use down_stretch::{DownStretch, Trim};
pub use joint::{Joint, ConnectionPoint, BezierConnection};
pub use reified_unit::ReifiedUnit;

use crate::grid_algebra::GridAlgebra;
use roadline_util::dependency::Id as DependencyId;
use roadline_util::task::Id as TaskId;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReifiedError {
    #[error("Task not found: {task_id:?}")]
    TaskNotFound { task_id: TaskId },
    #[error("Dependency not found: {dependency_id:?}")]
    DependencyNotFound { dependency_id: DependencyId },
}

/// Configuration for the visual layer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReifiedConfig {
    pub connection_trim: Trim,
    pub inter_lane_padding: DownLanePadding,
}

impl ReifiedConfig {
    pub fn new(connection_trim: Trim, inter_lane_padding: DownLanePadding) -> Self {
        Self { connection_trim, inter_lane_padding }
    }

    /// Default configuration with reasonable visual spacing
    pub fn default_config() -> Self {
        Self {
            connection_trim: Trim::new(ReifiedUnit::new(10)), // 10 units of gutter space
            inter_lane_padding: DownLanePadding::new(ReifiedUnit::new(2)), // 2 units between lanes
        }
    }
}

/// Pre-computation state for reified visual layer
pub struct PreReified {
    grid: GridAlgebra,
    config: ReifiedConfig,
}

impl PreReified {
    pub fn new(grid: GridAlgebra) -> Self {
        Self {
            grid,
            config: ReifiedConfig::default_config(),
        }
    }

    pub fn with_config(grid: GridAlgebra, config: ReifiedConfig) -> Self {
        Self { grid, config }
    }

    pub fn compute(self) -> Result<Reified, ReifiedError> {
        let mut down_cells = HashMap::new();
        let mut joints = HashMap::new();

        // Step 1: Create DownCells for all tasks
        for (task_id, cell) in self.grid.tasks() {
            let down_stretch = DownStretch::canonical_from_stretch(cell.stretch().clone(), self.config.connection_trim.clone());
            let down_lane = DownLane::canonical_from_lane(*cell.lane(), self.config.inter_lane_padding.clone());
            let down_cell = DownCell::new(cell.clone(), down_lane, down_stretch);
            
            down_cells.insert(*task_id, down_cell);
        }

        // Step 2: Create Joints for all dependencies
        for (task_id, _cell) in self.grid.tasks() {
            let dependencies = self.grid.range_algebra().graph().get_dependencies(task_id);
            
            for dependency_task_id in dependencies {
                // Create dependency ID from the from->to relationship
                let dependency_id = DependencyId::new(dependency_task_id, *task_id);
                
                // Get connection points
                let dependent_cell = down_cells.get(task_id)
                    .ok_or(ReifiedError::TaskNotFound { task_id: *task_id })?;
                let dependency_cell = down_cells.get(&dependency_task_id)
                    .ok_or(ReifiedError::TaskNotFound { task_id: dependency_task_id })?;

                let start_point = {
                    let (x, y) = dependency_cell.outgoing_connection_point();
                    ConnectionPoint::new(x, y)
                };
                
                let end_point = {
                    let (x, y) = dependent_cell.incoming_connection_point();
                    ConnectionPoint::new(x, y)
                };

                // Create flowing joint
                let joint = Joint::flowing_joint(dependency_id, start_point, end_point);
                joints.insert(dependency_id, joint);
            }
        }

        Ok(Reified {
            grid: self.grid,
            config: self.config,
            down_cells,
            joints,
        })
    }
}

/// Immutable reified visual representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reified {
    grid: GridAlgebra,
    config: ReifiedConfig,
    down_cells: HashMap<TaskId, DownCell>,
    joints: HashMap<DependencyId, Joint>,
}

impl Reified {
    pub fn grid(&self) -> &GridAlgebra {
        &self.grid
    }

    pub fn config(&self) -> &ReifiedConfig {
        &self.config
    }

    pub fn down_cells(&self) -> &HashMap<TaskId, DownCell> {
        &self.down_cells
    }

    pub fn joints(&self) -> &HashMap<DependencyId, Joint> {
        &self.joints
    }

    pub fn get_down_cell(&self, task_id: &TaskId) -> Option<&DownCell> {
        self.down_cells.get(task_id)
    }

    pub fn get_joint(&self, dependency_id: &DependencyId) -> Option<&Joint> {
        self.joints.get(dependency_id)
    }

    /// Get all tasks with their visual bounds
    pub fn task_bounds(&self) -> impl Iterator<Item = (&TaskId, &DownCell)> {
        self.down_cells.iter()
    }

    /// Get all connections with their visual routing
    pub fn connections(&self) -> impl Iterator<Item = (&DependencyId, &Joint)> {
        self.joints.iter()
    }

    /// Get the maximum visual bounds for layout purposes
    pub fn visual_bounds(&self) -> (ReifiedUnit, ReifiedUnit) {
        let max_x = self.down_cells.values()
            .map(|cell| cell.down_stretch().down_stretch().end().value())
            .max()
            .unwrap_or(0);

        let max_y = self.down_cells.values()
            .map(|cell| cell.down_lane().range().end().value())
            .max()
            .unwrap_or(0);

        (ReifiedUnit::new(max_x), ReifiedUnit::new(max_y))
    }

    /// Count total number of tasks
    pub fn task_count(&self) -> usize {
        self.down_cells.len()
    }

    /// Count total number of connections
    pub fn connection_count(&self) -> usize {
        self.joints.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid_algebra::{PreGridAlgebra};
    use crate::range_algebra::{PreRangeAlgebra, Date};
    use crate::graph::Graph;
    use roadline_util::task::{Task, range::{Start, End, PointOfReference, TargetDate}};
    use roadline_util::duration::Duration;
    use chrono::{DateTime, Utc};
    use std::collections::BTreeSet;
    use std::time::Duration as StdDuration;

    /// Creates a test date from an ISO string.
    fn test_date(iso_string: &str) -> Date {
        let datetime = DateTime::parse_from_rfc3339(iso_string)
            .expect("Valid datetime string")
            .with_timezone(&Utc);
        Date::new(datetime)
    }

    /// Creates a test task with the specified parameters.
    fn create_test_task(
        id: u8, 
        reference_id: u8, 
        offset_days: u64, 
        duration_days: u64
    ) -> Result<Task, anyhow::Error> {
        let id = TaskId::new(id);
        let reference_id = TaskId::new(reference_id);
        
        let start = Start::from(TargetDate {
            point_of_reference: PointOfReference::from(reference_id),
            duration: Duration::from(StdDuration::from_secs(offset_days * 24 * 60 * 60)),
        });
        
        let end = End::from(Duration::from(StdDuration::from_secs(duration_days * 24 * 60 * 60)));
        
        let range = roadline_util::task::Range::new(start, end);
        
        Ok(Task::new(
            id,
            roadline_util::task::Title::new_test(),
            BTreeSet::new(),
            BTreeSet::new(),
            roadline_util::task::Summary::new_test(),
            range,
        ))
    }

    fn create_test_task_with_dependencies(
        id: u8, 
        reference_id: u8, 
        offset_days: u64, 
        duration_days: u64,
        dependencies: BTreeSet<u8>
    ) -> Result<Task, anyhow::Error> {
        let mut task = create_test_task(id, reference_id, offset_days, duration_days)?;
        task.dependencies_mut().extend(dependencies.into_iter().map(|id| TaskId::new(id)));
        Ok(task)
    }

    #[test]
    fn test_reified_basic_functionality() -> Result<(), anyhow::Error> {
        // Create a simple graph: Task1 -> Task2
        let mut graph = Graph::new();
        
        let task1 = create_test_task_with_dependencies(1, 1, 0, 10 * 24 * 60 * 60, BTreeSet::new())?;
        let task2 = create_test_task_with_dependencies(2, 1, 5 * 24 * 60 * 60, 10 * 24 * 60 * 60, BTreeSet::from_iter([1]))?;
        
        graph.add(task1)?;
        graph.add(task2)?;

        // Build the layers
        let range_algebra = PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
        let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;
        let reified = PreReified::new(grid_algebra).compute()?;

        // Verify basic structure
        assert_eq!(reified.task_count(), 2);
        assert_eq!(reified.connection_count(), 1);

        // Verify tasks have down cells
        assert!(reified.get_down_cell(&TaskId::new(1)).is_some());
        assert!(reified.get_down_cell(&TaskId::new(2)).is_some());

        // Verify connection exists
        let dependency_id = DependencyId::new(TaskId::new(1), TaskId::new(2));
        assert!(reified.get_joint(&dependency_id).is_some());

        Ok(())
    }

    #[test]
    fn test_reified_visual_bounds() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        
        let task1 = create_test_task_with_dependencies(1, 1, 0, 10 * 24 * 60 * 60, BTreeSet::new())?;
        let task2 = create_test_task_with_dependencies(2, 1, 5 * 24 * 60 * 60, 15 * 24 * 60 * 60, BTreeSet::from_iter([1]))?;
        
        graph.add(task1)?;
        graph.add(task2)?;

        let range_algebra = PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
        let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;
        let reified = PreReified::new(grid_algebra).compute()?;

        // Check visual bounds are reasonable
        let (max_x, max_y) = reified.visual_bounds();
        assert!(max_x.value() > 0);
        assert!(max_y.value() > 0);

        println!("Visual bounds: x={}, y={}", max_x.value(), max_y.value());

        Ok(())
    }

    #[test]
    fn test_reified_connection_points() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        
        let task1 = create_test_task_with_dependencies(1, 1, 0, 10 * 24 * 60 * 60, BTreeSet::new())?;
        let task2 = create_test_task_with_dependencies(2, 1, 5 * 24 * 60 * 60, 10 * 24 * 60 * 60, BTreeSet::from_iter([1]))?;
        
        graph.add(task1)?;
        graph.add(task2)?;

        let range_algebra = PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
        let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;
        let reified = PreReified::new(grid_algebra).compute()?;

        // Check that connections have reasonable control points
        let dependency_id = DependencyId::new(TaskId::new(1), TaskId::new(2));
        let joint = reified.get_joint(&dependency_id).expect("Joint should exist");
        
        let bezier = joint.bezier_connection();
        
        // Start should be to the left of end (dependency ends before dependent starts)
        assert!(bezier.start.x.value() <= bezier.end.x.value());
        
        // Control points should create a reasonable curve
        assert!(bezier.control1.x.value() > bezier.start.x.value());
        assert!(bezier.control2.x.value() < bezier.end.x.value());

        println!("Bezier curve: start=({},{}), end=({},{}), control1=({},{}), control2=({},{})",
                 bezier.start.x.value(), bezier.start.y.value(),
                 bezier.end.x.value(), bezier.end.y.value(),
                 bezier.control1.x.value(), bezier.control1.y.value(),
                 bezier.control2.x.value(), bezier.control2.y.value());

        Ok(())
    }

    #[test]
    fn test_reified_custom_config() -> Result<(), anyhow::Error> {
        let mut graph = Graph::new();
        
        // Create two tasks so the range algebra works properly
        let task1 = create_test_task_with_dependencies(1, 1, 0, 10 * 24 * 60 * 60, BTreeSet::new())?;
        let task2 = create_test_task_with_dependencies(2, 1, 5 * 24 * 60 * 60, 10 * 24 * 60 * 60, BTreeSet::from_iter([1]))?;
        
        graph.add(task1)?;
        graph.add(task2)?;

        let range_algebra = PreRangeAlgebra::new(graph).compute(test_date("2021-01-01T00:00:00Z"))?;
        let grid_algebra = PreGridAlgebra::new(range_algebra).compute()?;

        // Test custom configuration
        let config = ReifiedConfig::new(
            Trim::new(ReifiedUnit::new(20)), // Larger trim
            DownLanePadding::new(ReifiedUnit::new(5)) // Larger padding
        );
        
        let reified = PreReified::with_config(grid_algebra, config).compute()?;

        assert_eq!(reified.task_count(), 2);
        assert_eq!(reified.config().connection_trim.value().value(), 20);
        assert_eq!(reified.config().inter_lane_padding.value().value(), 5);

        Ok(())
    }
}