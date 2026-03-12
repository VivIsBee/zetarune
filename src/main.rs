//! Test game for zetarune

use std::{collections::HashMap, num::NonZeroU8};

use zetarune::{
    ctx::Ctx,
    objs::{
        AniEvent, AniSheet, Animation, Callbacks, Collider, ColliderType, Color, EventName, LanguageData, Object, ObjectState, ObjectStateKey, Offset2, Room, Sprite, Vec2, World
    },
};

fn main() {
    let mut ctx = Ctx::new();
    let sprite = ctx.add_sprite(
        "sprite01".to_string(),
        Sprite {
            width: 32,
            height: 32,
            data: vec![
                Color {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255
                };
                32 * 32
            ],
        },
    );
    let ani = ctx.add_ani(
        "ani01".to_string(),
        Animation {
            timeline: vec![AniEvent::Sprite {
                sprite,
                frame_count: NonZeroU8::new(1).unwrap(),
            }],
            fps: 0,
            loops: true,
        },
    );
    let sheet = ctx.add_sheet(
        "sheet01".to_string(),
        AniSheet {
            anis: [("ani01".to_string(), ani)].into_iter().collect(),
        },
    );

    let mut obj1 = Object {
        collider: vec![Collider {
            t: ColliderType::Rect {
                size: Offset2 { x: 32.0, y: 32.0 },
            },
            off: Offset2::ZERO,
        }],
        static_body: true,
        sheet: Some(sheet),
        state: ObjectState::new(),
        callbacks: Some(Callbacks::new()),
    };

    obj1.callbacks
        .as_mut()
        .unwrap()
        .set(EventName::Collide, |_, args| {
            // match event {
            //     Event::KeyPress { key: _ } => {
            //         args.world
            //             .ctx
            //             .get_mut_object(args.obj.unwrap())
            //             .state
            //             .set(ObjectStateKey::Pos, Vec2 { x: 100.0, y: 100.0 });
            //     }
            //     _ => unreachable!(),
            // }
            println!("collision on {:?}!", args.obj.unwrap());
            zetarune::objs::EventResult::Default
        });
    obj1.state
        .set(ObjectStateKey::Pos, Vec2 { x: 50.0, y: 50.0 });
    obj1.state.set(ObjectStateKey::Visible, true);
    obj1.state
        .set(ObjectStateKey::Animation, "ani01".to_string());
    obj1.state.set(ObjectStateKey::AniFrame, 0usize);
    let obj1 = ctx.add_obj("obj01".to_string(), obj1);

    let sprite = ctx.add_sprite(
        "sprite02".to_string(),
        Sprite {
            width: 32,
            height: 32,
            data: vec![
                Color {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 255
                };
                32 * 32
            ],
        },
    );
    let ani = ctx.add_ani(
        "ani02".to_string(),
        Animation {
            timeline: vec![AniEvent::Sprite {
                sprite,
                frame_count: NonZeroU8::new(1).unwrap(),
            }],
            fps: 0,
            loops: true,
        },
    );
    let sheet = ctx.add_sheet(
        "sheet02".to_string(),
        AniSheet {
            anis: [("ani02".to_string(), ani)].into_iter().collect(),
        },
    );

    let mut obj2 = Object {
        collider: vec![Collider {
            t: ColliderType::Rect {
                size: Offset2 { x: 32.0, y: 32.0 },
            },
            off: Offset2::ZERO,
        }],
        static_body: false,
        sheet: Some(sheet),
        state: ObjectState::new(),
        callbacks: Some(Callbacks::new()),
    };

    obj2.state
        .set(ObjectStateKey::Pos, Vec2 { x: 60.0, y: 60.0 });
    obj2.state.set(ObjectStateKey::Visible, true);
    obj2.state
        .set(ObjectStateKey::Animation, "ani02".to_string());
    obj2.state.set(ObjectStateKey::AniFrame, 0usize);
    let obj2 = ctx.add_obj("obj02".to_string(), obj2);

    let room = ctx.add_room(
        "room01".to_string(),
        Room {
            background: None,
            objects: vec![obj1, obj2],
            callbacks: None,
            state: ObjectState::new(),
            entrypoints: HashMap::new(),
        },
    );

    let lang = ctx.add_lang("n/a".to_string(), LanguageData { strings: HashMap::new() });

    let world = World::new(
        ctx,
        room,
        vec![],
        vec![],
        None,
        ObjectState::new(),
        obj1,
        HashMap::new(),
        None,
        lang,
        "example.zetarune.engine.simple".to_string()
    );

    zetarune::rt::main("Zetarune Test", false, world);
}
