#[cfg(test)]
pub mod tests {

	use super::super::super::TaskSpawner;
	use bevy::prelude::*;
	use roadline_util::task::Id as TaskId;

	#[test]
	fn test_task_spawner_creation() -> Result<(), Box<dyn std::error::Error>> {
		let task_id = TaskId::from(1);
		let position = Vec3::new(100.0, 200.0, 0.0);
		let size = Vec2::new(200.0, 75.0);
		let title = "Test Task".to_string();

		let spawner = TaskSpawner::new(task_id, position, size, title.clone(), false, 3, 3);

		assert_eq!(spawner.data.task_id, task_id);
		assert_eq!(spawner.data.position, position);
		assert_eq!(spawner.data.size, size);
		assert_eq!(spawner.data.title, title);
		assert_eq!(spawner.data.font_size, 6.0);
		assert_eq!(spawner.data.completed, 3);
		assert_eq!(spawner.data.total, 3);

		Ok(())
	}

	#[test]
	fn test_task_spawner_with_font_size() -> Result<(), Box<dyn std::error::Error>> {
		let task_id = TaskId::from(1);
		let position = Vec3::new(100.0, 200.0, 0.0);
		let size = Vec2::new(200.0, 75.0);
		let title = "Test Task".to_string();

		let spawner =
			TaskSpawner::new(task_id, position, size, title, false, 3, 3).with_font_size(12.0);

		assert_eq!(spawner.data.font_size, 12.0);

		Ok(())
	}
}
