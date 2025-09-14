use crate::components::{MilestoneNode, RenderState, TaskEdge};
use crate::resources::{RenderUpdateEvent, Roadline};
use crate::RoadlineRenderConfig;
use bevy::color::palettes::css::WHITE_SMOKE;
use bevy::prelude::*;

/// System to update milestone sprites when reified data changes
pub fn update_milestone_sprites(
	mut commands: Commands,
	render_events: EventReader<RenderUpdateEvent>,
	reified_opt: Option<Res<Roadline>>,
	config: Res<RoadlineRenderConfig>,
	existing_milestones: Query<Entity, With<MilestoneNode>>,
) {
	// Only update if we received a render event and have reified data
	if render_events.is_empty() || reified_opt.is_none() {
		return;
	}
	// Events are automatically cleared after being read

	let reified = reified_opt.unwrap();

	// Clear existing milestone entities
	for entity in existing_milestones.iter() {
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

	// Create new milestone sprites for each task
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

		// Spawn the background with border (black border)
		commands.spawn((
			Sprite {
				color: Color::BLACK,
				custom_size: Some(Vec2::new(sprite_width + 2.0, sprite_height + 2.0)), // Slightly larger for border
				..default()
			},
			Transform::from_xyz(left_justified_x, pixel_y, 1.0),
			Visibility::Visible,
		));

		// Spawn the inner background (white)
		commands.spawn((
			Sprite {
				color: WHITE_SMOKE.into(),
				custom_size: Some(Vec2::new(sprite_width, sprite_height)),
				..default()
			},
			Transform::from_xyz(left_justified_x, pixel_y, 1.1), // Slightly higher z to appear on top of border
			Visibility::Visible,
			MilestoneNode::new(*task_id),
			RenderState::new(),
		));

		// Spawn the text within the sprite bounds
		commands.spawn((
			Text2d::new(title.text.clone()),
			TextFont { font_size: 6.0, ..default() },
			TextColor(Color::BLACK),
			Transform::from_xyz(left_justified_x, pixel_y, 2.0), // Higher z-index to appear on top
			Visibility::Visible,
		));
	}
}

/// System to update edge renderers when reified data changes
pub fn update_edge_renderers(
	mut _commands: Commands,
	_render_events: EventReader<RenderUpdateEvent>,
	_reified_opt: Option<Res<Roadline>>,
	_config: Res<RoadlineRenderConfig>,
	_existing_edges: Query<Entity, With<TaskEdge>>,
) {
}
