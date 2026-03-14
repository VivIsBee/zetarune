//! Test game for zetarune

use std::collections::{HashMap, HashSet};

use zetarune::{
    components::party::{self, WorldCharacterBuilder},
    ctx::{Ctx, ObjectRef, RoomRef},
    objs::{Collider, ColliderType, LanguageData, ObjectState, Offset2, Room, World},
    resources::{self, Provider},
};

fn main() {
    let mut ctx = Ctx::new();
    // let sprite = ctx.add_placeholder_sprite(
    //     "deltarune_ch1:spr_krisr_0".to_string(),
    // );
    // let ani = ctx.add_ani(
    //     "ani01".to_string(),
    //     Animation {
    //         timeline: vec![AniEvent::Sprite {
    //             sprite,
    //             frame_count: NonZeroU8::new(1).unwrap(),
    //         }],
    //         fps: 0,
    //         loops: true,
    //     },
    // );
    // let sheet = ctx.add_sheet(
    //     "sheet01".to_string(),
    //     AniSheet {
    //         anis: [("idle".to_string(), ani)].into_iter().collect(),
    //     },
    // );

    let lang = ctx.add_lang(
        "n/a".to_string(),
        LanguageData {
            strings: HashMap::new(),
        },
    );

    let placeholder_room = RoomRef::default(&mut ctx);
    let placeholder_camera = ObjectRef::default(&mut ctx);

    let (sheet, sprites_to_load) = party::WorldCharacter::set_sprite_placeholders_deltarune(
        "deltarune_ch1",
        "kris",
        &mut ctx,
        true,
        true,
    );

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

    let obj1 = WorldCharacterBuilder::new(sheet, true)
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

    let mut provider = resources::GamemakerDataProvider::new(
        "DELTARUNE: Chapter 1",
        sprites_to_load,
        HashSet::new(),
    );

    provider.present().unwrap();
    provider.load(&mut world.ctx, "deltarune_ch1").unwrap();

    zetarune::rt::main("Zetarune Party Test", false, world);
}
