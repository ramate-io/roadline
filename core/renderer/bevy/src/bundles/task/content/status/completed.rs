use bevy::prelude::*;
use bevy::render::mesh::Mesh2d;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
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
	) {
		// Create the check mark mesh
		let check_mark_mesh = self.create_check_mark_mesh();
		let mesh_handle = meshes.add(check_mark_mesh);
		let material_handle = materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.2)));

		let status_entity = commands
			.spawn((
				CompletedStatusMarker,
				Node {
					display: Display::Flex,
					align_items: AlignItems::Center,
					justify_content: JustifyContent::Center,
					justify_self: JustifySelf::End,
					align_self: AlignSelf::Center,
					..default()
				},
				Text::new(format!("{}/{}", self.completed, self.total)),
				TextColor(Color::oklch(0.40, 0.08, 149.0)),
				TextFont { font_size: 8.0, ..Default::default() },
				BorderRadius::all(Val::Px(16.0)),
			))
			.id();

		// Spawn the check mark mesh as a child
		let check_mark_entity = commands
			.spawn((
				CheckMarkMesh,
				Mesh2d(mesh_handle),
				MeshMaterial2d(material_handle),
				Transform::from_scale(Vec3::splat(8.0)), // Scale up the mesh
				Visibility::Visible,
			))
			.id();

		// Attach check mark to status
		commands.entity(status_entity).add_child(check_mark_entity);

		// Attach status to parent
		commands.entity(parent).add_child(status_entity);
	}

	/// Creates a stylized check mark mesh
	fn create_check_mark_mesh(&self) -> Mesh {
		let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());

		// Define check mark vertices - creating a stylized check mark shape
		let vertices = vec![
			// Left arm of check mark (going up-right)
			[-0.4, -0.1, 0.0], // Bottom left
			[-0.2, 0.1, 0.0],  // Top left
			[-0.1, 0.0, 0.0],  // Middle left
			// Right arm of check mark (going down-right)
			[-0.1, 0.0, 0.0], // Middle left
			[0.1, -0.2, 0.0], // Bottom right
			[0.3, -0.4, 0.0], // Far right
			// Add some thickness to make it look more solid
			[-0.35, -0.15, 0.0], // Thickened bottom left
			[-0.15, 0.15, 0.0],  // Thickened top left
			[0.15, -0.15, 0.0],  // Thickened bottom right
			[0.35, -0.35, 0.0],  // Thickened far right
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
