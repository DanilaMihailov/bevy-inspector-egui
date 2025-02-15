use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

#[derive(Inspectable, Default)]
struct Data {
    #[inspectable(min = 10.0, max = 70.0, suffix = "pt")]
    font_size: f32,
    #[inspectable(label = "Display Square")]
    show_square: bool,
    color: Color,
    #[inspectable(visual, min = Vec2::new(-200., -200.), max = Vec2::new(200., 200.))]
    position: Vec2,
    #[inspectable(min = 42.0, max = 100.0)] // attributes get passed to each child
    list: [f32; 2],
    custom_enum: CustomEnum,
    vector: Vec<String>,
    #[inspectable(min = Vec3::ZERO, max = Vec3::splat(128.0))]
    vec3: Vec3,
    text: String,
    transform: Transform,
    #[inspectable(collapse)]
    noise_settings: NoiseSettings,
    #[allow(unused)]
    #[inspectable(ignore)]
    non_inspectable: NonInspectable,
}

#[derive(Inspectable)]
enum CustomEnum {
    A,
    B,
    C,
}
impl Default for CustomEnum {
    fn default() -> Self {
        CustomEnum::A
    }
}

#[derive(Inspectable, Default)]
struct NoiseSettings {
    #[inspectable(max = 8)]
    octaves: u8,
    frequency: f32,
    lacunarity: f32,
    persistence: f32,
}

#[derive(Default)]
struct NonInspectable;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<Data>::new())
        .run();
}
