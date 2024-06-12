use std::str::FromStr;

use bevy::{prelude::*, window::WindowResized};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};

const PITCH_NAMES: [char; 7] = ['C', 'D', 'E', 'F', 'G', 'A', 'B'];

const STAFF_ELEM_IDX_TREBLE: usize = 0;
const STAFF_ELEM_IDX_BASS: usize = 1;
const STAFF_ELEM_IDX_NOTE_4: usize = 3;

const STAFF_ELEMENTS: [(Vec2, &str); 5] = [
    (Vec2::new(0., 18.), "mtb_images/clef_treble.png"),
    (Vec2::new(4., -14.), "mtb_images/clef_bass.png"),
    (Vec2::new(0., 0.), "mtb_images/note_2.png"),
    (Vec2::new(0., 0.), "mtb_images/note_4.png"),
    (Vec2::new(0., 0.), "mtb_images/rest_4.png"),
];

const STAFF_WIDTH: i32 = 1200;
const STAFF_X: i32 = -STAFF_WIDTH / 2;
const STAFF_Y: i32 = 40;
const STAFF_SPACE_Y: i32 = 20;

const STAFF_NOTE_X: i32 = 120;
const STAFF_MAX_NOTES: usize = 30;

#[derive(Clone, Copy, EnumString, EnumIter, AsRefStr)]
enum TrainCourse {
    TrebleLines,
    TrebleSpaces,
    TrebleAll,
    BassLines,
    BassSpaces,
    BassAll,
    All,
}

struct TrainNote {
    index: usize,
    pitch: i32,
    pressed_key: char,
}

#[derive(Resource)]
struct TrainSequence {
    notes: Vec<TrainNote>,
    next_key: usize,
    course: TrainCourse,
}

#[derive(Component)]
struct NoteSpriteInfo {
    index: usize,
}

#[derive(Component)]
struct ResolutionText;

fn index_to_x(index: usize, note_space: f32) -> f32 {
    (STAFF_X + STAFF_NOTE_X) as f32 + note_space * index as f32
}

fn pitch_to_y(pitch: i32) -> f32 {
    (STAFF_Y + pitch * STAFF_SPACE_Y / 2) as f32
}

fn pitch_to_char(pitch: i32) -> char {
    let index = pitch.rem_euclid(PITCH_NAMES.len() as i32) as usize;
    PITCH_NAMES[index]
}

impl TrainSequence {
    fn new() -> Self {
        TrainSequence {
            notes: vec![],
            next_key: 0,
            course: TrainCourse::All,
        }
    }

    fn gen_demo(&mut self) {
        self.next_key = 0;
        let mut notes = vec![];
        for (index, pitch) in (-20..5).enumerate() {
            notes.push(TrainNote {
                index,
                pitch,
                pressed_key: default(),
            });
        }
        self.notes = notes;
    }

    fn gen_course(&mut self, course: TrainCourse) {
        self.course = course;
        self.next_key = 0;
        let mut rng = rand::thread_rng();
        let mut notes = vec![];
        for index in 0..16 {
            notes.push(Self::_generate_course_note(index, course, &mut rng));
        }
        assert!(notes.len() <= STAFF_MAX_NOTES);
        self.notes = notes;
    }

    fn _generate_course_note<R>(index: usize, course: TrainCourse, rng: &mut R) -> TrainNote
    where
        R: rand::Rng + ?Sized,
    {
        let pitch: i32 = match course {
            TrainCourse::TrebleLines => *[0, 2, 4, 6, 8, 10, 12].choose(rng).unwrap(),
            TrainCourse::TrebleSpaces => *[1, 3, 5, 7, 9, 11].choose(rng).unwrap(),
            TrainCourse::TrebleAll => rng.gen_range(0..13),
            TrainCourse::BassLines => -*[0, 2, 4, 6, 8, 10, 12].choose(rng).unwrap(),
            TrainCourse::BassSpaces => -*[1, 3, 5, 7, 9, 11].choose(rng).unwrap(),
            TrainCourse::BassAll => -rng.gen_range(0..13),
            TrainCourse::All => -rng.gen_range(-12..13),
        };
        TrainNote {
            index,
            pitch,
            pressed_key: default(),
        }
    }

    fn get_note_space(&self) -> f32 {
        let segments_f = 1.0f32.max(self.notes.len() as f32 - 1.0);
        (STAFF_WIDTH - STAFF_NOTE_X - 40) as f32 / segments_f
    }
}

fn staff_update(mut gizmos: Gizmos, train: ResMut<TrainSequence>) {
    // Draw staff
    for idx in 0..5 {
        let x = STAFF_X as f32;
        let y = (STAFF_Y + STAFF_SPACE_Y + idx * STAFF_SPACE_Y) as f32;
        gizmos.line_2d(
            Vec2::new(x, y),
            Vec2::new(x + STAFF_WIDTH as f32, y),
            Color::BLACK,
        )
    }

    for idx in 0..5 {
        let x = STAFF_X as f32;
        let y = (STAFF_Y - STAFF_SPACE_Y - idx * STAFF_SPACE_Y) as f32;
        gizmos.line_2d(
            Vec2::new(x, y),
            Vec2::new(x + STAFF_WIDTH as f32, y),
            Color::BLACK,
        )
    }

    // Draw extra lines
    let note_space = train.get_note_space();

    fn draw_extra_line(g: &mut Gizmos, x: f32, width: f32, pitch: i32) {
        let y = pitch_to_y(pitch);
        let hw = width * 0.5;
        g.line_2d(Vec2::new(x - hw, y), Vec2::new(x + hw, y), Color::BLACK);
    }

    for note in train.notes.iter() {
        let x = index_to_x(note.index, note_space);
        if note.pitch == 0 {
            draw_extra_line(&mut gizmos, x, note_space, 0);
        } else if note.pitch.abs() > 10 {
            let mut pitch: i32 = 12;
            while pitch <= note.pitch {
                draw_extra_line(&mut gizmos, x, note_space, pitch);
                pitch += 2;
            }
            let mut pitch: i32 = -12;
            while pitch >= note.pitch {
                draw_extra_line(&mut gizmos, x, note_space, pitch);
                pitch -= 2;
            }
        }
    }

    // Draw Rectangle
    let next_key = train.next_key;
    if next_key < train.notes.len() {
        let x = index_to_x(next_key, note_space);
        let y = pitch_to_y(train.notes[next_key].pitch);
        gizmos.rect_2d(Vec2::new(x, y), 0., Vec2::splat(34.), Color::srgb(0., 0., 1.));
    }
}

fn staff_update_sprites(
    mut sprites: Query<(&Sprite, &NoteSpriteInfo, &mut Transform, &mut Visibility)>,
    train: ResMut<TrainSequence>,
) {
    let num_notes = train.notes.len();
    let note_space = train.get_note_space();
    for (_, info, mut transform, mut visibility) in &mut sprites {
        let index = info.index;
        if index < num_notes {
            *visibility = Visibility::Visible;

            let note = &train.notes[index];

            let x = index_to_x(note.index, note_space);
            let y = pitch_to_y(note.pitch);

            *transform = Transform::from_xyz(x, y, 0.);
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn staff_update_labels(
    mut labels: Query<(&mut Text, &NoteSpriteInfo, &mut Transform, &mut Visibility)>,
    train: ResMut<TrainSequence>,
) {
    let num_visible_labels = train.next_key;
    let note_space = train.get_note_space();
    for (mut text, info, mut transform, mut visibility) in &mut labels {
        let index = info.index;
        if index < num_visible_labels {
            *visibility = Visibility::Visible;

            let note = &train.notes[index];

            let x = index_to_x(note.index, note_space);
            let y = pitch_to_y(note.pitch);

            let expected_key = pitch_to_char(note.pitch);
            text.sections[0].value = expected_key.to_string();
            let col = if expected_key == note.pressed_key {
                Color::srgb(0., 1., 0.)
            } else {
                Color::srgb(1., 0., 0.)
            };
            text.sections[0].style.color = col;

            *transform = Transform::from_xyz(x, y, 1.);
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn staff_setup(
    mut commands: Commands,
    mut train: ResMut<TrainSequence>,
    asset_server: Res<AssetServer>,
) {
    {
        const CLEF_X: i32 = 40;
        // Treble Clef
        let (ref offset, asset_path) = STAFF_ELEMENTS[STAFF_ELEM_IDX_TREBLE];
        let x = (STAFF_X + CLEF_X) as f32 + offset.x;
        let y = (STAFF_Y + STAFF_SPACE_Y * 2) as f32 + offset.y;
        commands.spawn(SpriteBundle {
            texture: asset_server.load(asset_path),
            transform: Transform::from_xyz(x, y, 0.),
            ..default()
        });

        let (ref offset, asset_path) = STAFF_ELEMENTS[STAFF_ELEM_IDX_BASS];
        let x = (STAFF_X + CLEF_X) as f32 + offset.x;
        let y = (STAFF_Y - STAFF_SPACE_Y * 2) as f32 + offset.y;
        commands.spawn(SpriteBundle {
            texture: asset_server.load(asset_path),
            transform: Transform::from_xyz(x, y, 0.),
            ..default()
        });
    }

    train.gen_demo();
    {
        let (_, asset_path) = STAFF_ELEMENTS[STAFF_ELEM_IDX_NOTE_4];
        let texture = asset_server.load(asset_path);
        for index in 0..STAFF_MAX_NOTES {
            commands.spawn((
                SpriteBundle {
                    texture: texture.clone(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                NoteSpriteInfo { index },
            ));
            commands.spawn((
                Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "AAA",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        )],
                        ..default()
                    },
                    ..default()
                },
                NoteSpriteInfo { index },
            ));
        }
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

type ButtonQuery<'world, 'state, 'a, 'b, 'c, 'd> = Query<
    'world,
    'state,
    (&'a Interaction, &'b mut UiImage, &'c mut BorderColor, &'d Children),
    (Changed<Interaction>, With<Button>),
>;

fn game_button_system(
    mut interaction_query: ButtonQuery,
    text_query: Query<&mut Text>,
    mut train: ResMut<TrainSequence>,
) {
    for (interaction, mut image, mut border_color, children) in &mut interaction_query {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                image.color = PRESSED_BUTTON;
                border_color.0 = Color::WHITE;
                let t = &text.sections[0].value;
                if t.len() == 1 {
                    let key_char = t.chars().next().unwrap();
                    let next_key = train.next_key;
                    if next_key < train.notes.len() {
                        train.notes[next_key].pressed_key = key_char;
                        train.next_key += 1;
                    } else {
                        let course = train.course;
                        println!("GameOver, new round {}", course.as_ref());
                        train.gen_course(course);
                    }
                } else {
                    match TrainCourse::from_str(t) {
                        Ok(course) => train.gen_course(course),
                        Err(err) => println!("{}", err),
                    }
                }
            }
            Interaction::Hovered => {
                image.color = HOVERED_BUTTON;
                border_color.0 = Color::BLACK;
            }
            Interaction::None => {
                image.color = NORMAL_BUTTON;
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn game_button_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Courses
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                #[allow(clippy::unused_enumerate_index)]
                for (_idx, var) in TrainCourse::iter().enumerate() {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(150.0),
                                height: Val::Px(65.0),
                                border: UiRect::all(Val::Px(1.0)),
                                flex_direction: FlexDirection::Column,
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            border_radius: BorderRadius::all(Val::Px(4.)),
                            image: UiImage::default().with_color(NORMAL_BUTTON),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                var.as_ref(),
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                },
                            ));
                            if let TrainCourse::All = var {
                                // Resolution label
                                parent.spawn((
                                    TextBundle::from_section(
                                        "Resolution",
                                        TextStyle {
                                            font_size: 10.0,
                                            color: Color::srgb(0.5, 0.5, 0.9),
                                            ..default()
                                        },
                                    ),
                                    ResolutionText,
                                ));
                            }
                        });
                }
            });

            // Keyboard
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(50.0),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                #[allow(clippy::unused_enumerate_index)]
                for (_idx, &key) in PITCH_NAMES.iter().enumerate() {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(100.0),
                                height: Val::Px(130.0),
                                border: UiRect::all(Val::Px(1.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            border_color: BorderColor(Color::BLACK),
                            border_radius: BorderRadius::all(Val::Px(4.)),
                            image: UiImage::default().with_color(NORMAL_BUTTON),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                key,
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                }
            });
        });
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
    // mut windows: Query<&mut Window>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.read() {
        // let mut window = windows.single_mut();
        // window.resolution.set(1200., 540.);

        // When resolution is being changed
        text.sections[0].value = format!("{:.1} x {:.1}", e.width, e.height);
    }
}

// fn draw_example_collection(mut _gizmos: Gizmos) {
//     let start = Vec2::Y * 0.;
//     gizmos.line_2d(start, start + Vec2::X * 1000.0, Color::BLUE);
//     for idx in 0..STAFF_ELEMENTS.len() {
//         let x = idx as f32 * 100.;
//         let y = 50f32;
//         gizmos.line_2d(Vec2::new(x, -y), Vec2::new(x, y), Color::BLUE);
//     }
// }

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // for (idx, &(ref offset, asset_path)) in STAFF_ELEMENTS.iter().enumerate() {
    //     let x = idx as f32 * 100.;
    //     commands.spawn(SpriteBundle {
    //         texture: asset_server.load(asset_path),
    //         transform: Transform::from_xyz(x + offset.x, offset.y, 0.),
    //         ..default()
    //     });
    // }
}

pub struct TrainerPlugin;

impl Plugin for TrainerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TrainSequence::new())
            .add_systems(Startup, (setup, staff_setup, game_button_setup))
            .add_systems(
                Update,
                (staff_update_sprites, staff_update, staff_update_labels),
            )
            .add_systems(Update, on_resize_system)
            .add_systems(Update, game_button_system);
    }
}
