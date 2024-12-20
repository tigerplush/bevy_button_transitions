use bevy::prelude::*;
use bevy_button_transitions::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ButtonTransitionsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_interactability)
        .run();
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((Name::from("UiRoot"), Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }))
        .with_children(|ui_root| {
            ui_root.spawn((
                ButtonTransition::ColorTint(ColorTint::default()),
                ImageNode {
                    image: asset_server.load("normal_image.png"),
                    ..default()
                },
                Node {
                    width: Val::Px(250.0),
                    height: Val::Px(80.0),
                    ..default()
                },
            ));
            ui_root.spawn((
                ButtonTransition::ImageSwap(ImageSwap {
                    normal_image: asset_server.load("normal_image.png"),
                    hovered_image: asset_server.load("hovered_image.png"),
                    pressed_image: asset_server.load("pressed_image.png"),
                    disabled_image: asset_server.load("disabled_image.png"),
                }),
                Node {
                    width: Val::Px(250.0),
                    height: Val::Px(80.0),
                    ..default()
                },
            ));
            ui_root.spawn(Text::from("Press SPACE to toggle interactability"));
        });
}

fn toggle_interactability(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Interactable>) {
    if input.just_pressed(KeyCode::Space) {
        for mut interactable in &mut query {
            interactable.0 = !interactable.0;
        }
    }
}
