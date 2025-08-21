//! Unified API for the Roadline representation system.
//! 
//! This module provides a high-level, builder-pattern API that orchestrates
//! the entire pipeline from task graphs to visual representations.

use crate::reified::{
    Reified, PreReified, ReifiedConfig, ReifiedError,
    Trim, DownLanePadding, ReifiedUnit, 
    DownCell, Joint, ConnectionPoint
};
use crate::grid_algebra::{PreGridAlgebra, GridAlgebra, GridAlgebraError};
use crate::range_algebra::{PreRangeAlgebra, RangeAlgebra, RangeAlgebraError, Date};
use crate::graph::{Graph, GraphError};
use roadline_util::task::{Task, Id as TaskId};
use roadline_util::dependency::Id as DependencyId;
use serde::{Serialize, Deserialize};
use thiserror::Error;

/// Comprehensive error type for the Roadline builder
#[derive(Debug, Error)]
pub enum RoadlineBuilderError {
    // === Graph Errors ===
    #[error("Task already exists in graph: {task_id:?}")]
    TaskAlreadyExists { task_id: TaskId },
    
    #[error("Circular dependency detected: {task_id:?} cannot depend on itself")]
    CircularDependency { task_id: TaskId },
    
    #[error("Invalid dependency: task {from:?} → {to:?} (dependency target not found)")]
    InvalidDependency { from: TaskId, to: TaskId },
    
    // === Range Algebra Errors ===
    #[error("Graph contains cycles: {cycles:?}")]
    GraphHasCycles { cycles: Vec<Vec<TaskId>> },
    
    #[error("Task dependency constraint violated: {task_id:?} → {dependency:?} (dependency must end before task starts)")]
    DependencyConstraintViolated { task_id: TaskId, dependency: TaskId },
    
    #[error("Task not found in range calculation: {task_id:?}")]
    TaskNotFoundInRange { task_id: TaskId },
    
    #[error("Multiple range algebra errors: {error_count} errors detected")]
    MultipleRangeErrors { error_count: usize, first_error: String },
    
    #[error("No tasks found in range algebra - empty graph")]
    NoTasksInRange,
    
    // === Grid Algebra Errors ===
    #[error("Failed to determine time scale unit from task durations")]
    TimeScaleDeterminationFailed,
    
    #[error("Lane assignment failed: unable to place task {task_id:?} in available lanes")]
    LaneAssignmentFailed { task_id: TaskId },
    
    #[error("Grid computation failed: {reason}")]
    GridComputationFailed { reason: String },
    
    // === Reified Errors ===
    #[error("Task not found in reified layer: {task_id:?}")]
    TaskNotFoundInReified { task_id: TaskId },
    
    #[error("Dependency connection not found: {dependency_id:?}")]
    DependencyNotFoundInReified { dependency_id: DependencyId },
    
    // === Builder-specific Errors ===
    #[error("Date parsing error: {message}")]
    DateParsing { message: String },
    
    #[error("No tasks provided - at least one task is required to build a roadline")]
    NoTasks,
    
    #[error("Internal error during pipeline execution: {stage} - {message}")]
    InternalError { stage: String, message: String },
}

/// Summary information about a RoadlineBuilder's current state.
#[derive(Debug, Clone, PartialEq)]
pub struct BuilderSummary {
    pub task_count: usize,
    pub root_date: Date,
    pub trim_units: u16,
    pub padding_units: u16,
}

impl std::fmt::Display for BuilderSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "RoadlineBuilder: {} tasks, root: {:?}, spacing: {}×{}", 
            self.task_count, 
            self.root_date, 
            self.trim_units, 
            self.padding_units
        )
    }
}

impl RoadlineBuilderError {
    /// Map GraphError to specific RoadlineBuilderError variants
    fn from_graph_error(error: GraphError) -> Self {
        match error {
            GraphError::Internal(boxed_error) => {
                Self::InternalError {
                    stage: "graph".to_string(),
                    message: boxed_error.to_string(),
                }
            }
        }
    }

    /// Map RangeAlgebraError to specific RoadlineBuilderError variants  
    fn from_range_algebra_error(error: RangeAlgebraError) -> Self {
        match error {
            RangeAlgebraError::Graph(graph_error) => Self::from_graph_error(graph_error),
            RangeAlgebraError::TaskNotFound { task_id } => Self::TaskNotFoundInRange { task_id },
            RangeAlgebraError::InvalidRange { task_id } => Self::InternalError {
                stage: "range_algebra".to_string(),
                message: format!("Invalid range specification for task {:?}", task_id),
            },
            RangeAlgebraError::InvalidReference { task_id, reference_id } => Self::InternalError {
                stage: "range_algebra".to_string(),
                message: format!("Task {:?} references non-existent task {:?}", task_id, reference_id),
            },
            RangeAlgebraError::InvalidRootRange { task_id } => Self::InternalError {
                stage: "range_algebra".to_string(),
                message: format!("Root task {:?} must reference itself with +0 offset", task_id),
            },
            RangeAlgebraError::TooEarlyForDependency { task_id, dependency_id } => {
                Self::DependencyConstraintViolated { task_id, dependency: dependency_id }
            },
            RangeAlgebraError::NoRootTasks => Self::NoTasksInRange,
            RangeAlgebraError::OnlyRootTasksCanSelfReference { task_id, .. } => Self::InternalError {
                stage: "range_algebra".to_string(),
                message: format!("Task {:?} cannot self-reference - only root tasks can", task_id),
            },
            RangeAlgebraError::Multiple { errors } => {
                let first_error = errors.first()
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "Unknown error".to_string());
                Self::MultipleRangeErrors {
                    error_count: errors.len(),
                    first_error,
                }
            },
            RangeAlgebraError::GraphHasCycles { cycles } => Self::GraphHasCycles { cycles },
            RangeAlgebraError::InvalidDate { date } => Self::DateParsing { message: date },
        }
    }

    /// Map GridAlgebraError to specific RoadlineBuilderError variants
    fn from_grid_algebra_error(error: GridAlgebraError) -> Self {
        match error {
            GridAlgebraError::TaskNotFound { task_id } => Self::TaskNotFoundInRange { task_id },
            GridAlgebraError::NoTasks => Self::NoTasksInRange,
            GridAlgebraError::InvalidTimeRange { start, end } => Self::InternalError {
                stage: "grid_algebra".to_string(),
                message: format!("Invalid time range: {:?} to {:?}", start, end),
            },
            GridAlgebraError::LaneAssignmentFailed { task_id } => Self::LaneAssignmentFailed { task_id },
        }
    }

    /// Map ReifiedError to specific RoadlineBuilderError variants
    fn from_reified_error(error: ReifiedError) -> Self {
        match error {
            ReifiedError::TaskNotFound { task_id } => Self::TaskNotFoundInReified { task_id },
            ReifiedError::DependencyNotFound { dependency_id } => {
                Self::DependencyNotFoundInReified { dependency_id }
            },
        }
    }
}

/// Builder for creating Roadline representations.
/// 
/// Uses the builder pattern to configure visual parameters and assemble
/// tasks into a complete visual representation.
/// 
/// # Example
/// 
/// ```no_run
/// use roadline_representation_core::roadline::RoadlineBuilder;
/// use roadline_representation_core::reified::{Trim, DownLanePadding, ReifiedUnit};
/// 
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let roadline = RoadlineBuilder::new()
///     .with_trim(Trim::new(ReifiedUnit::new(15)))
///     .with_padding(DownLanePadding::new(ReifiedUnit::new(3)))
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct RoadlineBuilder {
    config: ReifiedConfig,
    graph: Graph,
    root_date: Date,
}

impl Default for RoadlineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RoadlineBuilder {
    /// Create a new builder with sensible defaults.
    /// 
    /// This constructor cannot fail and uses:
    /// - Start date: 2000-01-01 (Y2K as a reasonable epoch)
    /// - Default visual configuration (10 units trim, 2 units padding)
    /// - Empty task graph
    /// 
    /// This is the recommended way to start building a roadline.
    pub fn new() -> Self {
        // Use Y2K as a safe, memorable epoch that won't fail
        use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
        let naive_date = NaiveDate::from_ymd_opt(2000, 1, 1)
            .expect("Y2K date should always be valid");
        let naive_time = NaiveTime::from_hms_opt(0, 0, 0)
            .expect("Midnight should always be valid");
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
        let y2k_date = Date::new(DateTime::from_naive_utc_and_offset(naive_datetime, Utc));
        
        Self {
            config: ReifiedConfig::default_config(),
            graph: Graph::new(),
            root_date: y2k_date,
        }
    }

    /// Create a new builder starting at the Unix epoch.
    /// 
    /// This may fail if the system doesn't support Unix epoch dates.
    /// For an infallible constructor, use `RoadlineBuilder::new()` instead.
    pub fn start_of_epoch() -> Result<Self, RoadlineBuilderError> {
        let root_date = Date::start_of_epoch()
            .map_err(|e| RoadlineBuilderError::DateParsing { 
                message: e.to_string() 
            })?;
            
        Ok(Self {
            config: ReifiedConfig::default_config(),
            graph: Graph::new(),
            root_date,
        })
    }

    /// Create a new builder with a custom start date.
    pub fn with_start_date(start_date: Date) -> Self {
        Self {
            config: ReifiedConfig::default_config(),
            graph: Graph::new(),
            root_date: start_date,
        }
    }

    /// Create a new builder with a date parsed from an ISO string.
    /// 
    /// # Example
    /// ```no_run
    /// # use roadline_representation_core::roadline::RoadlineBuilder;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let builder = RoadlineBuilder::from_iso_date("2024-01-01T00:00:00Z")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_iso_date(iso_string: &str) -> Result<Self, RoadlineBuilderError> {
        use chrono::{DateTime, Utc};
        
        let datetime = DateTime::parse_from_rfc3339(iso_string)
            .map_err(|e| RoadlineBuilderError::DateParsing { 
                message: format!("Failed to parse '{}': {}", iso_string, e)
            })?
            .with_timezone(&Utc);
            
        let date = Date::new(datetime);
        Ok(Self::with_start_date(date))
    }

    /// Create a new builder with a date from year, month, day.
    /// 
    /// # Example
    /// ```no_run
    /// # use roadline_representation_core::roadline::RoadlineBuilder;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let builder = RoadlineBuilder::from_ymd(2024, 3, 15)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Self, RoadlineBuilderError> {
        use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
        
        let naive_date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| RoadlineBuilderError::DateParsing {
                message: format!("Invalid date {}-{:02}-{:02}", year, month, day)
            })?;
        let naive_time = NaiveTime::from_hms_opt(0, 0, 0)
            .expect("Midnight should always be valid");
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);
        let date = Date::new(DateTime::from_naive_utc_and_offset(naive_datetime, Utc));
        
        Ok(Self::with_start_date(date))
    }

    /// Set the connection trim (horizontal gutter space for arrows).
    pub fn with_trim(mut self, trim: Trim) -> Self {
        self.config.connection_trim = trim;
        self
    }

    /// Set the inter-lane padding (vertical spacing between task lanes).
    pub fn with_padding(mut self, padding: DownLanePadding) -> Self {
        self.config.inter_lane_padding = padding;
        self
    }

    /// Set the complete visual configuration.
    pub fn with_config(mut self, config: ReifiedConfig) -> Self {
        self.config = config;
        self
    }

    /// Set trim and padding with convenient unit values.
    /// 
    /// # Example
    /// ```
    /// # use roadline_representation_core::roadline::RoadlineBuilder;
    /// let builder = RoadlineBuilder::new()
    ///     .with_spacing(15, 3); // 15 units trim, 3 units padding
    /// ```
    pub fn with_spacing(mut self, trim_units: u16, padding_units: u16) -> Self {
        self.config.connection_trim = Trim::new(ReifiedUnit::new(trim_units));
        self.config.inter_lane_padding = DownLanePadding::new(ReifiedUnit::new(padding_units));
        self
    }

    /// Use a compact layout configuration (minimal spacing).
    pub fn compact(mut self) -> Self {
        self.config = ReifiedConfig::new(
            Trim::new(ReifiedUnit::new(5)),         // Minimal gutter
            DownLanePadding::new(ReifiedUnit::new(1)) // Minimal padding
        );
        self
    }

    /// Use a spacious layout configuration (generous spacing).
    pub fn spacious(mut self) -> Self {
        self.config = ReifiedConfig::new(
            Trim::new(ReifiedUnit::new(20)),        // Large gutter
            DownLanePadding::new(ReifiedUnit::new(5)) // Large padding
        );
        self
    }

    /// Add a single task to the graph.
    pub fn add_task(&mut self, task: Task) -> Result<&mut Self, RoadlineBuilderError> {
        self.graph.add(task)
            .map_err(RoadlineBuilderError::from_graph_error)?;
        Ok(self)
    }

    /// Add multiple tasks to the graph.
    pub fn add_tasks(&mut self, tasks: impl IntoIterator<Item = Task>) -> Result<&mut Self, RoadlineBuilderError> {
        for task in tasks {
            self.add_task(task)?;
        }
        Ok(self)
    }

    /// Add a task and return a mutable reference for method chaining.
    /// 
    /// # Example
    /// ```no_run
    /// # use roadline_representation_core::roadline::RoadlineBuilder;
    /// # use roadline_util::task::Task;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let task1: Task = todo!();
    /// let roadline = RoadlineBuilder::new()
    ///     .task(task1)?
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn task(mut self, task: Task) -> Result<Self, RoadlineBuilderError> {
        self.graph.add(task)
            .map_err(RoadlineBuilderError::from_graph_error)?;
        Ok(self)
    }

    /// Add multiple tasks in a fluent style.
    /// 
    /// # Example
    /// ```no_run
    /// # use roadline_representation_core::roadline::RoadlineBuilder;
    /// # use roadline_util::task::Task;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let tasks: Vec<Task> = vec![];
    /// let roadline = RoadlineBuilder::new()
    ///     .tasks(tasks)?
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn tasks(mut self, tasks: impl IntoIterator<Item = Task>) -> Result<Self, RoadlineBuilderError> {
        for task in tasks {
            self.graph.add(task)
                .map_err(RoadlineBuilderError::from_graph_error)?;
        }
        Ok(self)
    }

    /// Try to add a task, but continue building even if it fails.
    /// Returns the error for inspection but doesn't fail the builder.
    pub fn try_task(mut self, task: Task) -> (Self, Option<RoadlineBuilderError>) {
        match self.graph.add(task) {
            Ok(()) => (self, None),
            Err(e) => (self, Some(RoadlineBuilderError::from_graph_error(e))),
        }
    }

    /// Get the current number of tasks in the builder.
    pub fn task_count(&self) -> usize {
        self.graph.task_count()
    }

    /// Check if the builder contains any tasks.
    pub fn is_empty(&self) -> bool {
        self.task_count() == 0
    }

    /// Get the current root date.
    pub fn root_date(&self) -> &Date {
        &self.root_date
    }

    /// Get the current visual configuration.
    pub fn config(&self) -> &ReifiedConfig {
        &self.config
    }

    /// Validate the current task graph for common issues.
    /// Returns Ok(()) if the graph looks valid, or the first error found.
    pub fn validate(&self) -> Result<(), RoadlineBuilderError> {
        if self.is_empty() {
            return Err(RoadlineBuilderError::NoTasks);
        }

        // This is a lightweight validation - full validation happens in build()
        // We could add more checks here like cycle detection, but that's expensive
        Ok(())
    }

    /// Get a summary of the current builder state.
    pub fn summary(&self) -> BuilderSummary {
        BuilderSummary {
            task_count: self.task_count(),
            root_date: self.root_date.clone(),
            trim_units: self.config.connection_trim.value().value(),
            padding_units: self.config.inter_lane_padding.value().value(),
        }
    }

    /// Consume the builder and create the final Roadline representation.
    /// 
    /// This orchestrates the entire pipeline:
    /// 1. Graph → RangeAlgebra (temporal positioning)
    /// 2. RangeAlgebra → GridAlgebra (discrete grid placement)  
    /// 3. GridAlgebra → Reified (continuous visual coordinates + connections)
    pub fn build(self) -> Result<Roadline, RoadlineBuilderError> {
        if self.is_empty() {
            return Err(RoadlineBuilderError::NoTasks);
        }

        // Step 1: Build the range algebra (temporal positioning)
        let range_algebra = PreRangeAlgebra::new(self.graph)
            .compute(self.root_date)
            .map_err(RoadlineBuilderError::from_range_algebra_error)?;

        // Step 2: Build the grid algebra (discrete placement)
        let grid_algebra = PreGridAlgebra::new(range_algebra)
            .compute()
            .map_err(RoadlineBuilderError::from_grid_algebra_error)?;

        // Step 3: Reify to visual coordinates (continuous + connections)
        let reified = PreReified::with_config(grid_algebra, self.config)
            .compute()
            .map_err(RoadlineBuilderError::from_reified_error)?;

        Ok(Roadline::new(reified))
    }
}

/// The final Roadline representation containing all visual and connection data.
/// 
/// This is the output of the builder pipeline and provides high-level access
/// to the computed visual layout without exposing internal implementation details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Roadline {
    reified: Reified,
}

impl Roadline {
    /// Internal constructor - use RoadlineBuilder to create instances.
    pub(crate) fn new(reified: Reified) -> Self {
        Self { reified }
    }

    // === High-level Statistics ===

    /// Get the total number of tasks in the roadline.
    pub fn task_count(&self) -> usize {
        self.reified.task_count()
    }

    /// Get the total number of dependency connections.
    pub fn connection_count(&self) -> usize {
        self.reified.connection_count()
    }

    /// Get the maximum visual bounds (width, height) for layout purposes.
    pub fn visual_bounds(&self) -> (ReifiedUnit, ReifiedUnit) {
        self.reified.visual_bounds()
    }

    /// Get the visual configuration used to generate this roadline.
    pub fn config(&self) -> &ReifiedConfig {
        self.reified.config()
    }

    // === Task Access ===

    /// Get the visual bounds for a specific task.
    pub fn get_task_bounds(&self, task_id: &TaskId) -> Option<&DownCell> {
        self.reified.get_down_cell(task_id)
    }

    /// Check if a task exists in the roadline.
    pub fn contains_task(&self, task_id: &TaskId) -> bool {
        self.reified.get_down_cell(task_id).is_some()
    }

    /// Iterate over all tasks and their visual bounds.
    pub fn tasks(&self) -> impl Iterator<Item = (&TaskId, &DownCell)> {
        self.reified.task_bounds()
    }

    /// Get all task IDs in the roadline.
    pub fn task_ids(&self) -> impl Iterator<Item = &TaskId> {
        self.reified.task_bounds().map(|(id, _)| id)
    }

    // === Connection Access ===

    /// Get the visual routing for a specific dependency connection.
    pub fn get_connection(&self, dependency_id: &DependencyId) -> Option<&Joint> {
        self.reified.get_joint(dependency_id)
    }

    /// Check if a dependency connection exists in the roadline.
    pub fn contains_connection(&self, dependency_id: &DependencyId) -> bool {
        self.reified.get_joint(dependency_id).is_some()
    }

    /// Iterate over all connections and their visual routing.
    pub fn connections(&self) -> impl Iterator<Item = (&DependencyId, &Joint)> {
        self.reified.connections()
    }

    /// Get all dependency connection IDs in the roadline.
    pub fn connection_ids(&self) -> impl Iterator<Item = &DependencyId> {
        self.reified.connections().map(|(id, _)| id)
    }

    // === Rendering Helpers ===

    /// Get all task rectangles for rendering.
    /// Returns (task_id, x_start, y_start, x_end, y_end) tuples.
    pub fn task_rectangles(&self) -> impl Iterator<Item = (&TaskId, u16, u16, u16, u16)> {
        self.reified.task_bounds().map(|(id, cell)| {
            let x_start = cell.down_stretch().down_stretch().start().value();
            let x_end = cell.down_stretch().down_stretch().end().value();
            let y_start = cell.down_lane().range().start().value();
            let y_end = cell.down_lane().range().end().value();
            (id, x_start, y_start, x_end, y_end)
        })
    }

    /// Get all Bezier curves for rendering connections.
    /// Returns (dependency_id, start_point, end_point, control1, control2) tuples.
    pub fn bezier_curves(&self) -> impl Iterator<Item = (&DependencyId, &ConnectionPoint, &ConnectionPoint, &ConnectionPoint, &ConnectionPoint)> {
        self.reified.connections().map(|(id, joint)| {
            let bezier = joint.bezier_connection();
            (id, &bezier.start, &bezier.end, &bezier.control1, &bezier.control2)
        })
    }

    /// Get connection endpoints for rendering arrows.
    /// Returns (dependency_id, start_point, end_point) tuples.
    pub fn connection_endpoints(&self) -> impl Iterator<Item = (&DependencyId, &ConnectionPoint, &ConnectionPoint)> {
        self.reified.connections().map(|(id, joint)| {
            let bezier = joint.bezier_connection();
            (id, &bezier.start, &bezier.end)
        })
    }

    // === Layer Access (for advanced usage) ===

    /// Get direct access to the underlying reified representation.
    /// 
    /// This provides full access to the internal data structures but
    /// breaks the abstraction. Use the high-level methods when possible.
    pub fn reified(&self) -> &Reified {
        &self.reified
    }

    /// Get access to the underlying grid algebra layer.
    pub fn grid_algebra(&self) -> &GridAlgebra {
        self.reified.grid()
    }

    /// Get access to the underlying range algebra layer.
    pub fn range_algebra(&self) -> &RangeAlgebra {
        self.reified.grid().range_algebra()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roadline_util::task::{Task, range::{Start, End, PointOfReference, TargetDate}};
    use roadline_util::duration::Duration;
    use std::collections::BTreeSet;
    use std::time::Duration as StdDuration;

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
    fn test_builder_basic_functionality() -> Result<(), anyhow::Error> {
        let mut builder = RoadlineBuilder::start_of_epoch()?;
        
        let task1 = create_test_task_with_dependencies(1, 1, 0, 10 * 24 * 60 * 60, BTreeSet::new())?;
        let task2 = create_test_task_with_dependencies(2, 1, 5 * 24 * 60 * 60, 10 * 24 * 60 * 60, BTreeSet::from_iter([1]))?;
        
        builder.add_task(task1)?;
        builder.add_task(task2)?;
        
        let roadline = builder.build()?;
        
        assert_eq!(roadline.task_count(), 2);
        assert_eq!(roadline.connection_count(), 1);
        
        Ok(())
    }

    #[test]
    fn test_builder_no_tasks_error() {
        let builder = RoadlineBuilder::start_of_epoch().unwrap();
        let result = builder.build();
        
        assert!(matches!(result, Err(RoadlineBuilderError::NoTasks)));
    }

    #[test]
    fn test_infallible_constructor() {
        // This should never panic
        let builder = RoadlineBuilder::new();
        assert_eq!(builder.task_count(), 0);
        assert!(builder.is_empty());
        
        // Default should work the same way
        let default_builder = RoadlineBuilder::default();
        assert_eq!(default_builder.task_count(), 0);
    }

    #[test]
    fn test_fluent_api() -> Result<(), anyhow::Error> {
        let task1 = create_test_task_with_dependencies(1, 1, 0, 10 * 24 * 60 * 60, BTreeSet::new())?;
        let task2 = create_test_task_with_dependencies(2, 1, 5 * 24 * 60 * 60, 10 * 24 * 60 * 60, BTreeSet::from_iter([1]))?;
        
        let roadline = RoadlineBuilder::new()
            .compact()
            .task(task1)?
            .task(task2)?
            .build()?;
        
        assert_eq!(roadline.task_count(), 2);
        assert_eq!(roadline.config().connection_trim.value().value(), 5); // Compact config
        assert_eq!(roadline.config().inter_lane_padding.value().value(), 1);
        
        Ok(())
    }

    #[test] 
    fn test_date_constructors() -> Result<(), anyhow::Error> {
        // Test YMD constructor
        let builder1 = RoadlineBuilder::from_ymd(2024, 1, 1)?;
        assert!(!builder1.is_empty() || builder1.is_empty()); // Just verify it doesn't crash
        
        // Test ISO date constructor
        let builder2 = RoadlineBuilder::from_iso_date("2024-03-15T10:30:00Z")?;
        assert!(!builder2.is_empty() || builder2.is_empty()); // Just verify it doesn't crash
        
        // Test invalid date
        assert!(RoadlineBuilder::from_ymd(2024, 13, 1).is_err());
        assert!(RoadlineBuilder::from_iso_date("invalid-date").is_err());
        
        Ok(())
    }

    #[test]
    fn test_spacing_presets() -> Result<(), anyhow::Error> {
        let task1 = create_test_task_with_dependencies(1, 1, 0, 10 * 24 * 60 * 60, BTreeSet::new())?;
        let task2 = create_test_task_with_dependencies(2, 1, 5 * 24 * 60 * 60, 10 * 24 * 60 * 60, BTreeSet::from_iter([1]))?;
        
        // Test compact
        let compact_roadline = RoadlineBuilder::new()
            .compact()
            .task(task1.clone())?
            .task(task2.clone())?
            .build()?;
        assert_eq!(compact_roadline.config().connection_trim.value().value(), 5);
        assert_eq!(compact_roadline.config().inter_lane_padding.value().value(), 1);
        
        // Test spacious
        let spacious_roadline = RoadlineBuilder::new()
            .spacious()
            .task(task1)?
            .task(task2)?
            .build()?;
        assert_eq!(spacious_roadline.config().connection_trim.value().value(), 20);
        assert_eq!(spacious_roadline.config().inter_lane_padding.value().value(), 5);
        
        Ok(())
    }

    #[test]
    fn test_builder_summary() {
        let builder = RoadlineBuilder::new()
            .with_spacing(15, 3);
            
        let summary = builder.summary();
        assert_eq!(summary.task_count, 0);
        assert_eq!(summary.trim_units, 15);
        assert_eq!(summary.padding_units, 3);
        
        // Test display
        let display_str = format!("{}", summary);
        assert!(display_str.contains("0 tasks"));
        assert!(display_str.contains("15×3"));
    }

    #[test]
    fn test_validation() {
        let empty_builder = RoadlineBuilder::new();
        assert!(matches!(empty_builder.validate(), Err(RoadlineBuilderError::NoTasks)));
        
        // With tasks, validation should pass
        let mut builder_with_tasks = RoadlineBuilder::new();
        let task = create_test_task_with_dependencies(1, 1, 0, 10 * 24 * 60 * 60, BTreeSet::new()).unwrap();
        builder_with_tasks.add_task(task).unwrap();
        assert!(builder_with_tasks.validate().is_ok());
    }

    #[test]
    fn test_builder_custom_config() -> Result<(), anyhow::Error> {
        let mut builder = RoadlineBuilder::start_of_epoch()?
            .with_trim(Trim::new(ReifiedUnit::new(20)))
            .with_padding(DownLanePadding::new(ReifiedUnit::new(5)));
        
        let task1 = create_test_task_with_dependencies(1, 1, 0, 10 * 24 * 60 * 60, BTreeSet::new())?;
        let task2 = create_test_task_with_dependencies(2, 1, 5 * 24 * 60 * 60, 10 * 24 * 60 * 60, BTreeSet::from_iter([1]))?;
        
        builder.add_task(task1)?;
        builder.add_task(task2)?;
        
        let roadline = builder.build()?;
        
        assert_eq!(roadline.config().connection_trim.value().value(), 20);
        assert_eq!(roadline.config().inter_lane_padding.value().value(), 5);
        
        Ok(())
    }

    #[test]
    fn test_roadline_access_methods() -> Result<(), anyhow::Error> {
        let mut builder = RoadlineBuilder::start_of_epoch()?;
        
        let task1 = create_test_task_with_dependencies(1, 1, 0, 10 * 24 * 60 * 60, BTreeSet::new())?;
        let task2 = create_test_task_with_dependencies(2, 1, 5 * 24 * 60 * 60, 10 * 24 * 60 * 60, BTreeSet::from_iter([1]))?;
        
        builder.add_task(task1)?;
        builder.add_task(task2)?;
        
        let roadline = builder.build()?;
        
        // Test task access
        assert!(roadline.contains_task(&TaskId::new(1)));
        assert!(roadline.contains_task(&TaskId::new(2)));
        assert!(!roadline.contains_task(&TaskId::new(3)));
        
        assert!(roadline.get_task_bounds(&TaskId::new(1)).is_some());
        assert!(roadline.get_task_bounds(&TaskId::new(3)).is_none());
        
        // Test iteration
        let task_ids: Vec<_> = roadline.task_ids().cloned().collect();
        assert_eq!(task_ids.len(), 2);
        assert!(task_ids.contains(&TaskId::new(1)));
        assert!(task_ids.contains(&TaskId::new(2)));
        
        // Test visual bounds
        let (max_x, max_y) = roadline.visual_bounds();
        assert!(max_x.value() > 0);
        assert!(max_y.value() > 0);
        
        // Test rendering helpers
        let rectangles: Vec<_> = roadline.task_rectangles().collect();
        assert_eq!(rectangles.len(), 2);
        
        let curves: Vec<_> = roadline.bezier_curves().collect();
        assert_eq!(curves.len(), 1);
        
        Ok(())
    }
}