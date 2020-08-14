use bevy::prelude::*;
use bevy_input::keyboard::{convert_keycode_to_char, ElementState, KeyboardInput};

/// This example illustrates how to create text and update it in a system. It displays the current FPS in the upper left hand corner.
fn main() {
    App::build()
        .add_default_plugins()
        .init_resource::<State>()
        .init_resource::<LabelMaterials>()
        .add_startup_system(setup.system())
        .add_system(text_input_system.system())
        .add_system(interact_system.system())
        .add_system(focused_textinput_system.system())
        .run();
}

struct LabelMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

struct Focused;

#[derive(Default)]
struct State {
    event_reader: EventReader<KeyboardInput>,
}

impl FromResources for LabelMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        LabelMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}

fn interact_system(
    materials: Res<LabelMaterials>,
    mut interaction_query: Query<(&TextInput, Mutated<Interaction>, &mut Handle<ColorMaterial>)>,
) {
    for (_text_input, interaction, mut material) in &mut interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                *material = materials.pressed;
            }
            Interaction::Hovered => {
                *material = materials.hovered;
            }
            Interaction::None => {
                *material = materials.normal;
            }
        }
    }
}

fn focused_textinput_system(
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &TextInput, Mutated<Interaction>, &Children)>,
    mut focused_query: Query<(Entity, &TextInput, &Focused, &Children)>,
) {
    for (entity, _text_input, interaction, main_children) in &mut interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            for (entity, _, _, children) in &mut focused_query.iter() {
                commands.remove_one::<Focused>(entity);
                commands.remove_one::<Draw>(children[1]);
            }

            commands.insert_one(entity, Focused);
            commands.insert_one(main_children[1], Draw::default());
        }
    }
}

fn text_input_system(
    mut state: ResMut<State>,
    keyboard_input_events: Res<Events<KeyboardInput>>,
    mut focused_textinput: Query<(&TextInput, &Children, &Focused)>,
    text_query: Query<&mut Text>,
) {
    for (_text_input, children, _focused) in &mut focused_textinput.iter() {
        let mut text = text_query.get_mut::<Text>(children[0]).unwrap();

        for event in state.event_reader.iter(&keyboard_input_events) {
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
    textinput_materials: Res<LabelMaterials>,
) {
    let font_handle = asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap();

    commands
        .spawn(UiCameraComponents::default())
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextInputComponents {
                    style: Style {
                        size: Size::new(Val::Px(300.0), Val::Px(65.0)),
                        margin: Rect::all(Val::Auto),
                        padding: Rect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        ..Default::default()
                    },
                    material: textinput_materials.normal,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextComponents {
                        text: Text {
                            value: "Input 1".to_string(),
                            font: font_handle,
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.8, 0.8, 0.8),
                            },
                        },
                        ..Default::default()
                    });

                    parent.spawn(NodeComponents {
                        style: Style {
                            size: Size::new(Val::Px(5.0), Val::Px(30.0)),
                            margin: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            parent
                .spawn(TextInputComponents {
                    style: Style {
                        size: Size::new(Val::Px(300.0), Val::Px(65.0)),
                        // center
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: textinput_materials.normal,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextComponents {
                        text: Text {
                            value: "Input 2".to_string(),
                            font: font_handle,
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.8, 0.8, 0.8),
                            },
                        },
                        ..Default::default()
                    });

                    parent.spawn(NodeComponents {
                        style: Style {
                            size: Size::new(Val::Px(5.0), Val::Px(30.0)),
                            margin: Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
}
