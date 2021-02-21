use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::gamestate::{GameStage, GameState};
use crate::jam;
use crate::jam::JamEffect;

pub struct ShopScenePlugin;

static PHRASES: &[&[(Option<JamEffect>, &str)]] = &[
    /*Intro*/
    &[
        (Some(JamEffect::Hunger), "I was scavenging for food when "),
        (None, "The other day, "),
        (None, "In a firefight, "),
        (None, "Before the war, "),
    ],
    /*Villain*/
    &[
        (
            Some(JamEffect::SuperHumanStrength),
            "a raider far stronger than me ",
        ),
        (None, "a rival gang "),
        (
            Some(JamEffect::Antivenom),
            "a mutated snake with potent venom ",
        ),
        (None, "an Old War soldier "),
        (None, "an enemy fuel convoy "),
        (
            Some(JamEffect::CureDisease),
            "a feral dog, riddled with diseases, ",
        ),
    ],
    /*Adjective*/
    &[
        (None, "angrily "),
        (None, "furiously "),
        (None, "violently "),
        (None, "suddenly "),
    ],
    /*Action*/
    &[
        (Some(JamEffect::Coagulant), "stabbed "),
        (None, "robbed "),
        (None, "destroyed "),
        (None, "hunted "),
        (None, "shot at "),
    ],
    /*Hero*/
    &[
        (None, "my raiding party "),
        (None, "me "),
        (None, "my war-dog "),
        (
            Some(JamEffect::Speed),
            "my armoured truck, leaving me slow, ",
        ),
        (Some(JamEffect::Hunger), "my food supplies "),
    ],
    /*joining*/
    &[
        (None, "whilst I was "),
        (None, "when I was "),
        (None, "after I was caught "),
        (None, "for "),
    ],
    /*Action*/
    &[
        (Some(JamEffect::Invisibility), "trying to steal "),
        (None, "destroying "),
        (Some(JamEffect::Speed), "escaping with "),
        (None, "running over "),
        (None, "gambling away "),
        (Some(JamEffect::Poison), "poisoning "),
    ],
    /*belonging*/
    &[
        (None, "their water supply, "),
        (None, "their supplies, "),
        (None, "their credits, "),
        (None, "their jam, "),
        (Some(JamEffect::Flammable), "their fuel, "),
        (None, "their Old World relics, "),
        (Some(JamEffect::Flight), "their pre-war iron bird "),
    ],
    /*belonging*/
    &[
        (None, "so we "),
        (None, "so I "),
        (None, "and then I "),
        (None, "and then we "),
    ],
    /*belonging*/
    &[
        (None, "engaged them in hand to hand combat, "),
        (None, "began shooting at them, "),
        (None, "turned and ran away, "),
        (None, "offered them a truce, "),
        (None, "told them to surrender, "),
    ],
    /*belonging*/
    &[
        (None, "but then "),
        (None, "unfortunately this was interrupted when "),
        (None, "before this could happen "),
        (None, "suddenly, out of nowhere "),
    ],
    /*belonging*/
    &[
        (None, "a huge explosion went off, which caused "),
        (None, "a passionate glance was exchanged, which caused "),
        (
            Some(JamEffect::Antivenom),
            "a poisoned trap clamped on my leg , causing ",
        ),
        (
            None,
            "a severe gust of rad-wind tore through the valley, causing ",
        ),
        (
            Some(JamEffect::SuperHumanStrength),
            "my body became suddenly weak, causing ",
        ),
    ],
    /*belonging*/
    &[
        (Some(JamEffect::Coagulant), "my leg to fall off. "),
        (
            Some(JamEffect::CureDisease),
            "my raid members to become violently sick. ",
        ),
        (Some(JamEffect::Flammable), "my matches to get wet. "),
        (Some(JamEffect::NightVision), "everything to go dark. "),
    ],
];

struct Moveable {
    move_timer: Timer,
    start: Vec2,
    end: Vec2,
    delay_timer: Timer,
}

impl Plugin for ShopScenePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(GameStage::Main, GameState::Main, setup.system())
            .on_state_update(GameStage::Main, GameState::Main, animate_sprites.system())
            .on_state_update(GameStage::Main, GameState::Main, move_sprites.system())
            .on_state_update(GameStage::Main, GameState::Main, gen_story.system())
            .on_state_exit(GameStage::Main, GameState::Main, teardown.system());
    }
}

struct Background;

fn teardown(commands: &mut Commands, q_background: Query<Entity, With<Background>>) {
    for entity in q_background.iter() {
        commands.despawn(entity);
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let shopfront_handle = asset_server.load("sprites/front.png");
    let background_handle = asset_server.load("sprites/background.png");
    let tumbleweed_handle = asset_server.load("sprites/tumbleweedsheet.png");
    let tumbleweed_atlas = TextureAtlas::from_grid(tumbleweed_handle, Vec2::new(32.0, 32.0), 4, 1);
    let tumbleweed_atlas_handle = texture_atlases.add(tumbleweed_atlas);
    let buggy_handle = asset_server.load("sprites/buggy-sheet.png");
    let buggy_atlas = TextureAtlas::from_grid(buggy_handle, Vec2::new(128.0, 64.0), 4, 1);
    let buggy_atlas_handle = texture_atlases.add(buggy_atlas);

    commands
        .spawn(SpriteBundle {
            material: materials.add(background_handle.into()),
            ..Default::default()
        })
        .with(Background)
        .spawn(SpriteSheetBundle {
            texture_atlas: tumbleweed_atlas_handle,
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Moveable {
            move_timer: Timer::from_seconds(20.0, true),
            start: Vec2::new(-420.0, -50.0),
            end: Vec2::new(420.0, -50.0),
            delay_timer: Timer::from_seconds(15.0, true),
        })
        .with(Background)
        .spawn(SpriteSheetBundle {
            texture_atlas: buggy_atlas_handle,
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true))
        .with(Moveable {
            move_timer: Timer::from_seconds(5.0, true),
            start: Vec2::new(520.0, -70.0),
            end: Vec2::new(-520.0, -70.0),
            delay_timer: Timer::from_seconds(40.0, true),
        })
        .with(Background)
        .spawn(SpriteBundle {
            material: materials.add(shopfront_handle.into()),
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            ..Default::default()
        })
        .with(Background)
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                flex_wrap: FlexWrap::Wrap,
                position_type: PositionType::Absolute,
                max_size: Size {
                    width: Val::Px(520.0),
                    height: Val::Px(100.0),
                    ..Default::default()
                },
                position: Rect {
                    top: Val::Px(500.0),
                    left: Val::Px(140.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            text: Text::with_section(
                "Read the instructions, the game will start soon!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 15.0,
                    color: Color::BLACK,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .with(Background)
        .with(Timer::from_seconds(5.0, true));
}

fn animate_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn move_sprites(time: Res<Time>, mut query: Query<(&mut Moveable, &mut Transform)>) {
    for (mut moveable, mut transform) in query.iter_mut() {
        if !moveable
            .move_timer
            .tick(time.delta_seconds())
            .just_finished()
            && !moveable.move_timer.paused()
        {
            let new_pos =
                moveable.start + (moveable.end - moveable.start) * moveable.move_timer.percent();
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
        } else if !moveable
            .delay_timer
            .tick(time.delta_seconds())
            .just_finished()
        {
            moveable.move_timer.pause();
        } else {
            moveable.move_timer.reset();
            moveable.move_timer.unpause();
        }
    }
}

fn gen_story(time: Res<Time>, mut query: Query<(&mut Timer, &mut Text)>) {
    for (mut timer, mut text) in query.iter_mut() {
        if !timer.tick(time.delta_seconds()).just_finished() {
            return;
        }

        let mut effects = Vec::new();
        let mut story = String::from("");
        for x in 0..13 {
            let (effect, text_fragment) = PHRASES[x].choose(&mut rand::thread_rng()).unwrap();
            story.push_str(text_fragment);
            effects.push(effect);
        }
        story.push_str("As you can tell, I am in deperate need of assistance, do you have any jam that could help me ensure this doesn't happen again?");
        text.sections[0].value = format!("{:2}", story)
    }
}
