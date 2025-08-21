use crate::reified::down_stretch::Trim;
use crate::reified::down_lane::DownLanePadding;
use crate::reified::reified_unit::ReifiedUnit;
use crate::graph::{Graph, GraphError};
use roadline_util::task::Task;
use crate::reified::Reified;
use crate::range_algebra::PreRangeAlgebra;
use crate::grid_algebra::PreGridAlgebra;
use crate::reified::PreReified;
use crate::range_algebra::Date;


#[derive(Debug, thiserror::Error)]
pub enum RoadlineBuilderError {
    #[error("Internal error: {0}")]
    Internal(String),
}

pub struct RoadlineBuilder {
    trim: Trim,
    padding: DownLanePadding,
    graph: Graph,
    root_date: Date,
}


impl RoadlineBuilder {

    pub fn start_of_epoch() -> Result<Self, RoadlineBuilderError> {
        Ok(Self {
            trim: Trim::new(ReifiedUnit::new(2)),
            padding: DownLanePadding::new(ReifiedUnit::new(1)),
            graph: Graph::new(),
            root_date: Date::start_of_epoch().map_err(|e| RoadlineBuilderError::Internal(e.to_string()))?,
        })
    }

    pub fn with_trim(mut self, trim: Trim) -> Self {
        self.trim = trim;
        self
    }

    pub fn with_padding(mut self, padding: DownLanePadding) -> Self {
        self.padding = padding;
        self
    }

    pub fn add_task(&mut self, task: Task) -> Result<(), GraphError> {
        self.graph.add(task)
    }

    pub fn add_tasks(&mut self, tasks: impl IntoIterator<Item = Task>) -> Result<(), GraphError> {
        for task in tasks {
            self.add_task(task)?;
        }
        Ok(())
    }

    pub fn build(self) -> Result<Roadline, RoadlineBuilderError> {
        
        // Build the range algebra
        let pre_range_algebra = PreRangeAlgebra::new(self.graph);
        let range_algebra = pre_range_algebra.compute(self.root_date).map_err(|e| RoadlineBuilderError::Internal(e.to_string()))?;

        // Build the grid algebra
        let pre_grid_algebra = PreGridAlgebra::new(range_algebra);
        let grid_algebra = pre_grid_algebra.compute().map_err(|e| RoadlineBuilderError::Internal(e.to_string()))?;

        // Reify the grid algebra
        let pre_reified = PreReified::new(grid_algebra);
        let reified = pre_reified.compute().map_err(|e| RoadlineBuilderError::Internal(e.to_string()))?;

        Ok(Roadline::new(reified))
    }
}

pub struct Roadline {
    reified: Reified,
}

impl Roadline {
    pub fn new(reified: Reified) -> Self {
        Self { reified }
    }
}