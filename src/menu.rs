use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectPosition>()
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(update_select_button))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(clear_menu));
    }
}

struct SelectPosition {
    start_game: UiRect<Val>,
    continue_game: UiRect<Val>,
    bonus_menu: UiRect<Val>,
    options: UiRect<Val>,
    exit: UiRect<Val>,
}

impl Default for SelectPosition {
    fn default() -> Self {
        SelectPosition {
            start_game: UiRect {
                left: Val::Percent(93.0 / 640.0 * 100.0),
                top: Val::Percent(206.0 / 480.0 * 100.0),
                ..default()
            },
            continue_game: UiRect {
                left: Val::Percent(93.0 / 640.0 * 100.0),
                top: Val::Percent(238.0 / 480.0 * 100.0),
                ..default()
            },
            bonus_menu: UiRect {
                left: Val::Percent(93.0 / 640.0 * 100.0),
                top: Val::Percent(270.0 / 480.0 * 100.0),
                ..default()
            },
            options: UiRect {
                left: Val::Percent(93.0 / 640.0 * 100.0),
                top: Val::Percent(302.0 / 480.0 * 100.0),
                ..default()
            },
            exit: UiRect {
                left: Val::Percent(93.0 / 640.0 * 100.0),
                top: Val::Percent(334.0 / 480.0 * 100.0),
                ..default()
            },
        }
    }
}

enum StartMenuItems {
    StartGame = 0,
    ContinueGame = 1,
    BonusMenu = 2,
    Options = 3,
    Exit = 4,
}

#[derive(Component)]
struct StartMenu {
    item: StartMenuItems,
}

impl StartMenu {
    fn new() -> Self {
        Self {
            item: StartMenuItems::StartGame,
        }
    }

    fn prev(&mut self) {
        let mut v = self.item as u8;
        if v == 0 {
            v = 5;
        }
        v -= 1;
        self.item = StartMenuItems::from_u8(v);
    }

    fn next(&mut self) {
        let mut v = self.item as u8 + 1;
        if v > 4 {
            v = 0;
        }
        self.item = StartMenuItems::from_u8(v);
    }
}

fn setup_menu(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    select_position: Res<SelectPosition>,
) {
    // use tracing::{span, Level};
    // let span = span!(Level::INFO, "my_span");
    //
    // let _enter = span.enter();
    //
    // let a = 3;
    //
    // info!("Hello{}", a);

    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            image: UiImage::from(texture_assets.bg_ui.clone()),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: select_position.start_game,
                        size: Size::new(
                            Val::Percent(168.0 / 640.0 * 100.0),
                            Val::Percent(34.0 / 480.0 * 100.0),
                        ),
                        ..default()
                    },
                    image: UiImage::from(texture_assets.bg_select.clone()),
                    ..default()
                })
                .insert(StartMenu::new());

            // parent.spawn_bundle(ButtonBundle {
            //     style: Style {
            //         size: Size::new(Val::Px(200.0), Val::Px(50.0)),
            //         margin: UiRect::all(Val::Auto),
            //         justify_content: JustifyContent::Center,
            //         align_items: AlignItems::Center,
            //         ..default()
            //     },
            //     color: button_colors.normal,
            //     ..default()
            // })
            //     .with_children(|parent| {
            //         parent.spawn_bundle(TextBundle::from_section(
            //             "开始游戏",
            //             TextStyle {
            //                 font: font_assets.ms_yh.clone(),
            //                 font_size: 40.0,
            //                 color: Color::rgb(0.2, 0.2, 0.2),
            //             },
            //         ));
            //     });
        });
}

fn update_select_button(
    select_position: Res<SelectPosition>,
    mut state: ResMut<State<GameState>>,
    mut query: Query<(&mut Style, &mut StartMenu)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_released(KeyCode::Up)
        || keyboard_input.just_released(KeyCode::Down)
        || keyboard_input.just_released(KeyCode::Return)
    {
        for (mut ui, mut menu) in &mut query {
            if keyboard_input.just_released(KeyCode::Up) {
                menu.prev();
            } else if keyboard_input.just_released(KeyCode::Down) {
                menu.next();
            }
            ui.position = select_position.get_value(&menu.item);
            if keyboard_input.just_released(KeyCode::Return) {
                if let StartMenuItems::StartGame = menu.item {
                    state.set(GameState::Playing).unwrap();
                }
            }
        }
    }

    //
    // for (button, interaction, mut color) in interaction_query.iter_mut() {
    //     match *interaction {
    //         Interaction::Clicked => {
    //             commands.entity(button).despawn_recursive();
    //             state.set(GameState::Playing).unwrap();
    //         }
    //         Interaction::Hovered => {
    //             *color = button_colors.hovered;
    //         }
    //         Interaction::None => {
    //             *color = button_colors.normal;
    //         }
    //     }
    // }
}

fn clear_menu(mut commands: Commands, query: Query<Entity>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

impl StartMenuItems {
    fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::StartGame,
            1 => Self::ContinueGame,
            2 => Self::BonusMenu,
            3 => Self::Options,
            4 => Self::Exit,
            _ => Self::StartGame,
        }
    }
}

impl SelectPosition {
    fn get_value(&self, v: &StartMenuItems) -> UiRect<Val> {
        match v {
            StartMenuItems::StartGame => self.start_game,
            StartMenuItems::ContinueGame => self.continue_game,
            StartMenuItems::BonusMenu => self.bonus_menu,
            StartMenuItems::Options => self.options,
            StartMenuItems::Exit => self.exit,
        }
    }
}
