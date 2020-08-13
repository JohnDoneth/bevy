use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_input::keyboard::{convert_keycode_to_char, ElementState, KeyboardInput};

/// This example illustrates how to create text and update it in a system. It displays the current FPS in the upper left hand corner.
fn main() {
    App::build()
        .add_default_plugins()
        .init_resource::<State>()
        .init_resource::<ButtonMaterials>()
        .add_startup_system(setup.system())
        .add_system(text_input_system.system())
        .add_system(button_system.system())
        .add_system(focused_textinput_system.system())
        .run();
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<(&TextInput, Mutated<Interaction>, &mut Handle<ColorMaterial>)>,
) {
    for (_button, interaction, mut material) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed;
            }
            Interaction::Hovered => {
                *material = button_materials.hovered;
            }
            Interaction::None => {
                *material = button_materials.normal;
            }
        }
    }
}

#[derive(Default)]
struct State {
    event_reader: EventReader<KeyboardInput>,
}

fn focused_textinput_system(
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &TextInput, Mutated<Interaction>)>,
) {
    for (entity, _text_input, interaction) in &mut interaction_query.iter() {
        //println!("interaction {:?}", *interaction);
        if *interaction == Interaction::Clicked {
            commands.insert_one(entity, Focused);
        } else {
            //commands.remove_one::<Focused>(entity);
        }
    }
}

struct Focused;

fn text_input_system(
    mut state: ResMut<State>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
    mut text: Query<&mut Text>,
) {
    //let mut text = text_query.get_mut::<Text>(children[0]).unwrap();

    for mut text in &mut text.iter() {
        for event in state.event_reader.iter(&keyboard_input_events) {
            println!("{:?}", event);

            if event.state == ElementState::Pressed {
                if let Some(keycode) = &event.key_code {
                    if let Some(c) = convert_keycode_to_char(keycode, &event.modifiers) {
                        text.value.push(c);
                    }
                }

                if event.key_code == Some(KeyCode::Back) {
                    text.value.pop();
                }
            }
        }
    }
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    let font_handle: Handle<Font> = asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap();
    commands
        // 2d camera
        .spawn(UiCameraComponents::default())
        // texture
        .spawn(TextInputComponents {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal,
            text: Text {
                value: "FPS:".to_string(),
                font: font_handle,
                style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: "Button".to_string(),
                    font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                },
                ..Default::default()
            });
        });
}
