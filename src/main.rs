//! Test game for zetarune

use std::{collections::HashMap, num::NonZeroU8};

use zetarune::{
    components::party::PartyMemberBuilder,
    ctx::{Ctx, ObjectRef, RoomRef},
    objs::{
        AniEvent, AniSheet, Animation, Collider, ColliderType, Color, LanguageData, ObjectState,
        Offset2, Room, Sprite, World,
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
            anis: [("idle".to_string(), ani)].into_iter().collect(),
        },
    );

    let lang = ctx.add_lang(
        "n/a".to_string(),
        LanguageData {
            strings: HashMap::new(),
        },
    );

    let placeholder_room = RoomRef::default(&mut ctx);
    let placeholder_camera = ObjectRef::default(&mut ctx);

    let mut world = World::new(
        ctx,
        placeholder_room,
        vec![],
        vec![],
        None,
        ObjectState::new(),
        placeholder_camera,
        HashMap::new(),
        None,
        lang,
        "example.zetarune.components.party".to_string(),
    );

    let obj1 = PartyMemberBuilder::new(sheet)
        .add_collider(vec![Collider {
            t: ColliderType::Rect {
                size: Offset2 { x: 50.0, y: 50.0 },
            },
            off: Offset2::ZERO,
        }])
        .player()
        .create(&mut world)
        .add_to_party(&mut world);

    let room = world.ctx.add_room(
        "room01".to_string(),
        Room {
            background: None,
            objects: vec![*obj1],
            callbacks: None,
            state: ObjectState::new(),
            entrypoints: HashMap::new(),
        },
    );
    
    world.current_room = room;

    zetarune::rt::main("Zetarune Party Test", false, world);
}
