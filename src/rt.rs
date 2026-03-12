//! The actual runtime that plays a game.

use std::{
    collections::HashSet,
    fmt::Debug,
    mem::ManuallyDrop,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use macroquad::{
    color::WHITE,
    conf::Conf,
    input,
    texture::{DrawTextureParams, Image, Texture2D, draw_texture_ex},
    window::{next_frame, screen_height, screen_width},
};

use crate::{
    ctx::{AudioRef, EntryRef, ObjectRef, RoomRef},
    error,
    objs::{
        AniEvent, DisplayedText, Event, EventArgs, EventResult, Font, LanguageData, ObjectStateKey,
        Offset2, SealedRoomTransition, Sprite, Vec2, World,
    },
    warn,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Keyboard(macroquad::input::KeyCode),
    Gamepad(gilrs::Button),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InputState {
    NewlyPressed,
    /// Not emitted for gamepad buttons!
    NewlyReleased,
    Pressed,
    Released,
}

pub fn main(window_title: impl ToString, resizable: bool, world: World) -> ! {
    macroquad::Window::from_config(
        Conf {
            miniquad_conf: macroquad::window::Conf {
                window_title: window_title.to_string(),
                window_resizable: resizable,
                ..Default::default()
            },
            ..Default::default()
        },
        (move || async move {
            let mut timer = Duration::ZERO;
            let mut last_t = Instant::now();
            let mut world = world;
            let gilrs = gilrs::Gilrs::new().expect("failed to initalize gilrs");

            loop {
                let start_t = Instant::now();

                let mut new_pressed = HashSet::new();

                for key in input::get_keys_pressed() {
                    new_pressed.insert(key);
                    world.post_event(
                        EventTarget::All,
                        Box::new(move || Event::KeyPress {
                            key: Key::Keyboard(key),
                        }) as Box<_>,
                    );
                }
                for key in input::get_keys_released() {
                    world.post_event(
                        EventTarget::All,
                        Box::new(move || Event::KeyRelease {
                            key: Key::Keyboard(key),
                        }) as Box<_>,
                    );
                }
                for key in input::get_keys_down() {
                    if new_pressed.contains(&key) {
                        continue;
                    }
                    world.post_event(
                        EventTarget::All,
                        Box::new(move || Event::KeyHold {
                            key: Key::Keyboard(key),
                        }) as Box<_>,
                    );
                }

                world.current_frame_presses.clear();

                for (key, actions) in &world.input_mappings {
                    let input_state = match key {
                        Key::Keyboard(key) => {
                            if input::is_key_pressed(*key) {
                                InputState::NewlyPressed
                            } else if input::is_key_down(*key) {
                                InputState::Pressed
                            } else if input::is_key_released(*key) {
                                InputState::NewlyReleased
                            } else {
                                InputState::Released
                            }
                        }
                        Key::Gamepad(btn) => {
                            let mut state = InputState::Released;
                            for (_, gamepad) in gilrs.gamepads() {
                                if gamepad.button_data(*btn).unwrap().is_repeating() {
                                    state = InputState::Pressed;
                                } else if gamepad.is_pressed(*btn) {
                                    state = InputState::NewlyPressed;
                                }
                            }
                            state
                        }
                    };

                    for action in actions {
                        world.current_frame_presses.insert(*action, input_state);
                    }
                }

                frame(
                    &mut timer,
                    Instant::now().duration_since(last_t),
                    &mut world,
                )
                .await;

                last_t = Instant::now();

                let end_t = Instant::now();

                let dur = end_t.duration_since(start_t);

                println!("{} FPS ({dur:?} per frame)", 1.0 / dur.as_secs_f64());

                next_frame().await;
            }
        })(),
    );
    std::process::exit(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventTarget {
    Object(ObjectRef, Option<RoomRef>),
    Room(RoomRef),
    World,
    All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum InternalEvent {
    PlayAudio(AudioRef),
    PauseAudio(AudioRef),
    StopAudio(AudioRef),
    RoomTransition(EntryRef),
    /// Save all data to the provided save index.
    Save(u16),
    /// Load all data from the provided save index.
    Load(u16),
}

fn send_event_all<'a, F: FnMut() -> Event<'a> + 'a>(
    mut event_producer: F,
    world: &mut World,
    objs: &[(ObjectRef, Option<RoomRef>)],
    world_callback: impl FnOnce(&mut World, Option<EventResult>),
    room_callback: impl FnOnce(&mut World, Option<EventResult>, RoomRef),
    mut object_callback: impl FnMut(&mut World, Option<EventResult>, ObjectRef),
) {
    if let Some(callbacks) = world.callbacks.clone() {
        let res = callbacks.trigger(
            event_producer(),
            EventArgs {
                room: None,
                obj: None,
                world: world,
            },
        );
        world_callback(world, res);
    } else {
        world_callback(world, None);
    }

    if let Some(callbacks) = world.ctx.get_room(world.current_room).callbacks.clone() {
        let res = callbacks.trigger(
            event_producer(),
            EventArgs {
                room: Some(world.current_room),
                obj: None,
                world: world,
            },
        );
        room_callback(world, res, world.current_room);
    } else {
        room_callback(world, None, world.current_room);
    }

    for (obj_r, room) in objs {
        let obj = world.ctx.get_object(*obj_r);

        if let Some(callbacks) = obj.callbacks.clone() {
            let res = callbacks.trigger(
                event_producer(),
                EventArgs {
                    room: *room,
                    obj: Some(*obj_r),
                    world: world,
                },
            );
            object_callback(world, res, *obj_r);
        } else {
            object_callback(world, None, *obj_r);
        }
    }
}

pub fn get_data_path(world: &World) -> PathBuf {
    dirs::config_dir()
        .or_else(std::env::home_dir)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or(PathBuf::from("."))
        .join(&world.game_id)
}

pub fn get_save_path(world: &World, i: u16) -> PathBuf {
    let data_path = get_data_path(world);

    data_path.join(&format!("save-{i:05}.zrsav"))
}

async fn frame(timer: &mut Duration, delta: Duration, world: &mut World) {
    let screen_size = Offset2 {
        x: screen_width(),
        y: screen_height(),
    };

    for ev in world.internal_event_queue.drain(..).collect::<Vec<_>>() {
        match ev {
            InternalEvent::PlayAudio(aref) => {
                let audio = world.ctx.get_audio(aref);

                if let Some(player) = &audio.player {
                    player.play();
                } else if audio.source.is_some() {
                    let audio = world.ctx.get_mut_audio(aref);

                    let source = audio.source.take().unwrap();

                    let player = rodio::SpatialPlayer::connect_new(
                        &world.audio_handle.mixer(),
                        [audio.loc.0, audio.loc.1, audio.loc.2],
                        [-0.1, 0.0, 0.0],
                        [0.1, 0.0, 0.0],
                    );
                    player.append(source);
                    player.play();

                    audio.player = Some(player);
                } else {
                    error!("attempted to play audio after being stopped");
                }
            }
            InternalEvent::PauseAudio(aref) => {
                let audio = world.ctx.get_audio(aref);

                if let Some(player) = &audio.player {
                    player.pause();
                } else if audio.source.is_some() {
                    warn!("audio playback has not been started");
                } else {
                    warn!("attempted to pause audio after being stopped");
                }
            }
            InternalEvent::StopAudio(aref) => {
                let audio = world.ctx.get_audio(aref);

                if let Some(player) = &audio.player {
                    player.stop();
                    let audio = world.ctx.get_mut_audio(aref);
                    audio.player = None;
                } else if audio.source.is_some() {
                    warn!("attempted to stop audio before starting it; audio may not be started");

                    let audio = world.ctx.get_mut_audio(aref);

                    audio.source.take();
                } else {
                    warn!("attempted to stop audio despite being already stopped");
                }
            }
            InternalEvent::RoomTransition(entry) => {
                if let Some(trans) = world.room_transition {
                    let from = world.current_room;
                    let to = *world.ctx.get_entry_room(entry);
                    world.post_event(
                        EventTarget::Object(trans, Some(world.current_room)),
                        move || Event::RoomTransition {
                            _sealed: SealedRoomTransition,
                            from,
                            to,
                            entry,
                        },
                    );
                }
                for oref in &world.ctx.get_room(world.current_room).objects {
                    let oref = *oref;
                    world.event_queue.push((
                        EventTarget::Object(oref, Some(world.current_room)),
                        Box::new(|| Event::Unload),
                    ));
                }

                world.current_room = *world.ctx.get_entry_room(entry);

                let room = world.ctx.get_room(world.current_room);
                let entry_loc = room.entrypoints[&entry];

                for oref in &world.player {
                    let oref = *oref;
                    world
                        .ctx
                        .get_mut_object(oref)
                        .state
                        .set(ObjectStateKey::Pos, entry_loc);
                }

                for oref in &world.ctx.get_room(world.current_room).objects {
                    let oref = *oref;
                    world.event_queue.push((
                        EventTarget::Object(oref, Some(world.current_room)),
                        Box::new(|| Event::Load),
                    ));
                }
            }
            InternalEvent::Save(i) => {
                let loc = get_save_path(&*world, i);

                let bytes = world.save_to_bytes();

                std::fs::write(loc, bytes).unwrap();
            }
            InternalEvent::Load(i) => {
                let loc = get_save_path(&*world, i);

                if !loc.exists() {
                    panic!(
                        "Attempted to load non-existant save; check for existence before attempting to load with `world.save_exists`"
                    )
                }

                let bytes = std::fs::read(loc).unwrap();

                world.load_from_bytes(&bytes);
            }
        }
    }

    let mut objs = world
        .extra_objs
        .iter()
        .map(|v| (*v, None))
        .collect::<Vec<_>>();

    objs.append(
        &mut world
            .ctx
            .get_room(world.current_room)
            .objects
            .iter()
            .map(|v| (*v, Some(world.current_room)))
            .collect::<Vec<_>>(),
    );

    objs.sort_by_cached_key(|v| world.ctx.get_object(v.0).get_z_layer().unwrap_or(0));

    objs.reverse();

    let mut collided = HashSet::new(); // needed to prevent double-reporting each collision
    for (obj1, room1) in &objs {
        let obj1_colliders = &world.ctx.get_object(*obj1).collider;
        let obj1_pos = if let Some(pos) = world.ctx.get_object(*obj1).get_position() {
            pos
        } else {
            continue;
        };

        for (obj2, room2) in &objs {
            let mut objs = [*obj1, *obj2];
            objs.sort(); // needed to prevent double-reporting each collision
            if *obj1 == *obj2 || collided.contains(&objs) {
                continue;
            }

            let obj2_colliders = &world.ctx.get_object(*obj2).collider;
            let obj2_pos = if let Some(pos) = world.ctx.get_object(*obj2).get_position() {
                pos
            } else {
                continue;
            };

            'collider_loop1: for collider1 in obj1_colliders {
                for collider2 in obj2_colliders {
                    if collider1.overlapping_with(obj1_pos, *collider2, obj2_pos) {
                        let obj1 = *obj1;
                        let obj2 = *obj2;

                        world.event_queue.push((
                            EventTarget::Object(obj1, *room1),
                            Box::new(move || Event::Collide { other: obj2 }),
                        ));
                        world.event_queue.push((
                            EventTarget::Object(obj2, *room2),
                            Box::new(move || Event::Collide { other: obj1 }),
                        ));
                        collided.insert(objs);

                        break 'collider_loop1;
                    }
                }
            }
        }
    }

    if timer.as_millis() % (1000 / 20) == 0 {
        send_event_all(
            || Event::Tick(delta),
            world,
            &objs,
            |_, _| {},
            |_, _, _| {},
            |_, _, _| {},
        );
    }

    for (target, mut event) in world.event_queue.drain(..).collect::<Vec<_>>() {
        match target {
            EventTarget::Object(obj_r, room) => {
                let ev = event();

                let is_ani_continue = matches!(ev, Event::AniContinueEvent);
                let is_collision = match ev {
                    Event::Collide { other } => Some(other),
                    _ => None,
                };

                let res = if let Some(callbacks) = world.ctx.get_object(obj_r).callbacks.clone() {
                    callbacks.trigger(
                        ev,
                        EventArgs {
                            room: room,
                            obj: Some(obj_r),
                            world,
                        },
                    )
                } else {
                    None
                };

                if is_ani_continue && res != Some(EventResult::DisableDefault) {
                    let obj = world.ctx.get_object(obj_r);
                    if let Some(ani_sheet_ref) = obj.sheet
                        && obj.is_processing()
                    {
                        let sheet = world.ctx.get_sheet(ani_sheet_ref);
                        if let Some(ani) = obj.get_ani() {
                            let ani = world.ctx.get_animation(sheet.anis[&ani]);

                            let frame = obj.get_frame().unwrap_or(0);
                            if matches!(&ani.timeline[frame], AniEvent::PausePoint) {
                                world
                                    .ctx
                                    .get_mut_object(obj_r)
                                    .state
                                    .set(ObjectStateKey::AniFrame, frame + 1);
                            }
                        }
                    }
                } else if let Some(other_r) = is_collision
                    && res != Some(EventResult::DisableDefault)
                {
                    let obj_static_body = world.ctx.get_object(obj_r).static_body;
                    let other_static_body = world.ctx.get_object(other_r).static_body;

                    let pos1 = if let Some(pos) = world.ctx.get_object(obj_r).get_position() {
                        pos
                    } else {
                        warn!(
                            "please don't remove the position of colliding objects and still return EventResult::Default - it fucks with default collision"
                        );
                        continue;
                    };
                    let pos2 = if let Some(pos) = world.ctx.get_object(other_r).get_position() {
                        pos
                    } else {
                        warn!(
                            "please don't remove the position of colliding objects and still return EventResult::Default - it fucks with default collision"
                        );
                        continue;
                    };

                    let mut offset = None;

                    'collider1_loop: for collider1 in &world.ctx.get_object(obj_r).collider {
                        for collider2 in &world.ctx.get_object(other_r).collider {
                            if let Some(overlap) = collider1.outside_overlap(pos1, *collider2, pos2)
                            {
                                offset = Some(overlap);
                                break 'collider1_loop;
                            }
                        }
                    }
                    if offset.is_none() {
                        warn!(
                            "please don't move colliding objects and still return EventResult::Default - it fucks with default collision"
                        );
                        continue;
                    }
                    let offset = offset.unwrap();

                    if (!obj_static_body) && (!other_static_body) {
                        world
                            .ctx
                            .get_mut_object(obj_r)
                            .state
                            .set(ObjectStateKey::Pos, pos1 - (offset / 2.0));
                        world
                            .ctx
                            .get_mut_object(other_r)
                            .state
                            .set(ObjectStateKey::Pos, pos2 + (offset / 2.0));
                    } else if obj_static_body && (!other_static_body) {
                        world
                            .ctx
                            .get_mut_object(other_r)
                            .state
                            .set(ObjectStateKey::Pos, pos2 + offset);
                    } else if (!obj_static_body) && other_static_body {
                        println!("{pos1:?}, {offset:?}, {:?}", pos1 + offset);
                        world
                            .ctx
                            .get_mut_object(obj_r)
                            .state
                            .set(ObjectStateKey::Pos, pos1 + offset);
                    } else {
                        println!("aaaa {obj_static_body}+{other_static_body}")
                    }
                }
            }
            EventTarget::Room(room) => {
                if let Some(callbacks) = world.ctx.get_room(room).callbacks.clone() {
                    let _ = callbacks.trigger(
                        event(),
                        EventArgs {
                            room: Some(room),
                            obj: None,
                            world,
                        },
                    );
                }
            }
            EventTarget::World => {
                if let Some(callbacks) = world.callbacks.clone() {
                    let _ = callbacks.trigger(
                        event(),
                        EventArgs {
                            room: None,
                            obj: None,
                            world,
                        },
                    );
                }
            }
            EventTarget::All => {
                send_event_all(event, world, &objs, |_, _| {}, |_, _, _| {}, |_, _, _| {});
            }
        }
    }

    let draw_ctx = Arc::new(Mutex::new(DrawContext::new(
        world
            .ctx
            .get_object(world.camera_obj)
            .get_position()
            .unwrap()
            - (screen_size / 2.0),
        screen_size,
        world
            .ctx
            .fonts
            .iter()
            .filter(|v| v.is_some())
            .cloned()
            .map(|v| v.unwrap())
            .collect(),
        world
            .ctx
            .sprites
            .iter()
            .filter(|v| v.is_some())
            .cloned()
            .map(|v| v.unwrap())
            .collect(),
        world.ctx.get_lang(world.lang).clone(),
    )));

    for text in &world.text {
        draw_ctx
            .lock()
            .unwrap()
            .draw_text(world.ctx.get_text(*text));
    }

    if world
        .ctx
        .get_object(world.camera_obj)
        .get_position()
        .is_none()
    {
        error!("camera doesn't have a position; skipping rendering sprites this frame");

        *timer += delta;
        return;
    }

    send_event_all(
        || Event::Render(delta, draw_ctx.clone()),
        world,
        &objs,
        |_, _| {},
        |world, res, _| match res {
            Some(EventResult::DisableDefault) => {}
            _ => {
                if let Some((sprite, pos)) = world.ctx.get_room(world.current_room).background {
                    draw_ctx.lock().unwrap().draw_sprite(
                        world.ctx.get_sprite(sprite).clone(),
                        pos,
                        0.0,
                    );
                }
            }
        },
        |world, res, obj_r| match res {
            Some(EventResult::DisableDefault) => {}
            _ => {
                let obj = world.ctx.get_object(obj_r);
                if let Some(ani_sheet_ref) = obj.sheet
                    && let Some(pos) = obj.get_position()
                    && obj.get_visible().unwrap_or(true)
                {
                    println!("{obj_r:?}: {pos:?}");

                    let sheet = world.ctx.get_sheet(ani_sheet_ref);
                    if let Some(ani) = obj.get_ani()
                        && let Some(frame) = obj.get_frame()
                    {
                        let maybe_ani = sheet.anis.get(&ani);
                        if maybe_ani.is_none() {
                            error!(
                                "referenced animation \"{}\" does not exist in object \"{}\"",
                                ani,
                                world.ctx.get_obj_id(obj_r)
                            );
                            return;
                        }
                        let timeline = &world.ctx.get_animation(*maybe_ani.unwrap()).timeline;

                        let sprite = match timeline[frame] {
                            AniEvent::PausePoint => {
                                let mut out_sprite = None;
                                for event in &timeline[..frame] {
                                    match event {
                                        AniEvent::PausePoint => {}
                                        AniEvent::Sprite {
                                            sprite,
                                            frame_count: _,
                                        } => {
                                            out_sprite = Some(*sprite);
                                        }
                                    }
                                }
                                out_sprite
                            }
                            AniEvent::Sprite {
                                sprite,
                                frame_count: _,
                            } => Some(sprite),
                        };

                        if let Some(sprite) = sprite {
                            draw_ctx.lock().unwrap().draw_sprite(
                                world.ctx.get_sprite(sprite).clone(),
                                pos,
                                obj.get_rotation().unwrap_or_default(),
                            );
                        }
                    }
                }
                if let Some(ani_sheet_ref) = obj.sheet
                    && obj.is_processing()
                    && obj.is_playing()
                {
                    let sheet = world.ctx.get_sheet(ani_sheet_ref);
                    if let Some(ani) = obj.get_ani() {
                        let ani = world.ctx.get_animation(sheet.anis[&ani]);

                        let mut frame_timer = obj
                            .state
                            .get(ObjectStateKey::AniFrameTimer)
                            .unwrap_or(Duration::ZERO);

                        let frame_count = match ani.timeline[obj.get_frame().unwrap_or(0)] {
                            AniEvent::Sprite {
                                sprite: _,
                                frame_count,
                            } => Some(frame_count.get()),
                            AniEvent::PausePoint => None,
                        };

                        if let Some(frame_count) = frame_count
                            && ani.fps > 0
                            && frame_timer.as_millis()
                                >= ((1000 / ani.fps as u128) * frame_count as u128)
                        {
                            let mut new_frame = obj.get_frame().map_or(0, |v| v + 1);

                            if new_frame >= ani.timeline.len() {
                                if ani.loops {
                                    new_frame = 0;
                                } else {
                                    new_frame = ani.timeline.len() - 1;
                                    world
                                        .ctx
                                        .get_mut_object(obj_r)
                                        .state
                                        .set(ObjectStateKey::Playing, false);
                                }
                            }

                            world
                                .ctx
                                .get_mut_object(obj_r)
                                .state
                                .set(ObjectStateKey::AniFrame, new_frame);

                            frame_timer = Duration::ZERO;
                        } else {
                            frame_timer += delta;
                        }
                        world
                            .ctx
                            .get_mut_object(obj_r)
                            .state
                            .set(ObjectStateKey::AniFrameTimer, frame_timer);
                    }
                }
            }
        },
    );

    *timer += delta;
}

/// A context needed to draw the screen. Technically could be bypassed, but
/// please don't :c
///
/// Everything is in world coordinates, except for specific scenarios like text.
pub struct DrawContext(Vec2, Offset2, Vec<Font>, Vec<Sprite>, LanguageData);

impl DrawContext {
    fn new(
        camera_pos: Vec2,
        screen_size: Offset2,
        fonts: Vec<Font>,
        sprites: Vec<Sprite>,
        lang: LanguageData,
    ) -> Self {
        Self(camera_pos, screen_size, fonts, sprites, lang)
    }
    pub fn draw_sprite(&mut self, sprite: Sprite, pos: Vec2, rot: f32) {
        self.draw_sprite_screen(sprite, (pos - self.0).into(), rot);
    }
    fn draw_sprite_screen(&self, sprite: Sprite, top_left: Vec2, rot: f32) {
        let bottom_right = top_left + sprite.get_size();

        if bottom_right < Vec2::ZERO || top_left > self.1.into() {
            return;
        }

        let center = top_left - (sprite.get_size() / 2.0);

        let mut data = ManuallyDrop::new(sprite.data.clone());

        let len = data.len();
        let cap = data.capacity();
        let ptr = data.as_mut_ptr();

        let tex = Texture2D::from_image(&Image {
            width: sprite.width,
            height: sprite.height,
            bytes: unsafe { Vec::from_raw_parts(ptr as *mut u8, len * 4, cap * 4) }, /* SAFETY: Color is repr(C) */
        });

        draw_texture_ex(
            &tex,
            center.x,
            center.y,
            WHITE,
            DrawTextureParams {
                rotation: rot,
                pivot: Some(macroquad::prelude::Vec2::ZERO),
                ..Default::default()
            },
        );
    }
    pub fn draw_text(&mut self, text: &DisplayedText) {
        let mut pos = Offset2::ZERO;
        let font = &self.2[text.font.index];
        for ch in self.4.strings[&text.contents].chars() {
            match ch {
                '\r' => {}
                '\n' => {
                    pos.x = 0.0;
                    pos.y += font.line_height as f32;
                }
                _ => {
                    let (sprite_r, x_off) = font.sprites[font.char_index_map[&ch]];
                    let sprite = self.3[sprite_r.index].clone();
                    self.draw_sprite_screen(sprite, text.loc + pos, text.char_rot);

                    pos.x += x_off as f32;
                }
            }
        }
    }
}
