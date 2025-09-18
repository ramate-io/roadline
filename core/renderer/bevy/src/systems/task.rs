use crate::bundles::task::content;
use crate::bundles::task::content::status;
use crate::bundles::task::content::title;
use crate::bundles::TaskBundler;
use crate::components::Task;
use crate::resources::{RenderUpdateEvent, Roadline};
use crate::RoadlineRenderConfig;
use bevy::prelude::*;
use bevy::ui::GridPlacement;

/// Configuration for task systems
pub struct TaskSystemConfig;

impl TaskSystemConfig {
	/// Builds an owned closure for updating task sprites
	pub fn build() -> impl FnMut(
		Commands,
		EventReader<RenderUpdateEvent>,
		Option<Res<Roadline>>,
		Res<RoadlineRenderConfig>,
		Query<Entity, With<Task>>,
	) {
		move |mut commands: Commands,
		      render_events: EventReader<RenderUpdateEvent>,
		      reified_opt: Option<Res<Roadline>>,
		      _config: Res<RoadlineRenderConfig>,
		      existing_tasks: Query<Entity, With<Task>>| {
			// Only update if we received a render event and have reified data
			if render_events.is_empty() || reified_opt.is_none() {
				return;
			}
			// Events are automatically cleared after being read

			let reified = reified_opt.unwrap();

			// Clear existing task entities
			for entity in existing_tasks.iter() {
				commands.entity(entity).despawn();
			}

			// Get the visual bounds to scale everything properly
			let (max_width, max_height) = reified.visual_bounds();
			let max_width_f32 = max_width.value() as f32;
			let max_height_f32 = max_height.value() as f32;

			// Scale factor: try a much larger scale to see if that helps with visibility
			let pixels_per_unit = 50.0;

			// Calculate offsets to center the content around (0,0)
			let content_width_pixels = max_width_f32 * pixels_per_unit;
			let content_height_pixels = max_height_f32 * pixels_per_unit;
			let offset_x = -content_width_pixels / 2.0;
			let offset_y = -content_height_pixels / 2.0;

			// Create new task sprites for each task
			for (task_id, start_x, start_y, end_x, end_y) in reified.task_rectangles() {
				println!(
					"task_id: {:?}, start_x: {}, start_y: {}, end_x: {}, end_y: {}",
					task_id, start_x, start_y, end_x, end_y
				);
				println!("Max bounds: width={}, height={}", max_width_f32, max_height_f32);

				let (x, y) = (start_x, start_y);
				let height = end_y - start_y;
				let width = end_x - start_x;

				// Convert reified units to pixel coordinates using proper scaling
				let pixel_x = x as f32 * pixels_per_unit + offset_x;
				let pixel_y = y as f32 * pixels_per_unit + offset_y;
				let sprite_width = width as f32 * pixels_per_unit;
				let sprite_height = height as f32 * pixels_per_unit;

				// Adjust for left justification (Bevy positions by center, so move right by half width)
				let left_justified_x = pixel_x + (sprite_width / 2.0);

				println!(
				"Rendering: pixel_pos=({:.1}, {:.1}), size=({:.1}x{:.1}), left_justified_x={:.1}",
				pixel_x, pixel_y, sprite_width, sprite_height, left_justified_x
			);

				let task = reified.task(task_id);
				if task.is_none() {
					continue;
				}
				let task = task.unwrap();
				let title = task.title();

				// Use TaskBundler to spawn all task entities
				let task_bundler = TaskBundler::new(
					*task_id,
					Vec3::new(left_justified_x, pixel_y, 0.0),
					Vec2::new(sprite_width, sprite_height),
					title.text.clone(),
				);

				commands.spawn(task_bundler.pre_bundle().bundle());
			}
		}
	}
}
