use roadline_representation_core::roadline::{Roadline as CoreRoadline, RoadlineBuilder};
use roadline_util::task::Task;
use std::time::Duration as StdDuration;

pub fn create_test_roadline() -> Result<CoreRoadline, anyhow::Error> {
	// First line of tasks
	let task1 = Task::new_test().with_id(1.into()).for_standard_duration(
		// 3 days
		StdDuration::from_secs(3 * 24 * 60 * 60),
	);
	let task2 = Task::new_test()
		.with_id(2.into())
		.with_dependencies([*task1.id()])
		.after(&task1)
		.for_standard_duration(
			// 5 days
			StdDuration::from_secs(5 * 24 * 60 * 60),
		);
	let task3 = Task::new_test()
		.with_id(3.into())
		.with_dependencies([*task2.id()])
		.after(&task2)
		.for_standard_duration(
			// 10 days
			StdDuration::from_secs(10 * 24 * 60 * 60),
		);

	// Add a task that depends on the first task and stretches ten 10 days
	let task4 = Task::new_test()
		.with_id(4.into())
		.with_dependencies([*task1.id()])
		.after(&task1)
		.for_standard_duration(
			// 10 days
			StdDuration::from_secs(10 * 24 * 60 * 60),
		);

	// Add a task that depends on the first task and stretches ten 7 days

	let task5 = Task::new_test()
		.with_id(5.into())
		.with_dependencies([*task1.id()])
		.after(&task1)
		.for_standard_duration(
			// 7 days
			StdDuration::from_secs(7 * 24 * 60 * 60),
		);

	let rodline = RoadlineBuilder::new()
		.task(task1)?
		.task(task2)?
		.task(task3)?
		.task(task4)?
		.task(task5)?
		.build()?;

	Ok(rodline)
}
