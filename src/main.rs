use bevy::{
    prelude::*,
    render::camera::ScalingMode
};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Game".to_string(),
                width: height * RESOLUTION,
                height: height,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .run();
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(0.15, 0.15)),
            ..default()
        },
        texture: asset_server.load("bugboi.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 900.0),
            ..default()
        },
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;

    camera.projection.right = 1.0 * RESOLUTION;
    camera.projection.left = -1.0 * RESOLUTION;

    camera.projection.scaling_mode = ScalingMode::None;

    commands.spawn(camera);
}

#[derive(Resource)]
struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("Ascii.png");
    let atlas =
        TextureAtlas::from_grid(image, Vec2::splat(9.0), 16, 16, Some(Vec2::splat(2.0)), None);

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(AsciiSheet(atlas_handle));
}