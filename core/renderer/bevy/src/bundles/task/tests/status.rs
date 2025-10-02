#[cfg(test)]
pub mod tests {

	use crate::bundles::task::tests::utils::{setup_task_test_app, TestTasksParams};
	use bevy::ecs::system::RunSystemOnce;
	use bevy::prelude::*;
	use roadline_util::task::Id as TaskId;

	#[test]
	fn test_task_spawning_with_different_statuses() -> Result<(), Box<dyn std::error::Error>> {
		let mut app1 = setup_task_test_app();

		// Spawn NotStarted task
		app1.world_mut().run_system_once(
			TestTasksParams::new()
				.with_status_task(
					TaskId::from(1),
					Vec3::new(100.0, 100.0, 0.0),
					Vec2::new(200.0, 75.0),
					"Not Started Task".to_string(),
					true,
					0,
					3,
				)
				.build(),
		)?;

		let world1 = app1.world_mut();
		let mut not_started_query =
			world1
				.query::<&crate::bundles::task::content::status::not_started::NotStartedStatusMarker>(
				);
		let not_started_markers: Vec<_> = not_started_query.iter(world1).collect();
		assert_eq!(
			not_started_markers.len(),
			1,
			"Should spawn NotStartedStatusMarker for not started task"
		);

		let mut app2 = setup_task_test_app();

		// Spawn InProgress task
		app2.world_mut().run_system_once(
			TestTasksParams::new()
				.with_status_task(
					TaskId::from(2),
					Vec3::new(200.0, 200.0, 0.0),
					Vec2::new(275.0, 60.0),
					"InProgress Task".to_string(),
					true,
					1,
					3,
				)
				.build(),
		)?;

		let world2 = app2.world_mut();
		let mut in_progress_query =
			world2
				.query::<&crate::bundles::task::content::status::in_progress::InProgressStatusMarker>(
				);
		let in_progress_markers: Vec<_> = in_progress_query.iter(world2).collect();
		assert_eq!(
			in_progress_markers.len(),
			1,
			"Should spawn InProgressStatusMarker for in progress task"
		);

		// Test completed status
		let mut app3 = setup_task_test_app();

		// Spawn Completed task
		app3.world_mut().run_system_once(
			TestTasksParams::new()
				.with_status_task(
					TaskId::from(3),
					Vec3::new(300.0, 300.0, 0.0),
					Vec2::new(300.0, 70.0),
					"Completed Task".to_string(),
					false,
					3,
					3,
				)
				.build(),
		)?;

		let world3 = app3.world_mut();
		let mut completed_query =
			world3
				.query::<&crate::bundles::task::content::status::completed::CompletedStatusMarker>(
				);
		let completed_markers: Vec<_> = completed_query.iter(world3).collect();
		assert_eq!(
			completed_markers.len(),
			1,
			"Should spawn CompletedStatusMarker for completed task"
		);

		let mut check_mark_query =
			world3.query::<&crate::bundles::task::content::status::completed::CheckMarkMesh>();
		let check_marks: Vec<_> = check_mark_query.iter(world3).collect();
		assert_eq!(check_marks.len(), 1, "Should spawn CheckMarkMesh for completed task");

		// Test missed status
		let mut app4 = setup_task_test_app();

		// Spawn Missed task
		app4.world_mut().run_system_once(
			TestTasksParams::new()
				.with_status_task(
					TaskId::from(4),
					Vec3::new(400.0, 400.0, 0.0),
					Vec2::new(400.0, 80.0),
					"Missed Task".to_string(),
					false,
					0,
					3,
				)
				.build(),
		)?;

		let world4 = app4.world_mut();
		let mut missed_query =
			world4.query::<&crate::bundles::task::content::status::missed::MissedStatusMarker>();
		let missed_markers: Vec<_> = missed_query.iter(world4).collect();
		assert_eq!(missed_markers.len(), 1, "Should spawn MissedStatusMarker for missed task");

		Ok(())
	}
}
