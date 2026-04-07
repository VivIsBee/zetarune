#![cfg_attr(target_os = "horizon", no_std)]
//! Test game for zetarune

extern crate alloc;

use zetarune::{
    HashMap, HashSet,
    components::{
        dialogue::{DialogueItem, Dialoguer, TextItem},
        world_npc::{self, WorldCharacterBuilder},
    },
    ctx::{Ctx, ObjectRef, RoomRef},
    objs::{Callbacks, Collider, EventName, EventResult, LanguageData, Object, ObjectColliderType, ObjectState, ObjectStateKey, Offset2, Room, Vec2, World},
    resources::{self, Provider}, rt::{Key, KeyCode},
    trace,
};

use alloc::{string::ToString, vec};

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
        Callbacks::new(),
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

    world.callbacks.set(EventName::KeyPress, move |ev, args| {
        let ev = ev.unwrap_KeyPress();
        let world = args.world;

        if ev.key == Key::Keyboard(KeyCode::A) {
            let name = world.ctx.get_room_id(world.current_room);
            trace!("{name}");
        }

        EventResult::Default
    });

    let obj2 = WorldCharacterBuilder::new(sheet2, false)
        .party_member(1)
        .create(&mut world)
        .add_to_party(&mut world);

    let obj3 = WorldCharacterBuilder::new(sheet3, true)
        .party_member(2)
        .create(&mut world)
        .add_to_party(&mut world);

    let entry2 = world.ctx.add_placeholder_entrypoint("entry2".to_string());

    let mut mover = Object {
        collider: vec![Collider { t: zetarune::objs::ColliderType::Rect { size: Offset2 { x: 2.0, y: 2.0 } }, off: Offset2::ZERO }],
        collider_type: ObjectColliderType::Area,
        sheet: Some(sheet1),
        state: ObjectState::new(),
        callbacks: Callbacks::new()
    };

    mover.state.set(ObjectStateKey::Animation, "u_dark");
    mover.state.set(ObjectStateKey::AniFrame, 0usize);
    mover.state.set(ObjectStateKey::Pos, Vec2 { x: 100.0, y: 0.0 });
    mover.state.set(ObjectStateKey::Visible, true);
    mover.state.set(ObjectStateKey::Scale, Offset2::ONE);
    mover.state.set(ObjectStateKey::Processing, true);

    mover.callbacks.set(EventName::PlayerCollide, move |ev, args| {
        trace!("collision");
        args.world.transition_room(entry2);

        EventResult::Default
    });

    let mover = world.ctx.add_obj("mover".to_string(), mover);

    let mut room1 = Room {
        background: None,
        objects: vec![mover],
        callbacks: Callbacks::new(),
        state: ObjectState::new(),
        entrypoints: HashMap::new(),
    };

    let entry1 = world.ctx.add_placeholder_entrypoint("entry1".to_string());

    room1.entrypoints.insert(entry1, Vec2::ZERO);

    let room1 = world.ctx.add_room("room01".to_string(), room1);

    world.ctx.fill_placeholder_entrypoint(entry1, room1);

    world.current_room = room1;

    let mut room2 = Room {
        background: None,
        objects: vec![],
        callbacks: Callbacks::new(),
        state: ObjectState::new(),
        entrypoints: HashMap::new(),
    };

    room2.entrypoints.insert(entry2, Vec2::ZERO);

    let room2 = world.ctx.add_room("room02".to_string(), room2);

    world.ctx.fill_placeholder_entrypoint(entry2, room2);

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
