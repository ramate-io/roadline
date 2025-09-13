use crate::components::{MilestoneNode, RenderState, TaskEdge};
use crate::resources::{ReifiedData, RenderUpdateEvent};
use crate::RoadlineRenderConfig;
use bevy::prelude::*;

/// System to update milestone sprites when reified data changes
pub fn update_milestone_sprites(
	mut commands: Commands,
	render_events: EventReader<RenderUpdateEvent>,
	reified_opt: Option<Res<ReifiedData>>,
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
	for (task_id, down_cell) in reified.task_bounds() {
		let (x, y) = down_cell.outgoing_connection_point();

		// Convert reified units to pixel coordinates
		let pixel_x = x.value() as f32 * config.unit_to_pixel_scale;
		let pixel_y = y.value() as f32 * config.unit_to_pixel_scale;

		commands.spawn((
			Sprite {
				color: config.milestone_color,
				custom_size: Some(Vec2::new(
					config.milestone_radius * 2.0,
					config.milestone_radius * 2.0,
				)),
				..default()
			},
			Transform::from_xyz(pixel_x, pixel_y, 1.0),
			Visibility::Visible,
			MilestoneNode::new(*task_id),
			RenderState::new(),
		));
	}
}

/// System to update edge renderers when reified data changes
pub fn update_edge_renderers(
	mut commands: Commands,
	render_events: EventReader<RenderUpdateEvent>,
	reified_opt: Option<Res<ReifiedData>>,
	config: Res<RoadlineRenderConfig>,
	existing_edges: Query<Entity, With<TaskEdge>>,
) {
	// Only update if we received a render event and have reified data
	if render_events.is_empty() || reified_opt.is_none() {
		return;
	}

	let reified = reified_opt.unwrap();

	// Clear existing edge entities
	for entity in existing_edges.iter() {
		commands.entity(entity).despawn();
	}

	// Create new edge renderers for each connection
	for (dependency_id, joint) in reified.connections() {
		let bezier = joint.bezier_connection();

		// For now, render as a simple line from start to end
		// TODO: Implement proper Bezier curve rendering
		let start_x = bezier.start.x.value() as f32 * config.unit_to_pixel_scale;
		let start_y = bezier.start.y.value() as f32 * config.unit_to_pixel_scale;
		let end_x = bezier.end.x.value() as f32 * config.unit_to_pixel_scale;
		let end_y = bezier.end.y.value() as f32 * config.unit_to_pixel_scale;

		// Calculate line properties
		let center_x = (start_x + end_x) / 2.0;
		let center_y = (start_y + end_y) / 2.0;
		let length = ((end_x - start_x).powi(2) + (end_y - start_y).powi(2)).sqrt();
		let angle = (end_y - start_y).atan2(end_x - start_x);

		commands.spawn((
			Sprite {
				color: config.edge_color,
				custom_size: Some(Vec2::new(length, config.edge_thickness)),
				..default()
			},
			Transform::from_xyz(center_x, center_y, 0.0)
				.with_rotation(Quat::from_rotation_z(angle)),
			Visibility::Visible,
			TaskEdge::new(*dependency_id),
			RenderState::new(),
		));
	}
}
