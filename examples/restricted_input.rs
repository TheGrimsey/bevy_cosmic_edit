use bevy::{prelude::*, window::PrimaryWindow};
use bevy_cosmic_edit::{
    bevy_color_to_cosmic, change_active_editor_sprite, change_active_editor_ui, Attrs, AttrsOwned,
    CosmicAttrs, CosmicEditPlugin, CosmicEditUiBundle, CosmicMaxChars, CosmicMaxLines,
    CosmicMetrics, CosmicText, Focus,
};

fn setup(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn(Camera2dBundle::default());

    let attrs =
        AttrsOwned::new(Attrs::new().color(bevy_color_to_cosmic(Color::rgb(0.27, 0.27, 0.27))));
    let primary_window = windows.single();

    let editor = commands
        .spawn(CosmicEditUiBundle {
            border_color: Color::LIME_GREEN.into(),
            style: Style {
                // Size and position of text box
                border: UiRect::all(Val::Px(4.)),
                width: Val::Percent(20.),
                height: Val::Px(50.),
                left: Val::Percent(40.),
                top: Val::Px(100.),
                ..default()
            },
            cosmic_attrs: CosmicAttrs(attrs.clone()),
            cosmic_metrics: CosmicMetrics {
                font_size: 16.,
                line_height: 16.,
                scale_factor: primary_window.scale_factor() as f32,
            },
            max_chars: CosmicMaxChars(15),
            max_lines: CosmicMaxLines(1),
            set_text: CosmicText::OneStyle(
                "1 line 15 chars! But this\n is longer\n than is\n allowed by\n the limits.\n"
                    .into(),
            ),
            ..default()
        })
        .id();

    commands.insert_resource(Focus(Some(editor)));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CosmicEditPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, change_active_editor_ui)
        .add_systems(Update, change_active_editor_sprite)
        .run();
}
