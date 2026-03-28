#![no_std]
//! Test game for zetarune

extern crate alloc;

use zetarune::{
    HashMap, HashSet,
    components::{
        dialogue::{DialogueItem, Dialoguer, TextItem},
        world_npc::{self, WorldCharacterBuilder},
    },
    ctx::{Ctx, ObjectRef, RoomRef},
    objs::{LanguageData, ObjectState, Room, World},
    resources::{self, Provider},
};

use alloc::{vec, string::ToString};

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

    let text = ctx.add_local_text("example".to_string());

    let mut lang = LanguageData {
        strings: HashMap::new(),
    };

    lang.strings.insert(
        text,
        "HelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHelloHello"
            .to_string(),
    );

    let lang = ctx.add_lang("en-US".to_string(), lang);

    let placeholder_room = RoomRef::default(&mut ctx);
    let placeholder_camera = ObjectRef::default(&mut ctx);

    let (sheet1, sprites_to_load1) = world_npc::WorldCharacter::set_sprite_placeholders_deltarune(
        "deltarune_ch1",
        "kris",
        &mut ctx,
        true,
        true,
    );
    let (sheet2, sprites_to_load2) = world_npc::WorldCharacter::set_sprite_placeholders_deltarune(
        "deltarune_ch1",
        "ralsei",
        &mut ctx,
        false,
        false,
    );
    let (sheet3, sprites_to_load3) = world_npc::WorldCharacter::set_sprite_placeholders_deltarune(
        "deltarune_ch1",
        "susie",
        &mut ctx,
        false,
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

    let obj1 = WorldCharacterBuilder::new(sheet1, true)
        .player()
        .create(&mut world)
        .add_to_party(&mut world);

    world
        .state
        .set(zetarune::objs::ObjectStateKey::IsLight, false);

    let obj2 = WorldCharacterBuilder::new(sheet2, false)
        .party_member(1)
        .create(&mut world)
        .add_to_party(&mut world);

    let obj3 = WorldCharacterBuilder::new(sheet3, true)
        .party_member(2)
        .create(&mut world)
        .add_to_party(&mut world);

    let room = world.ctx.add_room(
        "room01".to_string(),
        Room {
            background: None,
            objects: vec![],
            callbacks: None,
            state: ObjectState::new(),
            entrypoints: HashMap::new(),
        },
    );

    world.current_room = room;

    Dialoguer::new(&mut world);

    world.push_dialogue(DialogueItem {
        text: vec![TextItem::Text(text)].into(),
        small_extra: None,
    });

    let mut provider = resources::GamemakerDataProvider::new(
        "DELTARUNE: Chapter 1",
        sprites_to_load1
            .into_iter()
            .chain(sprites_to_load2.into_iter())
            .chain(sprites_to_load3.into_iter())
            .collect(),
        HashSet::new(),
    );

    provider.present().unwrap();
    provider.load(&mut world.ctx, "deltarune_ch1").unwrap();

    zetarune::rt::main("Zetarune Party Test", false, world);
}
