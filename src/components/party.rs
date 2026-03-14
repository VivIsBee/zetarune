use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    num::NonZeroU8,
    ops::Deref,
    sync::atomic::AtomicUsize,
};

use macroquad::input::KeyCode;

use crate::{
    ctx::{ActionRef, AniSheetRef, Ctx, ObjectRef},
    objs::{
        AniEvent, AniSheet, Animation, Callbacks, Collider, Direction, Object, ObjectState,
        ObjectStateKey, Offset2, StateData, Vec2, World,
    },
    rt::Key,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorldCharacterBuilder {
    pub is_player: bool,
    pub is_lightner: bool,
    pub party_member_i: Option<usize>,
    pub character_sheet: AniSheetRef,
    pub collider: Vec<Collider>,
}

impl WorldCharacterBuilder {
    pub fn new(character_sheet: AniSheetRef, is_lightner: bool) -> Self {
        Self {
            is_player: false,
            is_lightner,
            party_member_i: None,
            character_sheet,
            collider: vec![],
        }
    }
    pub fn player(mut self) -> Self {
        self.is_player = true;
        self
    }
    pub fn not_player(mut self) -> Self {
        self.is_player = false;
        self
    }
    pub fn party_member(mut self, i: usize) -> Self {
        self.party_member_i = Some(i);
        self
    }
    pub fn not_party_member(mut self) -> Self {
        self.party_member_i = None;
        self
    }
    pub fn add_collider(mut self, mut collider: Vec<Collider>) -> Self {
        self.collider.append(&mut collider);
        self
    }
    pub fn create(self, world: &mut World) -> WorldCharacter {
        if self.is_player {
            let mut camera = Object {
                collider: vec![],
                static_body: true,
                sheet: None,
                state: ObjectState::new(),
                callbacks: Some(Callbacks::new()),
            };
            camera
                .callbacks
                .as_mut()
                .unwrap()
                .set(crate::objs::EventName::Tick, |_, args| {
                    let world = args.world;
                    let obj_r = args.obj.unwrap();

                    let pos = world
                        .state
                        .get(ObjectStateKey::PlayerPartyMember)
                        .and_then(|v: ObjectRef| world.ctx.get_object(v).get_position())
                        .unwrap_or(Vec2::ZERO);

                    let obj = world.ctx.get_mut_object(obj_r);
                    obj.state.set(ObjectStateKey::Pos, pos);
                    // obj.state.set(ObjectStateKey::Pos, Vec2::ZERO);

                    crate::objs::EventResult::Default
                });

            world.camera_obj = world.ctx.add_obj("zeta_camera".to_string(), camera);

            world.extra_objs.push(world.camera_obj);
        }

        static PM_NUM: AtomicUsize = AtomicUsize::new(0);
        let up = world.ctx.add_action("zeta_player_up".to_string());
        let down = world.ctx.add_action("zeta_player_down".to_string());
        let left = world.ctx.add_action("_zeta_player_left".to_string());
        let right = world.ctx.add_action("zeta_player_right".to_string());

        world.add_mapping(up, Key::Keyboard(KeyCode::Up));
        world.add_mapping(down, Key::Keyboard(KeyCode::Down));
        world.add_mapping(left, Key::Keyboard(KeyCode::Left));
        world.add_mapping(right, Key::Keyboard(KeyCode::Right));

        world.add_mapping(up, Key::Keyboard(KeyCode::W));
        world.add_mapping(down, Key::Keyboard(KeyCode::S));
        world.add_mapping(left, Key::Keyboard(KeyCode::A));
        world.add_mapping(right, Key::Keyboard(KeyCode::D));

        let mut obj = Object {
            sheet: Some(self.character_sheet),
            collider: self.collider,
            static_body: false,
            state: ObjectState::new(),
            callbacks: Some(Self::callbacks(self.is_player, self.is_lightner, up, down, left, right)),
        };

        obj.state.set(ObjectStateKey::Pos, Vec2::ZERO);
        obj.state.set(ObjectStateKey::Animation, "d".to_string());
        obj.state.set(ObjectStateKey::AniFrame, 0usize);
        obj.state.set(ObjectStateKey::Playing, false);
        obj.state.set(ObjectStateKey::Visible, true);
        obj.state.set(ObjectStateKey::Rotate, 0usize);
        obj.state
            .set(ObjectStateKey::Scale, Offset2 { x: 2.0, y: 2.0 });
        if let Some(party_member_i) = self.party_member_i {
            obj.state.set(ObjectStateKey::PartyMemberI, party_member_i);
        }

        let oref = world.ctx.add_obj(
            format!(
                "__WORLD_PARTY_MEMBER_{}",
                PM_NUM.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ),
            obj,
        );

        world.extra_objs.push(oref);

        WorldCharacter(oref, self.is_player)
    }
    fn callbacks(
        is_player: bool,
        is_lightner: bool,
        up: ActionRef,
        down: ActionRef,
        left: ActionRef,
        right: ActionRef,
    ) -> Callbacks {
        let mut out = Callbacks::new();
        if is_player {
            out.set(crate::objs::EventName::Tick, move |_, args| {
                let world = args.world;
                let obj_r = args.obj.unwrap();

                let vel = {
                    let mut vel = world.action_get_vec(up, down, left, right);
                    if vel == Offset2::ZERO {
                        vel = world.axis_get_vec(gilrs::Axis::LeftStickX, gilrs::Axis::LeftStickY);
                    }
                    if vel == Offset2::ZERO {
                        vel = world.axis_get_vec(gilrs::Axis::DPadX, gilrs::Axis::DPadY);
                    }

                    vel * 6.0
                };

                let obj = world.ctx.get_mut_object(obj_r);

                let current_dir = obj.state.get(ObjectStateKey::CurrentPlayerDir);
                let dir = vel.dir(current_dir).unwrap_or(Direction::Down);
                obj.state.set(ObjectStateKey::CurrentPlayerDir, dir);

                obj.state.set(
                    ObjectStateKey::Animation,
                    format!(
                        "{}{}",
                        dir.to_char(),
                        if (!world.state.get(ObjectStateKey::IsLight).unwrap_or(true))
                            && is_lightner
                        {
                            "_dark"
                        } else {
                            ""
                        }
                    ),
                );

                if vel != Offset2::ZERO {
                    obj.state.set(ObjectStateKey::Playing, true);
                } else {
                    obj.state.set(ObjectStateKey::Playing, false);
                    obj.state.set(ObjectStateKey::AniFrame, 0usize);
                }

                if let Some(pos) = obj.state.get_mut::<Vec2>(ObjectStateKey::Pos) {
                    *pos += vel;
                }

                crate::objs::EventResult::Default
            });
        } else {
            out.set(crate::objs::EventName::Tick, move |_, args| {
                let world = args.world;
                let obj_r = args.obj.unwrap();

                let vel = {
                    let mut vel = world.action_get_vec(up, down, left, right);
                    if vel == Offset2::ZERO {
                        vel = world.axis_get_vec(gilrs::Axis::LeftStickX, gilrs::Axis::LeftStickY);
                    }
                    if vel == Offset2::ZERO {
                        vel = world.axis_get_vec(gilrs::Axis::DPadX, gilrs::Axis::DPadY);
                    }

                    vel * 6.0
                };

                let obj = world.ctx.get_mut_object(obj_r);

                let current_dir = obj.state.get(ObjectStateKey::CurrentPlayerDir);
                let dir = vel.dir(current_dir).unwrap_or(Direction::Down);
                obj.state.set(ObjectStateKey::CurrentPlayerDir, dir);

                obj.state
                    .set(ObjectStateKey::Animation, dir.to_char().to_string());

                if vel != Offset2::ZERO {
                    obj.state.set(ObjectStateKey::Playing, true);
                } else {
                    obj.state.set(ObjectStateKey::Playing, false);
                    obj.state.set(ObjectStateKey::AniFrame, 0usize);
                }

                if let Some(pos) = obj.state.get_mut::<Vec2>(ObjectStateKey::Pos) {
                    *pos += vel;
                }

                crate::objs::EventResult::Default
            });
        }
        out
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WorldCharacter(ObjectRef, bool);

impl WorldCharacter {
    pub fn build(character_sheet: AniSheetRef, is_lightner: bool) -> WorldCharacterBuilder {
        WorldCharacterBuilder::new(character_sheet, is_lightner)
    }
    /// For the namespace "deltarune_ch1", the name "kris", is_player = true,
    /// and is_lightner = true, this would:
    /// - load placeholders for
    ///   deltarune_ch1:spr_kris[udlr](_heart|_dark)?_[0-4]
    /// - create animations for each one
    /// - put them into an animation sheet with the appropriate names needed for
    ///   a WorldPartyMember
    /// - return that sheet
    pub fn set_sprite_placeholders_deltarune(
        namespace: impl Display,
        name: impl Display,
        ctx: &mut Ctx,
        is_player: bool,
        is_lightner: bool,
    ) -> (AniSheetRef, HashMap<String, HashSet<usize>>) {
        let mut anis = Vec::new();
        let mut sprites_to_load = HashMap::new();
        for dir in Direction::ALL {
            let mut suffixes = Vec::with_capacity(3);
            suffixes.push("");
            if is_player {
                suffixes.push("_heart");
            }
            if is_lightner {
                suffixes.push("_dark");
            }
            for suffix in suffixes {
                let mut sprites = vec![];

                let base_id = format!("spr_{name}{}{suffix}", dir.to_char());
                let mut set = HashSet::new();

                for i in 0usize..4 {
                    sprites.push(ctx.add_placeholder_sprite(format!("{namespace}:{base_id}_{i}")));
                    set.insert(i);
                }
                anis.push((
                    format!("{name}_{}{suffix}", dir.to_char()),
                    format!("{}{suffix}", dir.to_char()),
                    Animation {
                        timeline: sprites
                            .into_iter()
                            .map(|v| AniEvent::Sprite {
                                sprite: v,
                                frame_count: NonZeroU8::new(1).unwrap(),
                            })
                            .collect(),
                        fps: 4, // placeholder, should figure out actual value
                        loops: true,
                    },
                ));
                sprites_to_load.insert(base_id, set);
            }
        }

        let sheet = AniSheet {
            anis: anis
                .into_iter()
                .map(|v| (v.1, ctx.add_ani(v.0, v.2)))
                .collect::<HashMap<_, _>>(),
        };

        static PM_NUM: AtomicUsize = AtomicUsize::new(0);
        (
            ctx.add_sheet(
                format!(
                    "__WORLD_PARTY_MEMBER_SHEET_{}",
                    PM_NUM.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                ),
                sheet,
            ),
            sprites_to_load,
        )
    }
    /// Add this member to the party. No animation will play of the player party
    /// member switching if that happens!
    pub fn add_to_party(self, world: &mut World) -> Self {
        match world
            .state
            .get_mut::<Vec<StateData>>(ObjectStateKey::PartyMembers)
        {
            Some(v) => {
                v.push(self.0.into());
            }
            None => {
                world.state.set(ObjectStateKey::PartyMembers, vec![self.0]);
            }
        };
        if self.1 {
            world.state.set(ObjectStateKey::PlayerPartyMember, self.0);
        }

        self
    }
}

impl Deref for WorldCharacter {
    type Target = ObjectRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
