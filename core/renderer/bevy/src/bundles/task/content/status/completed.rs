use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::view::RenderLayers;
use bevy::sprite::{ColorMaterial, MeshMaterial2d};
use bevy::ui::{Node, Val};

#[derive(Component)]
pub struct CompletedStatusMarker;

#[derive(Component)]
pub struct CheckMarkMesh;

pub struct CompletedStatusSpawner {
	pub completed: u32,
	pub total: u32,
}

impl CompletedStatusSpawner {
	pub fn new(completed: u32, total: u32) -> Self {
		Self { completed, total }
	}

	pub fn spawn(
		self,
		commands: &mut Commands,
		meshes: &mut ResMut<Assets<Mesh>>,
		materials: &mut ResMut<Assets<ColorMaterial>>,
		parent: Entity,
		world_position: Vec3,
		task_size: Vec2,
	) {
		// Create the check mark mesh
		let check_mark_mesh = self.create_check_mark_mesh();
		let mesh_handle = meshes.add(check_mark_mesh);
		let material_handle = materials.add(ColorMaterial::from(Color::oklch(0.40, 0.08, 149.0)));

		let status_entity = commands
			.spawn((
				CompletedStatusMarker,
				Node {
					display: Display::Flex,
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					justify_self: JustifySelf::Start,
					align_self: AlignSelf::Center,
					..default()
				},
				Text::new(format!("{}/{}", self.completed, self.total)),
				TextColor(Color::oklch(0.40, 0.08, 149.0)),
				TextFont { font_size: 8.0, ..Default::default() },
				BorderRadius::all(Val::Px(16.0)),
			))
			.id();

		// Position the check mark at the right side of the task box
		// Offset by half the task width to get to the right edge
		let task_width_offset = task_size.x / 2.0;
		let check_mark_position = world_position + Vec3::new(task_width_offset - 20.0, 0.0, 0.1);
		println!(
			"Spawning check mark mesh at world_position: {:?}, final_position: {:?}",
			world_position, check_mark_position
		);

		// Spawn the check mark mesh at the same world position as the UI node
		let _check_mark_entity = commands
			.spawn((
				CheckMarkMesh,
				Mesh2d(mesh_handle),
				MeshMaterial2d(material_handle),
				Transform::from_translation(check_mark_position).with_scale(Vec3::splat(20.0)), // Scale up the mesh to match test triangle
				Visibility::Visible,
				RenderLayers::layer(2),
			))
			.id();

		// Attach status to parent
		commands.entity(parent).add_child(status_entity);
	}

	/// Creates a stylized check mark mesh
	fn create_check_mark_mesh(&self) -> Mesh {
		let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());

		// Define check mark vertices - creating a stylized check mark shape (flipped)
		let vertices = vec![
			// Left arm of check mark (going up-right) - flipped y coordinates
			[-0.4, 0.1, 0.0],  // Bottom left (was -0.1)
			[-0.2, -0.1, 0.0], // Top left (was 0.1)
			[-0.1, 0.0, 0.0],  // Middle left (unchanged)
			// Right arm of check mark (going down-right) - flipped y coordinates
			[-0.1, 0.0, 0.0], // Middle left (unchanged)
			[0.1, 0.2, 0.0],  // Bottom right (was -0.2)
			[0.3, 0.4, 0.0],  // Far right (was -0.4)
			// Add some thickness to make it look more solid - flipped y coordinates
			[-0.35, 0.15, 0.0],  // Thickened bottom left (was -0.15)
			[-0.15, -0.15, 0.0], // Thickened top left (was 0.15)
			[0.15, 0.15, 0.0],   // Thickened bottom right (was -0.15)
			[0.35, 0.35, 0.0],   // Thickened far right (was -0.35)
		];

		// Define triangles to create the check mark shape
		let indices = vec![
			// Left arm triangles
			0, 1, 2, 0, 2, 6, 6, 2, 7, 2, 1, 7, // Right arm triangles
			2, 3, 4, 2, 4, 8, 8, 4, 9, 4, 3, 9, // Connection triangles
			2, 3, 7, 7, 3, 8,
		];

		mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
		mesh.insert_indices(Indices::U32(indices));

		mesh
	}
}
