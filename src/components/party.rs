use std::{ops::Deref, sync::atomic::AtomicUsize};

use macroquad::input::KeyCode;

use crate::{
    ctx::{ActionRef, AniSheetRef, ObjectRef},
    objs::{
        Callbacks, Collider, Object, ObjectState, ObjectStateKey, Offset2, StateData, Vec2, World,
    },
    rt::Key,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PartyMemberBuilder {
    pub is_player: bool,
    pub character_sheet: AniSheetRef,
    pub collider: Vec<Collider>,
}

impl PartyMemberBuilder {
    pub fn new(character_sheet: AniSheetRef) -> Self {
        Self {
            is_player: false,
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
    pub fn add_collider(mut self, mut collider: Vec<Collider>) -> Self {
        self.collider.append(&mut collider);
        self
    }
    pub fn create(self, world: &mut World) -> PartyMember {
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
            callbacks: Some(Self::callbacks(self.is_player, up, down, left, right)),
        };

        obj.state.set(ObjectStateKey::Pos, Vec2::ZERO);
        obj.state.set(ObjectStateKey::Animation, "idle".to_string());
        obj.state.set(ObjectStateKey::AniFrame, 0usize);
        obj.state.set(ObjectStateKey::Playing, true);
        obj.state.set(ObjectStateKey::Visible, true);
        obj.state.set(ObjectStateKey::Rotate, 0usize);
        obj.state.set(ObjectStateKey::Scale, Vec2 { x: 1.0, y: 1.0 });

        let oref = world.ctx.add_obj(
            format!(
                "__PARTY_MEMBER_{}",
                PM_NUM.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ),
            obj,
        );

        world.extra_objs.push(oref);

        PartyMember(oref, self.is_player)
    }
    fn callbacks(
        is_player: bool,
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

                    vel * 5.0
                };

                let obj = world.ctx.get_mut_object(obj_r);

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
pub struct PartyMember(ObjectRef, bool);

impl PartyMember {
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

impl Deref for PartyMember {
    type Target = ObjectRef;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
