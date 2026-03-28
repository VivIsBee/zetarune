use std::collections::VecDeque;

use crate::{
    components::battles::BattleAction,
    ctx::{ActionRef, AudioRef, FontRef, LocalTextRef, ObjectRef, SpriteRef},
    objs::{
        Callbacks, Color, DialogueItemOnScreen, DisplayedText, Object, ObjectState, Offset2, Vec2,
        World,
    },
    rt::KeyCode,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Face {
    Deltarune {
        character: String,
        /// iirc can go between 0 and 16 with hex
        emote: u8,
    },
    Other {
        character: String,
        emote: String,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub struct TyperSettings {
    pub font: FontRef,
    /// How long one character takes in frames.
    pub speed: usize,
    pub default_color: Color,
    pub scale: Offset2,
    pub voice: Option<AudioRef>,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum TextItem {
    /// Newlines are interpreted properly
    Text(LocalTextRef),
    SetFace(Face),
    RemoveFace,
    PauseFrames(usize),
    /// Disable the user pressing X to skip through dialogue
    DisableSkip,
    /// See [`DisableSkip`](TextItem::DisableSkip).
    EnableSkip,
    /// Wait until the user presses Z/the interact button.
    WaitForUser,
    /// out-of-the-box, has:
    /// - `susie`
    /// - `ralsei`
    /// - `noelle`
    /// - `toriel`
    /// - `lancer`
    /// - `sans`
    /// - `undyne`
    InsertMiniFace(String),
    SetTyper(TyperSettings),
    /// Shows the primary key (first key in the list) of an action
    ShowPrimaryActionKey(ActionRef),
    SetColor(Color),
    ResetColor,
    /// Triggers a DialogueEvent to the specified object.
    TriggerEvent(ObjectRef, usize),
    /// Sprite should be one line in size or less. Checked in debug builds.
    InsertSprite(SpriteRef),
    InsertBattleAction(BattleAction),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Terminator {
    Default,
    Choice {
        obj: ObjectRef,
        choice_id: String,
        choices: Vec<LocalTextRef>,
    },
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct DialogueItem {
    pub text: VecDeque<TextItem>,
    /// Only valid on a top-level DialogueItem.
    pub small_extra: Option<Box<DialogueItem>>,
}

#[derive(Clone, Copy, Debug)]
pub struct Dialoguer(ObjectRef);

impl Dialoguer {
    pub fn new(world: &mut World) -> Self {
        let interact = world.ctx.add_action("zeta_interact".to_string());
        let skip = world.ctx.add_action("zeta_skip".to_string());

        world.add_mapping(
            interact,
            crate::rt::Key::Keyboard(KeyCode::Z),
        );
        world.add_mapping(skip, crate::rt::Key::Keyboard(KeyCode::X));

        let mut obj = Object {
            collider: vec![],
            static_body: true,
            sheet: None,
            state: ObjectState::new(),
            callbacks: Some(Self::callbacks(interact, skip)),
        };

        let fref = world
            .ctx
            .add_placeholder_font("deltarune_ch1:fnt_comicsans".to_string());
        obj.state.set(
            crate::objs::ObjectStateKey::CurrentTyperSettings,
            TyperSettings {
                font: fref,
                speed: 5,
                default_color: Color::WHITE,
                scale: Offset2::ONE,
                voice: None,
            },
        );

        let oref = world.ctx.add_obj("zeta_dialoguer".to_string(), obj);
        world.extra_objs.push(oref);
        world.dialogue_queue = Some(VecDeque::new());
        Self(oref)
    }

    fn callbacks(interact: ActionRef, skip: ActionRef) -> Callbacks {
        let mut out = Callbacks::new();

        out.set(crate::objs::EventName::Tick, move |_, args| {
            let world = &mut *args.world;
            let obj_r = args.obj.unwrap();
            let mut obj = world.ctx.get_object(obj_r);
            if obj
                .state
                .get(crate::objs::ObjectStateKey::WaitingForUser)
                .unwrap_or(false)
            {
                if !world.action_down(interact) {
                    return crate::objs::EventResult::Default;
                }
                #[allow(
                    dropping_references,
                    reason = "borrow checker automatically sets lifetime as just above the get_mut_object, but this helps show it"
                )]
                drop(obj);

                world
                    .ctx
                    .get_mut_object(obj_r)
                    .state
                    .set(crate::objs::ObjectStateKey::WaitingForUser, false);

                obj = world.ctx.get_object(obj_r);
            }

            if let Some(text) = obj
                .state
                .get::<Option<String>>(crate::objs::ObjectStateKey::CurrentlyDialoguing)
                .unwrap_or(None) && !text.is_empty() && obj.state
                                .get(crate::objs::ObjectStateKey::CanSkip).unwrap_or(true) && world.action_down(skip) {
                match world.current_shown_dialogue_stuff.last_mut().unwrap() {
                    DialogueItemOnScreen::Text(out_text) => {

                        out_text.contents.push_str(&text);

                        world
                            .ctx
                            .get_mut_object(obj_r)
                            .state.set(crate::objs::ObjectStateKey::CurrentlyDialoguing, String::new());
                    },
                    _ => unreachable!()
                }
                return crate::objs::EventResult::Default;
            }

            if let Some(v) = obj
                .state
                .get::<usize>(crate::objs::ObjectStateKey::PauseFrames) && v > 0
            {
                println!("pausing for {v} frames");
                world
                    .ctx
                    .get_mut_object(obj_r)
                    .state
                    .set(crate::objs::ObjectStateKey::PauseFrames, v - 1);
            } else if let Some(mut text) = obj
                .state
                .get::<Option<String>>(crate::objs::ObjectStateKey::CurrentlyDialoguing)
                .unwrap_or(None) && !text.is_empty()
            {
                match world.current_shown_dialogue_stuff.last_mut().unwrap() {
                    DialogueItemOnScreen::Text(out_text) => {
                        println!("{out_text:?} (adding from {text})");
                        let typer = obj
                            .state
                            .get::<TyperSettings>(
                                crate::objs::ObjectStateKey::CurrentTyperSettings,
                            )
                            .unwrap();

                        out_text.contents.push(text.remove(0));

                        world
                            .ctx
                            .get_mut_object(obj_r)
                            .state.set(crate::objs::ObjectStateKey::CurrentlyDialoguing, Some(&text));

                        world
                            .ctx
                            .get_mut_object(obj_r)
                            .state.set(crate::objs::ObjectStateKey::PauseFrames, typer.speed);
                    },
                    _ => unreachable!()
                }
            } else {
                if let Some(current) = world.dialogue_queue.as_mut().unwrap().front_mut() {
                    if let Some(next) = current.text.pop_front() {
                        match next {
                            TextItem::Text(key) => {
                                let text = world.ctx.get_lang(world.lang).strings[&key].clone();

                                let mut dialogue_loc = obj
                                    .state
                                    .get(crate::objs::ObjectStateKey::CurrentDialogueLoc)
                                    .unwrap_or(Vec2 { x: 5.0, y: (world.screen_height() / 6.0) * 5.0 });
                                let typer = obj
                                    .state
                                    .get::<TyperSettings>(
                                        crate::objs::ObjectStateKey::CurrentTyperSettings,
                                    )
                                    .unwrap();

                                let width = world.ctx.get_font(typer.font).width(&text) as f32;

                                world.ctx.get_mut_object(obj_r).state.set(
                                    crate::objs::ObjectStateKey::CurrentlyDialoguing,
                                    Some(text),
                                );

                                world.current_shown_dialogue_stuff.push(crate::objs::DialogueItemOnScreen::Text(DisplayedText {
                                    contents: String::new(),
                                    loc: dialogue_loc,
                                    char_rot: 0.0,
                                    font: typer.font,
                                    scale: typer.scale
                                }));

                                dialogue_loc.x += width;

                                world.ctx.get_mut_object(obj_r).state.set(crate::objs::ObjectStateKey::CurrentDialogueLoc, dialogue_loc);
                            }
                            TextItem::SetFace(face) => world
                                .ctx
                                .get_mut_object(obj_r)
                                .state
                                .set(crate::objs::ObjectStateKey::CurrentFace, Some(face)),
                            TextItem::RemoveFace => {
                                world.ctx.get_mut_object(obj_r).state.set::<Option<Face>>(
                                    crate::objs::ObjectStateKey::CurrentFace,
                                    None,
                                )
                            }
                            TextItem::EnableSkip => world
                                .ctx
                                .get_mut_object(obj_r)
                                .state
                                .set(crate::objs::ObjectStateKey::CanSkip, true),
                            TextItem::DisableSkip => world
                                .ctx
                                .get_mut_object(obj_r)
                                .state
                                .set(crate::objs::ObjectStateKey::CanSkip, false),
                            TextItem::PauseFrames(frames) => world
                                .ctx
                                .get_mut_object(obj_r)
                                .state
                                .set(crate::objs::ObjectStateKey::PauseFrames, frames),
                            TextItem::InsertBattleAction(act) => todo!(),
                            TextItem::InsertMiniFace(face) => todo!(),
                            TextItem::InsertSprite(spr_r) => {
                                let font = obj
                                    .state
                                    .get::<TyperSettings>(
                                        crate::objs::ObjectStateKey::CurrentTyperSettings,
                                    )
                                    .unwrap()
                                    .font;
                                let mut dialogue_loc = obj
                                    .state
                                    .get(crate::objs::ObjectStateKey::CurrentDialogueLoc)
                                    .unwrap_or(Vec2::ZERO);

                                let spr = world.ctx.get_sprite(spr_r);

                                debug_assert!(spr.height <= world.ctx.get_font(font).line_height);
                                world.current_shown_dialogue_stuff.push(crate::objs::DialogueItemOnScreen::Sprite { sprite: spr_r, location: dialogue_loc, scale: Offset2::ONE });

                                dialogue_loc.x += spr.width as f32;
                                world.ctx.get_mut_object(obj_r).state.set(
                                    crate::objs::ObjectStateKey::CurrentDialogueLoc,
                                    dialogue_loc,
                                );
                            }
                            TextItem::ResetColor => {
                                let c = obj
                                    .state
                                    .get::<TyperSettings>(
                                        crate::objs::ObjectStateKey::CurrentTyperSettings,
                                    )
                                    .unwrap()
                                    .default_color;
                                world
                                    .ctx
                                    .get_mut_object(obj_r)
                                    .state
                                    .set(crate::objs::ObjectStateKey::DialogueColor, c)
                            }
                            TextItem::SetColor(c) => world
                                .ctx
                                .get_mut_object(obj_r)
                                .state
                                .set(crate::objs::ObjectStateKey::DialogueColor, c),
                            TextItem::SetTyper(ty) => world
                                .ctx
                                .get_mut_object(obj_r)
                                .state
                                .set(crate::objs::ObjectStateKey::CurrentTyperSettings, ty),
                            TextItem::ShowPrimaryActionKey(act) => todo!(),
                            TextItem::TriggerEvent(oref, meta) => world.post_event(
                                crate::rt::EventTarget::Object(oref, Some(world.current_room)),
                                move || crate::objs::Event::DialogueEvent { meta },
                            ),
                            TextItem::WaitForUser => world
                                .ctx
                                .get_mut_object(obj_r)
                                .state
                                .set(crate::objs::ObjectStateKey::WaitingForUser, true),
                        }
                    } else {
                        world.dialogue_queue.as_mut().unwrap().pop_front();
                    }
                }
            }

            crate::objs::EventResult::Default
        });
        out.set(crate::objs::EventName::Render, |ev, args| {
            let ev = ev.unwrap_Render();
            let drawer = ev.1;
            let world = &mut *args.world;

            for item in &world.current_shown_dialogue_stuff {
                match item {
                    DialogueItemOnScreen::Sprite {
                        sprite,
                        location,
                        scale,
                    } => drawer.lock().unwrap().draw_sprite_screen(
                        world.ctx.get_sprite(*sprite).clone(),
                        *scale,
                        *location,
                        0.0,
                    ),
                    DialogueItemOnScreen::Text(text) => drawer.lock().unwrap().draw_text(&text),
                }
            }

            crate::objs::EventResult::DisableDefault
        });

        out
    }
}