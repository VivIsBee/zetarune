//! Engine interner.

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::objs::*;

macro_rules! ctx_get_borrowed_or_owned {
    (borrowed mut $unwrapped_t:ty $(: ($owned_t:ty))? $(: $default:ident)?) => {
        &mut $unwrapped_t
    };
    (borrowed $unwrapped_t:ty $(: ($owned_t:ty))? $(: $default:ident)?) => {
        &$unwrapped_t
    };
    (owned $unwrapped_t:ty: default) => {
        <$unwrapped_t as ToOwned>::Owned
    };
    (owned $unwrapped_t:ty: ($owned_t:ty): default) => {
        compile_error!("cannot both specify owned type and use default owned type");
    };
    (owned $unwrapped_t:ty: $default:ident) => {
        compile_error!(concat!("unknown ident ", stringify!($default)));
    };
    (owned $unwrapped_t:ty: ($owned_t:ty)) => {
        $owned_t
    };
    (owned $unwrapped_t:ty) => {
        $unwrapped_t
    };
}

macro_rules! ctx_impl_default_ref {
    (default , ; $name:ident $prop:ident $(, $($t:ty),+)?) => {
        paste::paste! {
            impl $name {
                pub fn default(ctx: &mut Ctx) -> Self {
                    static ID_NUM: AtomicUsize = AtomicUsize::new(0);

                    ctx.[< add_ $prop >] (
                        format!(
                            concat!("__", stringify!($prop), "_{}"),
                            ID_NUM.fetch_add(1, Ordering::Relaxed)
                        ),
                        $($(<$t as Default>::default()),+)?
                    )
                }
            }
        }
    };
    ($default_impl:ident , ; $name:ident $v:ident $(, $($v2:ty),+)?) => {
        compile_error!(concat!("unknown ident ", stringify!($default_impl)));
    };
    (; $name:ident $v:ident $(, $($v2:ty),+)?) => {};
}

macro_rules! define_ctx {
    {
        $(
            $(#[$meta:meta])*
            $name:ident $($default_impl:ident)?: $prop:ident
            $(; $(
                $prop2:ident => $unwrapped_t2:ty $(: ($owned_t2:ty))? $(: $default:ident)?
            );+ $(;)?)?
        ),* $(,)?
    } => {
        paste::paste! {
            /// Interner for data throughout the engine.
            #[derive(Default, Debug)]
            pub struct Ctx {
                $(
                    pub(crate) [< $prop _ids >]:
                        Vec<Option<String>>,
                    pub(crate) [< $prop _id_map >]:
                        HashMap<String, usize>,
                    pub(crate) [< $prop _placeholder >]:
                        Vec<bool>
                    $(
                        , $(
                            pub(crate) [< $prop2 s >]:
                                Vec<Option<ctx_get_borrowed_or_owned!(owned $unwrapped_t2 $(: $owned_t2)? $(: $default)?)>>
                            // pub(crate) [< $prop2 _map >]:
                            //     HashMap<<$unwrapped_t2 as ToOwned>::Owned, usize>
                        ),*
                    )?
                ),*
            }
            $(
                $(#[$meta])*
                #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
                #[repr(transparent)]
                pub struct $name {
                    pub(crate) index: usize,
                }
                ctx_impl_default_ref!(
                    $($default_impl ,)? ;
                        $name $prop
                        $(, $(ctx_get_borrowed_or_owned!(owned $unwrapped_t2 $(: $owned_t2)? $(: $default)?)),+)?
                );
                impl Debug for $name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.debug_tuple(stringify!($name))
                            .field(&self.index)
                            .finish()
                    }
                }
            )*
            impl Ctx {
                /// Create a new, empty, [`Ctx`].
                pub fn new() -> Self {
                    Default::default()
                }
                $(
                    #[doc = concat!("Get and return the provided [`", stringify!($name),
                        "`]'s associated ID.")
                    ]
                    #[track_caller]
                    #[must_use]
                    pub fn [< get_ $prop _id >] (&self, val: $name) -> &str {
                        if self.[< $prop _ids >] [val.index].is_none() {
                            panic!("tried to use a dead reference");
                        }

                        self.[< $prop _ids >] [val.index].as_ref().unwrap()
                    }
                    #[doc = concat!("Get a reference to the `", stringify!($prop), "` with the provided ID, if one exists.")]
                    #[must_use]
                    pub fn [< get_ $prop _id_ref >] (&self, val: &str) -> Option<$name> {
                        self.[< $prop _id_map >].get(val).copied().map(|v| $name {
                            index: v
                        })
                    }
                    #[doc = concat!(
                        "REMOVE the provided `", stringify!($name), "` from this `Ctx`. All other references
                        to it are no longer valid and will panic if used, HOWEVER if not dropped they may later reference
                        a different `", stringify!($prop), "`!. This will never shrink the underlying data storage, however, and simply frees
                        up the provided slot in memory.")]
                    pub fn [< remove_ $prop >] (&mut self, v: $name) {
                        let id = core::mem::take(&mut self.[< $prop _ids >][v.index]).unwrap();
                        $(
                            $(
                                self.[< $prop2 s >][v.index] = None;
                            )+
                        )?
                        self.[< $prop _id_map >].remove(&id);
                    }
                    #[doc = concat!("Get a vec of all `", stringify!($prop), "`s in this context.")]
                    pub fn [< all_ $prop s >] (&self) -> Vec<$name> { // TODO: avoid this allocation (maybe impossible)
                        self.[< $prop _id_map >]
                            .values()
                            .copied()
                            .map(|v| $name { index: v })
                            .collect()
                    }
                    #[doc = concat!("Add a new `", stringify!($prop),
                        "` or find an already existing one with the same value
                        and return a [`", stringify!($name), "`] reference to it. ID CANNOT contain a `.`, or the function will panic.")]
                    #[track_caller]
                    pub fn [< add_ $prop >] (
                        &mut self,
                        id: String
                        $($(, $prop2: ctx_get_borrowed_or_owned!(owned $unwrapped_t2 $(: $owned_t2)? $(: $default)?))*)?
                    ) -> $name {
                        if id.contains('.') {
                            panic!("ID of object cannot contain .")
                        }
                        if !self.[< $prop _id_map >].contains_key(&id)  {
                            let mut i = self.[< $prop _ids>].len();

                            for (c_i, id) in self.[< $prop _ids >].iter().enumerate() {
                                if id.is_none() {
                                    i = c_i;
                                    break;
                                }
                            }

                            if i >= self.[< $prop _ids>].len() {
                                self.[< $prop _ids >].push(Some(id.clone()));
                                self.[< $prop _placeholder >].push(false);
                            } else {
                                self.[< $prop _ids >][i] = Some(id.clone());
                                self.[< $prop _placeholder >][i] = false;
                            }

                            self.[< $prop _id_map >]
                                .insert(id.clone(), i);
                            $(
                                $(
                                    if i >= self.[< $prop2 s>].len() {
                                        self.[< $prop2 s >].push(Some($prop2));
                                    } else {
                                        self.[< $prop2 s >][i] = Some($prop2);
                                    }
                                );*
                            )?
                        } else if self.[< $prop _placeholder >][self.[< $prop _id_map >][&id]] {
                            let i = self.[< $prop _id_map >][&id];
                            self.[< $prop _placeholder >][i] = false;
                            $(
                                $(
                                    self.[< $prop2 s >][i] = Some($prop2);
                                )*
                            )?
                        }
                        $name {
                            index: (*self.[< $prop _id_map >].get(&id).unwrap())
                        }
                    }
                    #[doc = concat!("Add a new `", stringify!($prop),
                        "` or find an already existing one with the same value
                        and return a [`", stringify!($name), "`] reference to it. ID CANNOT contain a `.`, or the function will panic.")]
                    #[track_caller]
                    pub fn [< add_placeholder_ $prop >] (
                        &mut self,
                        id: String
                    ) -> $name {
                        if id.contains('.') {
                            panic!("ID of object cannot contain .")
                        }
                        if !self.[< $prop _id_map >].contains_key(&id) {
                            let mut i = self.[< $prop _ids>].len();

                            for (c_i, id) in self.[< $prop _ids >].iter().enumerate() {
                                if id.is_none() {
                                    i = c_i;
                                    break;
                                }
                            }

                            if i >= self.[< $prop _ids>].len() {
                                self.[< $prop _ids >].push(Some(id.clone()));
                                self.[< $prop _placeholder >].push(true);
                            } else {
                                self.[< $prop _ids >][i] = Some(id.clone());
                                self.[< $prop _placeholder >][i] = true;
                            }

                            self.[< $prop _id_map >]
                                .insert(id.clone(), i);
                            $(
                                $(
                                    if i >= self.[< $prop2 s>].len() {
                                        self.[< $prop2 s >].push(None);
                                    } else {
                                        self.[< $prop2 s >][i] = None;
                                    }
                                );*
                            )?
                        }
                        $name {
                            index: (*self.[< $prop _id_map >].get(&id).unwrap())
                        }
                    }
                    $(
                        $(
                            #[doc = concat!("Get the provided [`", stringify!($name),
                                "`]'s associated value, returning a [`", stringify!(
                                    $unwrapped_t2
                                ), "`].")
                            ]
                            #[must_use]
                            #[track_caller]
                            pub fn [< get_ $prop2 >] (&self, val: $name) ->
                                ctx_get_borrowed_or_owned!(borrowed $unwrapped_t2 $(: $owned_t2)? $(: $default)?) {
                                if self.[< $prop2 s >] [val.index].is_none() {
                                    panic!("tried to use a dead or placeholder reference");
                                }
                                self.[< $prop2 s >] [val.index].as_ref().unwrap()
                            }
                            #[doc = concat!("Get the provided [`", stringify!($name),
                                "`]'s associated value, returning a mutable reference to the [`", stringify!(
                                    $unwrapped_t2
                                ), "`].")
                            ]
                            #[must_use]
                            #[track_caller]
                            pub fn [< get_mut_ $prop2 >] (&mut self, val: $name) ->
                                ctx_get_borrowed_or_owned!(borrowed mut $unwrapped_t2 $(: $owned_t2)? $(: $default)?) {
                                if self.[< $prop2 s >] [val.index].is_none() {
                                    panic!("tried to use a dead or placeholder reference");
                                }
                                self.[< $prop2 s >] [val.index].as_mut().unwrap()
                            }
                        )*
                    )?
                )*
            }
        }
    }
}

define_ctx! {
    SpriteRef default: sprite; sprite => Sprite,
    AnimationRef: ani; animation => Animation,
    AniSheetRef: sheet; sheet => AniSheet,
    ObjectRef default: obj; object => Object,
    RoomRef default: room; room => Room,
    ActionRef: action,
    AudioRef: audio; audio => Audio,
    FontRef: font; font => Font,
    /// An entrypoint to a room.
    EntryRef: entrypoint; entry_room => RoomRef,
    TextRef: text; text => DisplayedText,
    LocalTextRef: local_text,
    LanguageRef: lang; lang => LanguageData,
}
