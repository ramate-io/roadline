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

	// Create new milestone sprites for each task
	for (task_id, start_x, start_y, end_x, end_y) in reified.task_rectangles() {
		println!(
			"task_id: {:?}, start_x: {}, start_y: {}, end_x: {}, end_y: {}",
			task_id, start_x, start_y, end_x, end_y
		);
		let (x, y) = (start_x, start_y);
		let height = end_y - start_y;
		let width = end_x - start_x;

		// Convert reified units to pixel coordinates
		let pixel_x = x as f32 * 20.423;
		let pixel_y = y as f32 * 20.423;
		let sprite_width = width as f32 * 20.423;
		let sprite_height = height as f32 * 20.423;

		// Adjust for left justification (Bevy positions by center, so move right by half width)
		let left_justified_x = pixel_x + (sprite_width / 2.0);

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
			TextFont { font_size: 12.0, ..default() },
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
