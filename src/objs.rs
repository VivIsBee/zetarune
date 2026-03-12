//! Data model

use std::{
    collections::{self, HashMap, HashSet},
    fmt::{self, Debug, Display},
    hash::Hash,
    num::NonZeroU8,
    ops::{Index, IndexMut},
    sync::{Arc, Mutex},
    time::Duration,
};

use rodio::{Source, SpatialPlayer};
use serde::{
    Deserialize, Serialize,
    de::{DeserializeSeed, VariantAccess, Visitor},
    ser::SerializeTupleVariant as _,
};

use crate::{
    ctx::*,
    rt::{EventTarget, InputState, InternalEvent, Key},
};

macro_rules! impl_vec_ops1 {
    ($((($lhs:ty) ($op_t:ident [$f_name:ident $op:tt]) ($rhs:ty) $($assign:ident)? -> $out:ty)),* $(,)?) => {
        paste::paste! {
            $(
                impl std::ops::$op_t<$rhs> for $lhs {
                    type Output = $out;
                    fn $f_name (self, rhs: $rhs) -> $out {
                        $out {
                            x: self.x $op rhs.x,
                            y: self.y $op rhs.y,
                        }
                    }
                }

                $(
                    #[doc = stringify!($assign)]
                    impl std::ops::[<$op_t Assign>] <$rhs> for $lhs {
                        fn [<$f_name _assign>] (&mut self, rhs: $rhs) {
                            *self = $out {
                                x: self.x $op rhs.x,
                                y: self.y $op rhs.y,
                            };
                        }
                    }
                )?
            )*
        }
    };
}

macro_rules! impl_vec_ops2 {
    ($((($lhs:ty) ($op_t:ident [$f_name:ident $op:tt]) ($rhs:ty) $($assign:ident)? -> $out:ty)),* $(,)?) => {
        paste::paste! {
            $(
                impl std::ops::$op_t<$rhs> for $lhs {
                    type Output = $out;
                    fn $f_name (self, rhs: $rhs) -> $out {
                        $out {
                            x: self.x $op rhs,
                            y: self.y $op rhs,
                        }
                    }
                }

                $(
                    #[doc = stringify!($assign)]
                    impl std::ops::[<$op_t Assign>] <$rhs> for $lhs {
                        fn [<$f_name _assign>] (&mut self, rhs: $rhs) {
                            *self = $out {
                                x: self.x $op rhs,
                                y: self.y $op rhs,
                            };
                        }
                    }
                )?
            )*
        }
    };
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Offset2 {
    pub x: f32,
    pub y: f32,
}

impl Offset2 {
    pub const ZERO: Self = Offset2 { x: 0.0, y: 0.0 };
}

impl From<Offset2> for Vec2 {
    fn from(value: Offset2) -> Self {
        Vec2 {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Vec2 { x: 0.0, y: 0.0 };

    pub fn dist_sq(self, other: Vec2) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
    pub fn dist(self, other: Vec2) -> f32 {
        self.dist_sq(other).sqrt()
    }
    pub fn clamp(self, low: Vec2, high: Vec2) -> Vec2 {
        Vec2 {
            x: self.x.clamp(low.x, high.x),
            y: self.y.clamp(low.y, high.y),
        }
    }
}

impl Hash for Vec2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl Hash for Offset2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl_vec_ops1! (
    ((Offset2) (Add [add +]) (Offset2) assign -> Offset2),
    ((Offset2) (Sub [sub -]) (Offset2) assign -> Offset2),
    ((Offset2) (Mul [mul *]) (Offset2) assign -> Offset2),
    ((Offset2) (Div [div /]) (Offset2) assign -> Offset2),
    ((Offset2) (Rem [rem %]) (Offset2) assign -> Offset2),

    ((Vec2) (Add [add +]) (Vec2) assign -> Vec2),
    ((Vec2) (Sub [sub -]) (Vec2) -> Offset2),
    ((Vec2) (Mul [mul *]) (Vec2) assign -> Vec2),
    ((Vec2) (Div [div /]) (Vec2) assign -> Vec2),
    ((Vec2) (Rem [rem %]) (Vec2) assign -> Vec2),

    ((Vec2) (Add [add +]) (Offset2) assign -> Vec2),
    ((Vec2) (Sub [sub -]) (Offset2) assign -> Vec2),
);

impl_vec_ops2! (
    ((Offset2) (Div [div /]) (f32) assign -> Offset2),
    ((Offset2) (Mul [mul /]) (f32) assign -> Offset2),

    ((Vec2) (Div [div /]) (f32) assign -> Vec2),
    ((Vec2) (Mul [mul /]) (f32) assign -> Vec2),
);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColliderType {
    Rect { size: Offset2 },
    Circle { radius: f32 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Collider {
    pub t: ColliderType,
    pub off: Offset2,
}

impl Collider {
    pub fn overlapping_with(self, self_base_pos: Vec2, other: Self, other_base_pos: Vec2) -> bool {
        match (self.t, other.t) {
            (ColliderType::Rect { size: _ }, ColliderType::Rect { size: _ }) => {
                self.bounding_boxes_collide(self_base_pos, other, other_base_pos)
            }
            (
                ColliderType::Circle {
                    radius: self_radius,
                },
                ColliderType::Circle {
                    radius: other_radius,
                },
            ) => {
                if !self.bounding_boxes_collide(self_base_pos, other, other_base_pos) {
                    return false;
                }

                let self_center = self_base_pos + self.off;
                let other_center = other_base_pos + other.off;

                self_center.dist(other_center) <= (self_radius + other_radius)
            }
            (ColliderType::Circle { radius: _ }, ColliderType::Rect { size: _ }) => {
                Self::circle_on_rect_overlapping(other, other_base_pos, self, self_base_pos)
            }
            (ColliderType::Rect { size: _ }, ColliderType::Circle { radius: _ }) => {
                Self::circle_on_rect_overlapping(self, self_base_pos, other, other_base_pos)
            }
        }
    }
    pub fn outside_overlap(
        self,
        self_base_pos: Vec2,
        other: Self,
        other_base_pos: Vec2,
    ) -> Option<Offset2> {
        match (self.t, other.t) {
            (ColliderType::Rect { size: _ }, ColliderType::Rect { size: _ }) => {
                let a_min = self.get_rect_top_left(self_base_pos);
                let a_max = self.get_rect_bottom_right(self_base_pos);
                let b_min = other.get_rect_top_left(other_base_pos);
                let b_max = other.get_rect_bottom_right(other_base_pos);

                let overlap_x = a_max.x.min(b_max.x) - a_min.x.max(b_min.x);
                let overlap_y = a_max.y.min(b_max.y) - a_min.y.max(b_min.y);

                if overlap_x <= 0.0 || overlap_y <= 0.0 {
                    return None;
                }

                let a_center = self.center(self_base_pos);
                let b_center = other.center(other_base_pos);

                println!("({overlap_x}, {overlap_y}): {a_center:?},{b_center:?}");

                if overlap_x < overlap_y {
                    let sign = if a_center.x < b_center.x { -1.0 } else { 1.0 };
                    Some(Offset2 {
                        x: sign * overlap_x,
                        y: 0.0,
                    })
                } else {
                    let sign = if a_center.y < b_center.y { -1.0 } else { 1.0 };
                    Some(Offset2 {
                        x: 0.0,
                        y: sign * overlap_y,
                    })
                }
            }
            (
                ColliderType::Circle { radius: radius_a },
                ColliderType::Circle { radius: radius_b },
            ) => {
                if !self.bounding_boxes_collide(self_base_pos, other, other_base_pos) {
                    return None;
                }

                let center_a = self_base_pos + self.off;
                let center_b = other_base_pos + other.off;

                let diff = center_a - center_b;
                let dist_sq = diff.x * diff.x + diff.y * diff.y;
                let min_dist = radius_a + radius_b;

                if dist_sq >= min_dist * min_dist {
                    return None;
                }

                Some(diff.into())
            }
            (ColliderType::Circle { radius: _ }, ColliderType::Rect { size: _ }) => {
                Self::circle_on_rect_overlap(other, other_base_pos, self, self_base_pos)
            }
            (ColliderType::Rect { size: _ }, ColliderType::Circle { radius: _ }) => {
                Self::circle_on_rect_overlap(self, self_base_pos, other, other_base_pos)
            }
        }
    }
    pub fn circle_on_rect_overlap(
        rect: Self,
        rect_base_pos: Vec2,
        circle: Self,
        circle_base_pos: Vec2,
    ) -> Option<Offset2> {
        if !rect.bounding_boxes_collide(rect_base_pos, circle, circle_base_pos) {
            return None;
        }

        let p = circle.center(circle_base_pos).clamp(
            rect.get_rect_top_left(rect_base_pos),
            rect.get_rect_bottom_right(rect_base_pos),
        );

        if circle.center(circle_base_pos).dist(p) > circle.radius().unwrap() {
            return None;
        }

        Some(p - circle.center(circle_base_pos))
    }
    pub fn circle_on_rect_overlapping(
        rect: Self,
        rect_base_pos: Vec2,
        circle: Self,
        circle_base_pos: Vec2,
    ) -> bool {
        if !rect.bounding_boxes_collide(rect_base_pos, circle, circle_base_pos) {
            return false;
        }

        let p = circle.center(circle_base_pos).clamp(
            rect.get_rect_top_left(rect_base_pos),
            rect.get_rect_bottom_right(rect_base_pos),
        );

        circle.center(circle_base_pos).dist(p) <= circle.radius().unwrap()
    }
    pub fn center(self, base_pos: Vec2) -> Vec2 {
        base_pos + self.off
    }
    pub fn radius(self) -> Option<f32> {
        match self.t {
            ColliderType::Circle { radius } => Some(radius),
            ColliderType::Rect { size: _ } => None,
        }
    }
    pub fn get_rect_top_left(self, self_base_pos: Vec2) -> Vec2 {
        (self_base_pos + self.off) - (self.get_size() / 2.0)
    }
    pub fn get_rect_bottom_right(self, self_base_pos: Vec2) -> Vec2 {
        self.get_rect_top_left(self_base_pos) + self.get_size()
    }
    pub fn get_size(self) -> Offset2 {
        match self.t {
            ColliderType::Circle { radius } => Offset2 {
                x: radius,
                y: radius,
            },
            ColliderType::Rect { size } => size,
        }
    }
    pub fn bounding_boxes_collide(
        self,
        self_base_pos: Vec2,
        other: Self,
        other_base_pos: Vec2,
    ) -> bool {
        let self_top_left = self.get_rect_top_left(self_base_pos);
        let self_bottom_right = self.get_rect_bottom_right(self_base_pos);

        let other_top_left = other.get_rect_top_left(other_base_pos);
        let other_bottom_right = other.get_rect_bottom_right(other_base_pos);

        self_top_left.x < other_bottom_right.x
            && self_bottom_right.x > other_top_left.x
            && self_top_left.y < other_bottom_right.y
            && self_bottom_right.y > other_top_left.y
    }
}

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum StateData {
    Vec2(Vec2),
    Offset2(Offset2),
    /// Serialized using its string ID
    ObjectRef(ObjectRef),
    /// Serialized using its string ID
    RoomRef(RoomRef),
    Color(Color),
    Float(f32),
    Int(isize),
    Uint(usize),
    Bool(bool),
    Duration(Duration),
    String(String),
    List(Vec<StateData>),
    Option(Option<Box<StateData>>),
    LanguageRef(LanguageRef),
}

struct StateDataSeed<'a, 'b> {
    ctx: &'a Ctx,
    data: Option<&'b StateData>,
}

impl Serialize for StateDataSeed<'_, '_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // SAFETY: Because StateData is #[repr(u8)], this is guarenteed to be safe.
        let num = unsafe { *<*const _>::from(self.data.unwrap()).cast::<u8>() };
        let mut tuple = serializer.serialize_tuple_variant(
            "SD_",
            num as u32,
            self.data.unwrap().variant_name(),
            1,
        )?;

        use StateData::*;
        match self.data.unwrap() {
            Vec2(v) => tuple.serialize_field(v)?,
            Offset2(v) => tuple.serialize_field(v)?,
            ObjectRef(v) => tuple.serialize_field(self.ctx.get_obj_id(*v))?,
            RoomRef(v) => tuple.serialize_field(self.ctx.get_room_id(*v))?,
            Color(v) => tuple.serialize_field(v)?,
            Float(v) => tuple.serialize_field(v)?,
            Int(v) => tuple.serialize_field(v)?,
            Uint(v) => tuple.serialize_field(v)?,
            Bool(v) => tuple.serialize_field(v)?,
            Duration(v) => tuple.serialize_field(v)?,
            String(v) => tuple.serialize_field(v)?,
            List(v) => tuple.serialize_field(
                &v.iter()
                    .map(|v| StateDataSeed {
                        ctx: self.ctx,
                        data: Some(v),
                    })
                    .collect::<Vec<_>>(),
            )?,
            Option(v) => tuple.serialize_field(&v.as_ref().map(|v| StateDataSeed {
                ctx: self.ctx,
                data: Some(v),
            }))?,
            LanguageRef(v) => tuple.serialize_field(self.ctx.get_lang_id(*v))?,
        };

        tuple.end()
    }
}

impl<'de> DeserializeSeed<'de> for StateDataSeed<'_, '_> {
    type Value = StateData;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StateDataVisitor<'a> {
            ctx: &'a Ctx,
            var: Option<String>,
        }
        impl<'de> Visitor<'de> for StateDataVisitor<'_> {
            type Value = StateData;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "a piece of state")
            }
            fn visit_enum<A>(mut self, data: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::EnumAccess<'de>,
            {
                let (var, contents) = data.variant()?;
                self.var = Some(var);
                contents.tuple_variant(1, self)
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let var = self
                    .var
                    .as_ref()
                    .ok_or_else(|| serde::de::Error::custom("missing variant name"))?;
                match var.as_str() {
                    "Vec2" => {
                        let v: Vec2 = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        Ok(StateData::Vec2(v))
                    }
                    "Offset2" => {
                        let v: Offset2 = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        Ok(StateData::Offset2(v))
                    }
                    "ObjectRef" => {
                        let id: String = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        let obj_ref = self.ctx.get_obj_id_ref(&id).ok_or_else(|| {
                            serde::de::Error::custom(format!("unknown object ID: {}", id))
                        })?;
                        Ok(StateData::ObjectRef(obj_ref))
                    }
                    "RoomRef" => {
                        let id: String = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        let room_ref = self.ctx.get_room_id_ref(&id).ok_or_else(|| {
                            serde::de::Error::custom(format!("unknown room ID: {}", id))
                        })?;
                        Ok(StateData::RoomRef(room_ref))
                    }
                    "Color" => {
                        let v: Color = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        Ok(StateData::Color(v))
                    }
                    "Float" => {
                        let v: f32 = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        Ok(StateData::Float(v))
                    }
                    "Int" => {
                        let v: isize = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        Ok(StateData::Int(v))
                    }
                    "Uint" => {
                        let v: usize = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        Ok(StateData::Uint(v))
                    }
                    "Bool" => {
                        let v: bool = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        Ok(StateData::Bool(v))
                    }
                    "Duration" => {
                        let v: Duration = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        Ok(StateData::Duration(v))
                    }
                    "String" => {
                        let v: String = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        Ok(StateData::String(v))
                    }
                    "List" => {
                        let v: Vec<StateData> = seq
                            .next_element_seed(StateDataListSeed { ctx: self.ctx })?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        Ok(StateData::List(v))
                    }
                    "Option" => {
                        let v: Option<StateData> = seq
                            .next_element_seed(StateDataOptionSeed { ctx: self.ctx })?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        Ok(StateData::Option(v.map(Box::new)))
                    }
                    "LanguageRef" => {
                        let id: String = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                        let room_ref = self.ctx.get_lang_id_ref(&id).ok_or_else(|| {
                            serde::de::Error::custom(format!("unknown room ID: {}", id))
                        })?;
                        Ok(StateData::LanguageRef(room_ref))
                    }
                    _ => Err(serde::de::Error::unknown_variant(&var, StateData::VARIANTS)),
                }
            }
        }
        deserializer.deserialize_enum(
            "SD_",
            StateData::VARIANTS,
            StateDataVisitor {
                ctx: self.ctx,
                var: None,
            },
        )
    }
}

struct StateDataOptionSeed<'a> {
    ctx: &'a Ctx,
}

impl<'de, 'a> DeserializeSeed<'de> for StateDataOptionSeed<'a> {
    type Value = Option<StateData>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct OptionVisitor<'a> {
            ctx: &'a Ctx,
        }

        impl<'de, 'a> Visitor<'de> for OptionVisitor<'a> {
            type Value = Option<StateData>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "possibly a StateData")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                StateDataSeed {
                    ctx: self.ctx,
                    data: None,
                }
                .deserialize(deserializer)
                .map(Some)
            }
        }

        deserializer.deserialize_seq(OptionVisitor { ctx: self.ctx })
    }
}

struct StateDataListSeed<'a> {
    ctx: &'a Ctx,
}

impl<'de, 'a> DeserializeSeed<'de> for StateDataListSeed<'a> {
    type Value = Vec<StateData>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SeqVisitor<'a> {
            ctx: &'a Ctx,
        }

        impl<'de, 'a> Visitor<'de> for SeqVisitor<'a> {
            type Value = Vec<StateData>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a list of StateDatas")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut list = Vec::with_capacity(seq.size_hint().unwrap_or(0));

                while let Some(val) = seq.next_element_seed(StateDataSeed {
                    ctx: self.ctx,
                    data: None,
                })? {
                    list.push(val);
                }

                Ok(list)
            }
        }

        deserializer.deserialize_seq(SeqVisitor { ctx: self.ctx })
    }
}

struct StateDataMapSeed<'a> {
    ctx: &'a Ctx,
}

impl<'de, 'a> DeserializeSeed<'de> for StateDataMapSeed<'a> {
    type Value = HashMap<ObjectStateKey, StateData>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct MapVisitor<'a> {
            ctx: &'a Ctx,
        }

        impl<'de, 'a> Visitor<'de> for MapVisitor<'a> {
            type Value = HashMap<ObjectStateKey, StateData>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a map of ObjectStateKey to StateData")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut hashmap = HashMap::with_capacity(map.size_hint().unwrap_or(0));
                while let Some(key) = map.next_key::<ObjectStateKey>()? {
                    let value_seed = StateDataSeed {
                        ctx: self.ctx,
                        data: None,
                    };
                    let value = map.next_value_seed(value_seed)?;
                    hashmap.insert(key, value);
                }
                Ok(hashmap)
            }
        }

        deserializer.deserialize_map(MapVisitor { ctx: self.ctx })
    }
}

impl PartialEq for StateData {
    fn eq(&self, other: &Self) -> bool {
        use StateData::*;
        match (self, other) {
            (Vec2(v1), Vec2(v2)) => v1 == v2,
            (Offset2(v1), Offset2(v2)) => v1 == v2,
            (ObjectRef(v1), ObjectRef(v2)) => v1 == v2,
            (RoomRef(v1), RoomRef(v2)) => v1 == v2,
            (Color(v1), Color(v2)) => v1 == v2,
            (Int(v1), Int(v2)) => v1 == v2,
            (Uint(v1), Uint(v2)) => v1 == v2,
            (Bool(v1), Bool(v2)) => v1 == v2,
            (String(v1), String(v2)) => v1 == v2,
            (Duration(v1), Duration(v2)) => v1 == v2,
            (List(v1), List(v2)) => v1 == v2,
            (Option(v1), Option(v2)) => v1 == v2,
            (LanguageRef(v1), LanguageRef(v2)) => v1 == v2,
            (Float(v1), Float(v2)) => v1.to_bits() == v2.to_bits(),
            _ => false,
        }
    }
}

impl Eq for StateData {}

impl Hash for StateData {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        use StateData::*;
        match self {
            Vec2(v) => v.hash(hasher),
            Offset2(v) => v.hash(hasher),
            ObjectRef(v) => v.hash(hasher),
            RoomRef(v) => v.hash(hasher),
            Color(v) => v.hash(hasher),
            Int(v) => v.hash(hasher),
            Uint(v) => v.hash(hasher),
            Bool(v) => v.hash(hasher),
            String(v) => v.hash(hasher),
            Duration(v) => v.hash(hasher),
            List(v) => v.hash(hasher),
            Option(v) => v.hash(hasher),
            LanguageRef(v) => v.hash(hasher),
            Float(v) => v.to_bits().hash(hasher),
        }
    }
}

macro_rules! impl_statedata {
    ($($name:ident($t:ty)),* $(,)?) => {
        $(
            impl From<$t> for StateData {
                fn from(v: $t) -> StateData {
                    StateData::$name(v)
                }
            }
            impl From<&$t> for StateData {
                fn from(v: &$t) -> StateData {
                    StateData::$name(v.clone())
                }
            }
            impl TryFrom<StateData> for $t {
                type Error = ();
                fn try_from(v: StateData) -> Result<$t, ()> {
                    match v {
                        StateData::$name(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
        impl StateData {
            const VARIANTS: &[&str] = &[
                $(stringify!($name)),*
            ];
            fn variant_name(&self) -> &'static str {
                match self {
                    $(
                        StateData::$name(_) => stringify!($name),
                    )*
                    StateData::List(_) => "List",
                    StateData::Option(_) => "Option",
                }
            }
        }
    };
}

impl_statedata! {
    Vec2(Vec2),
    Offset2(Offset2),
    ObjectRef(ObjectRef),
    RoomRef(RoomRef),
    Color(Color),
    Float(f32),
    Int(isize),
    Uint(usize),
    Bool(bool),
    String(String),
    Duration(Duration),
    LanguageRef(LanguageRef),
}

impl<T> From<&Vec<T>> for StateData
where
    StateData: for<'a> From<&'a T>,
{
    fn from(v: &Vec<T>) -> StateData {
        StateData::List(v.iter().map(|v| v.into()).collect())
    }
}
impl<T> From<Vec<T>> for StateData
where
    StateData: From<T>,
{
    fn from(v: Vec<T>) -> StateData {
        StateData::List(v.into_iter().map(|v| v.into()).collect())
    }
}
impl<T> TryFrom<StateData> for Vec<T>
where
    T: TryFrom<StateData, Error = ()>,
{
    type Error = ();
    fn try_from(v: StateData) -> Result<Vec<T>, ()> {
        match v {
            StateData::List(v) => Ok(v
                .into_iter()
                .map(|v| <StateData as TryInto<T>>::try_into(v))
                .collect::<Result<Vec<_>, ()>>()?),
            _ => Err(()),
        }
    }
}

impl<T> From<&Option<T>> for StateData
where
    StateData: for<'a> From<&'a T>,
{
    fn from(v: &Option<T>) -> StateData {
        StateData::Option(v.as_ref().map(|v| Box::new(v.into())))
    }
}
impl<T> From<Option<T>> for StateData
where
    StateData: From<T>,
{
    fn from(v: Option<T>) -> StateData {
        StateData::Option(v.map(|v| Box::new(v.into())))
    }
}
impl<T> TryFrom<StateData> for Option<T>
where
    T: TryFrom<StateData, Error = ()>,
{
    type Error = ();
    fn try_from(v: StateData) -> Result<Option<T>, ()> {
        match v {
            StateData::Option(v) => Ok(v
                .map(|v| <StateData as TryInto<T>>::try_into(*v))
                .transpose()?),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObjectStateKey {
    Pos,
    ZLayer,
    Rotate,
    Scale,
    Visible,
    Processing,
    Animation,
    AniFrame,
    #[doc(hidden)]
    AniFrameTimer,
    Playing,
    Other(String),
}

impl serde::Serialize for ObjectStateKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for ObjectStateKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StringVisitor;
        impl<'de> Visitor<'de> for StringVisitor {
            type Value = String;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string key")
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(v)
            }
        }
        Ok(Self::from(deserializer.deserialize_string(StringVisitor)?))
    }
}

macro_rules! impl_obj_state_key {
    ($($key:literal => $out:ident),* $(,)?) => {
        impl<T: AsRef<str>> From<T> for ObjectStateKey {
            fn from(val: T) -> Self {
                match val.as_ref() {
                    $($key => Self::$out,)*
                    v => Self::Other(v.to_string())
                }
            }
        }
        impl Display for ObjectStateKey {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$out => f.write_str($key),)*
                    Self::Other(v) => f.write_str(&v)
                }
            }
        }
    };
}

impl_obj_state_key!(
    "_zr.pos" => Pos,
    "_zr.rot" => Rotate,
    "_zr.scl" => Scale,
    "_zr.vis" => Visible,
    "_zr.prc" => Processing,
    "_zr.ani" => Animation,
    "_zr.anf" => AniFrame,
    "_zr.pla" => Playing,
    "_zr.aft" => AniFrameTimer,
    "_zr.zlr" => ZLayer,
);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ObjectState(HashMap<ObjectStateKey, StateData>, bool);

impl Hash for ObjectState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for (k, v) in &self.0 {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl ObjectState {
    pub fn new() -> Self {
        Self(HashMap::new(), false)
    }
    /// Used internally to create a state that everything is flattened into for
    /// saving purposes.
    pub(crate) fn new_flattening_state() -> Self {
        Self(HashMap::new(), true)
    }
    /// Serialize this, panicing if not in flattening mode
    pub(crate) fn serialize(&self, ctx: &Ctx) -> Vec<u8> {
        if !self.1 {
            panic!("Object state keys may not include dots due to flattening rules")
        }
        postcard::to_allocvec(
            &self
                .0
                .iter()
                .map(|v| {
                    (
                        v.0,
                        StateDataSeed {
                            ctx,
                            data: Some(v.1),
                        },
                    )
                })
                .collect::<HashMap<_, _>>(),
        )
        .unwrap()
    }
    /// Deserialize this without unflattening it.
    pub(crate) fn deserialize(buf: &[u8], ctx: &Ctx) -> Self {
        let mut de = postcard::Deserializer::from_bytes(buf);
        let seed = StateDataMapSeed { ctx };

        let v: HashMap<ObjectStateKey, StateData> = seed.deserialize(&mut de).unwrap();

        Self(v, true)
    }
    /// Flatten a single ObjectState into this one with the specified namespace.
    /// The namespace should not end with a ., but realisticly it doesn't really
    /// matter.
    #[track_caller]
    pub(crate) fn flatten(&mut self, mut namespace: String, other: &ObjectState) {
        if !self.1 {
            panic!("May not flatten non-flattening state")
        }
        namespace.push('.');

        for (k, v) in other.iter() {
            self.set(
                ObjectStateKey::Other(namespace.clone() + &k.to_string()),
                v.clone(),
            );
        }
    }
    pub(crate) fn unflatten(&self, namespace: impl AsRef<str>, out: &mut ObjectState) {
        let namespace = namespace.as_ref();
        for (k, v) in self.iter() {
            if let Some(key) = k
                .to_string()
                .strip_prefix(namespace)
                .and_then(|v| v.strip_prefix('.'))
            {
                out.set(ObjectStateKey::from(key), v.clone());
            }
        }
    }
    #[must_use]
    pub fn get<T: TryFrom<StateData>>(&self, key: ObjectStateKey) -> Option<T> {
        self.0.get(&key).cloned().and_then(|v| v.try_into().ok())
    }
    #[track_caller]
    pub fn set<T: Into<StateData>>(&mut self, key: ObjectStateKey, val: T) {
        if !self.1
            && let ObjectStateKey::Other(v) = &key
            && v.contains('.')
        {
            panic!("Object state keys may not include dots due to flattening rules")
        }
        self.0.insert(key, val.into());
    }
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a ObjectStateKey, &'a StateData)> + 'a {
        self.0.iter()
    }
}

impl IntoIterator for ObjectState {
    type IntoIter = collections::hash_map::IntoIter<ObjectStateKey, StateData>;
    type Item = (ObjectStateKey, StateData);
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(derive_more::Debug)]
pub struct World {
    pub current_room: RoomRef,
    pub ctx: Ctx,
    /// Objects that aren't a part of this room but are loaded (i.e. the player)
    pub extra_objs: Vec<ObjectRef>,
    /// The player object(s) that need to be teleported to an entrypoint when
    /// entering a room. NOT loaded automatically, must be in extra_objs.
    pub player: Vec<ObjectRef>,
    pub room_transition: Option<ObjectRef>,
    pub callbacks: Option<Callbacks>,
    pub state: ObjectState,
    pub camera_obj: ObjectRef,
    pub input_mappings: HashMap<Key, HashSet<ActionRef>>,
    /// The currently selected language.
    pub lang: LanguageRef,
    /// The game ID. Only requirement is it must be a valid path element on all
    /// platforms this game is exported to.
    pub game_id: String,
    #[debug(skip)]
    pub(crate) text: HashSet<TextRef>,
    #[debug(skip)]
    pub(crate) event_queue: Vec<(EventTarget, Box<dyn FnMut() -> Event<'static>>)>,
    #[debug(skip)]
    pub(crate) internal_event_queue: Vec<InternalEvent>,
    #[debug(skip)]
    pub(crate) current_frame_presses: HashMap<ActionRef, InputState>,
    #[debug(skip)]
    pub(crate) audio_handle: rodio::MixerDeviceSink,
}

impl World {
    pub fn new(
        ctx: Ctx,
        current_room: RoomRef,
        extra_objs: Vec<ObjectRef>,
        player: Vec<ObjectRef>,
        callbacks: Option<Callbacks>,
        state: ObjectState,
        camera_obj: ObjectRef,
        input_mappings: HashMap<Key, HashSet<ActionRef>>,
        room_transition: Option<ObjectRef>,
        lang: LanguageRef,
        game_id: String,
    ) -> Self {
        let mut handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        handle.log_on_drop(false);
        World {
            current_room,
            ctx,
            extra_objs,
            player,
            callbacks,
            state,
            camera_obj,
            input_mappings,
            room_transition,
            lang,
            game_id,
            event_queue: vec![],
            internal_event_queue: vec![],
            current_frame_presses: HashMap::new(),
            audio_handle: handle,
            text: HashSet::new(),
        }
    }
    pub fn save(&mut self, save_num: u16) {
        self.internal_event_queue
            .push(InternalEvent::Save(save_num));
    }
    pub fn load(&mut self, save_num: u16) {
        self.internal_event_queue
            .push(InternalEvent::Load(save_num));
    }
    pub fn save_exists(&self, save_num: u16) -> bool {
        crate::rt::get_save_path(self, save_num).exists()
    }
    pub fn save_to_bytes(&self) -> Vec<u8> {
        let mut out = ObjectState::new_flattening_state();

        out.set("game_id".into(), self.game_id.clone());

        let root_internal = "world-internal.".to_string();
        for (name, value) in [
            ("current_room", (&self.current_room).into()),
            ("extra_objs", (&self.extra_objs).into()),
            ("player", (&self.player).into()),
            ("camera_obj", (&self.camera_obj).into()),
            ("room_transition", (&self.room_transition).into()),
            ("lang", (&self.lang).into()),
        ] {
            out.set::<StateData>(ObjectStateKey::Other(root_internal.clone() + name), value);
        }

        out.flatten("world".to_string(), &self.state);

        for i in 0..self.ctx.rooms.len() {
            let room = self.ctx.rooms[i].as_ref();
            let id = self.ctx.room_ids[i].as_ref();
            if room.is_none() {
                continue;
            }
            let room = room.unwrap();
            let id = id.unwrap();
            out.flatten(format!("rooms.{id}"), &room.state);
        }

        for i in 0..self.ctx.objects.len() {
            let object = self.ctx.objects[i].as_ref();
            let id = self.ctx.obj_ids[i].as_ref();
            if object.is_none() {
                continue;
            }
            let object = object.unwrap();
            let id = id.unwrap();
            out.flatten(format!("objs.{id}"), &object.state);
        }

        out.serialize(&self.ctx)
    }
    pub fn load_from_bytes(&mut self, bytes: &[u8]) {
        let from_state = ObjectState::deserialize(bytes, &self.ctx);

        assert_eq!(
            from_state.get(ObjectStateKey::Other("game_id".to_string())),
            Some(self.game_id.clone()),
            "cannot load game save from different game"
        );

        self.current_room = from_state.get("current_room".into()).unwrap();
        self.extra_objs = from_state.get("extra_objs".into()).unwrap();
        self.player = from_state.get("player".into()).unwrap();
        self.camera_obj = from_state.get("camera_obj".into()).unwrap();
        self.room_transition = from_state.get("room_transition".into()).unwrap();
        self.lang = from_state.get("lang".into()).unwrap();

        from_state.unflatten("world", &mut self.state);

        for i in 0..self.ctx.rooms.len() {
            let room = self.ctx.rooms[i].as_mut();
            let id = self.ctx.room_ids[i].as_ref();
            if room.is_none() {
                continue;
            }
            let room = room.unwrap();
            let id = id.unwrap();
            from_state.unflatten(format!("rooms.{id}"), &mut room.state);
        }

        for i in 0..self.ctx.objects.len() {
            let object = self.ctx.objects[i].as_mut();
            let id = self.ctx.obj_ids[i].as_ref();
            if object.is_none() {
                continue;
            }
            let object = object.unwrap();
            let id = id.unwrap();
            from_state.unflatten(format!("objs.{id}"), &mut object.state);
        }
    }
    pub fn show_text(&mut self, text: TextRef) {
        self.text.insert(text);
    }
    pub fn hide_text(&mut self, text: TextRef) {
        self.text.remove(&text);
    }
    pub fn transition_room(&mut self, entry: EntryRef) {
        self.internal_event_queue
            .push(InternalEvent::RoomTransition(entry));
    }
    /// Begin/resume playing some audio. When beginning playing it, the source
    /// will be consumed and replaced with `None`!
    pub fn play_audio(&mut self, audio: AudioRef) {
        self.internal_event_queue
            .push(InternalEvent::PlayAudio(audio));
    }
    /// Pause audio. It can be resumed with [`play_audio`](World::play_world).
    pub fn pause_audio(&mut self, audio: AudioRef) {
        self.internal_event_queue
            .push(InternalEvent::PauseAudio(audio));
    }
    /// Stop audio. It CANNOT be resumed after this and the audio is consumed!
    pub fn stop_audio(&mut self, audio: AudioRef) {
        self.internal_event_queue
            .push(InternalEvent::StopAudio(audio));
    }
    pub fn audio_playing(&self, audio: AudioRef) -> bool {
        let audio = self.ctx.get_audio(audio);
        audio.source.is_none() && audio.player.is_some()
    }
    pub fn audio_not_started(&self, audio: AudioRef) -> bool {
        let audio = self.ctx.get_audio(audio);
        audio.source.is_some()
    }
    pub fn audio_stopped(&self, audio: AudioRef) -> bool {
        let audio = self.ctx.get_audio(audio);
        audio.source.is_none() && audio.player.is_none()
    }
    pub fn audio_paused(&self, audio: AudioRef) -> bool {
        let audio = self.ctx.get_audio(audio);
        audio.player.is_some() && audio.player.as_ref().unwrap().is_paused()
    }
    pub fn post_event(
        &mut self,
        target: EventTarget,
        event_producer: impl FnMut() -> Event<'static> + 'static,
    ) {
        self.event_queue
            .push((target, Box::new(event_producer) as Box<_>))
    }
    pub fn add_mapping(&mut self, action: ActionRef, key: Key) {
        match self.input_mappings.entry(key) {
            collections::hash_map::Entry::Occupied(mut v) => {
                v.get_mut().insert(action);
            }
            collections::hash_map::Entry::Vacant(v) => {
                let mut set = HashSet::new();
                set.insert(action);
                v.insert(set);
            }
        }
    }
    /// Remove the provided mapping from key to action. Will take effect on the
    /// next frame.
    pub fn remove_mapping(&mut self, action: ActionRef, key: Key) {
        if let Some(actions) = self.input_mappings.get_mut(&key) {
            actions.remove(&action);
            if actions.is_empty() {
                self.input_mappings.remove(&key);
            }
        }
    }
    pub fn input_state(&self, action: ActionRef) -> InputState {
        self.current_frame_presses
            .get(&action)
            .copied()
            .unwrap_or(InputState::Released)
    }
    pub fn action_down(&self, action: ActionRef) -> bool {
        matches!(
            self.input_state(action),
            InputState::Pressed | InputState::NewlyPressed
        )
    }
    pub fn action_up(&self, action: ActionRef) -> bool {
        matches!(
            self.input_state(action),
            InputState::Released | InputState::NewlyReleased
        )
    }
    pub fn action_new_down(&self, action: ActionRef) -> bool {
        matches!(self.input_state(action), InputState::NewlyPressed)
    }
    pub fn action_new_up(&self, action: ActionRef) -> bool {
        matches!(self.input_state(action), InputState::NewlyReleased)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Room {
    /// The sprite and offset of the background.
    pub background: Option<(SpriteRef, Vec2)>,
    pub objects: Vec<ObjectRef>,
    pub callbacks: Option<Callbacks>,
    pub state: ObjectState,
    /// Entry points where the player will spawn when entering this room.
    pub entrypoints: HashMap<EntryRef, Vec2>,
}

macro_rules! event_enums {
    (
        $(
            $(#[$meta:meta])*
            $matcher:pat =>
            $variant:ident
                $( ($($(#[$meta3:meta])* $t2:ty),* $(,)?) )?
                $({$( $(#[$meta2:meta])* $name:ident: $t:ty),* $(,)?})?
            ),* $(,)?) => {
        #[derive(derive_more::Display, Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum EventName {
            $($(#[$meta])* $variant),*
        }
        #[derive(derive_more::Debug)]
        pub enum Event<'a> {
            $($(#[$meta])* $variant $(( $($(#[$meta3])* $t2),* ))? $({ $( $(#[$meta2])* $name: $t ),* })?),*
        }

        impl From<&Event<'_>> for EventName {
            fn from(value: &Event<'_>) -> Self {
                match value {
                    $(
                        $matcher => EventName::$variant
                    ),*
                }
            }
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub(crate) struct SealedRoomTransition;

event_enums!(
    /// Does NOT continue if DisableDefault.
    Event::AniContinueEvent => AniContinueEvent,
    /// Called ~20 times a second. Duration is the delta time. DisableDefault is
    /// ignored. Called before graphics tick.
    Event::Tick(..) => Tick(Duration),
    /// Called at most 20 times a second; Default action will attempt to push it
    /// out to where it was at the previous frame before it started colliding.
    Event::Collide{..} => Collide {
        other: ObjectRef,
    },
    /// Called the full 60 times a second, after Tick when Tick is run. Duration is delta time,
    /// DisableDefault disables rendering the current sprite/animation like usual.
    Event::Render(..) => Render(
        Duration,
        #[debug(skip)] Arc<Mutex<crate::rt::DrawContext>>,
    ),
    /// DisableDefault has no effect. Called after all resources are loaded and
    /// initalized, but before this is presented to the player on-screen.
    Event::Load => Load,
    /// Called before this object/room/world is unloaded. On the World, functions
    /// as a callback for before the game closes or is otherwise unloaded by the
    /// engine.
    Event::Unload => Unload,
    Event::SaveData {..} => SaveData {
        /// A new object state NOT tied to the current object to write new data that
        /// has to be saved to or otherwise change the saved data.
        new_obj_state: &'a mut ObjectState
    },
    /// After all data is loaded into the ObjectState, called before Load to adjust
    /// the data or do any game-specific setup needed after loading.
    Event::LoadData => LoadData,
    Event::KeyPress { .. } => KeyPress {
        key: Key,
    },
    Event::KeyRelease { .. } => KeyRelease {
        key: Key,
    },
    Event::KeyHold { .. } => KeyHold {
        key: Key,
    },
    Event::JoystickMove { .. } => JoystickMove {
        axis: gilrs::Axis,
        /// In the range 0..=1
        value: f32,
    },
    Event::MousePress { .. } => MousePress {
        button: macroquad::input::MouseButton,
        /// World position of the mouse press
        pos: Vec2,
    },
    /// Produced by the engine for the room_transition object, if one exists. Emitted
    /// directly before unloading the old room and loading the new room (i.e. on the same frame).
    Event::RoomTransition { .. } => RoomTransition {
        /// To prevent manually constructing outside of the engine
        #[allow(private_interfaces)]
        _sealed: SealedRoomTransition,
        from: RoomRef,
        to: RoomRef,
        entry: EntryRef,
    }
);

#[derive(Debug)]
pub struct EventArgs<'a> {
    pub room: Option<RoomRef>,
    pub obj: Option<ObjectRef>,
    pub world: &'a mut World,
}

#[derive(Clone, Debug, PartialEq, Hash)]
#[must_use = "An event result should not be ignored"]
pub enum EventResult {
    Default,
    DisableDefault,
    Result(StateData),
}

#[derive(Clone, Default)]
pub struct Callbacks(HashMap<EventName, Arc<dyn Fn(Event, EventArgs) -> EventResult>>);

impl Debug for Callbacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Callbacks")
            .field(&self.0.keys())
            .finish_non_exhaustive()
    }
}

impl Callbacks {
    pub fn new() -> Self {
        Callbacks(HashMap::new())
    }
    pub fn set(
        &mut self,
        event: EventName,
        callback: impl Fn(Event, EventArgs) -> EventResult + 'static,
    ) {
        self.0.insert(event, Arc::new(callback));
    }
    #[must_use]
    pub(crate) fn trigger(&self, event: Event, args: EventArgs) -> Option<EventResult> {
        self.0.get(&(&event).into()).map(|v| v(event, args))
    }
}

#[derive(Clone, Debug, Default)]
pub struct Object {
    pub collider: Vec<Collider>,
    /// If true, will never be moved by the engine (but may be by the game).
    pub static_body: bool,
    pub sheet: Option<AniSheetRef>,
    pub state: ObjectState,
    pub callbacks: Option<Callbacks>,
}

impl Object {
    #[must_use]
    pub fn get_position(&self) -> Option<Vec2> {
        self.state.get(ObjectStateKey::Pos)
    }
    #[must_use]
    pub fn get_z_layer(&self) -> Option<isize> {
        self.state.get(ObjectStateKey::ZLayer)
    }
    #[must_use]
    pub fn get_rotation(&self) -> Option<f32> {
        self.state.get(ObjectStateKey::Rotate)
    }
    #[must_use]
    pub fn get_scale(&self) -> Option<Offset2> {
        self.state.get(ObjectStateKey::Scale)
    }
    #[must_use]
    pub fn get_ani(&self) -> Option<String> {
        self.state.get(ObjectStateKey::Animation)
    }
    #[must_use]
    pub fn get_frame(&self) -> Option<usize> {
        self.state.get(ObjectStateKey::AniFrame)
    }
    #[must_use]
    pub fn get_visible(&self) -> Option<bool> {
        self.state.get(ObjectStateKey::Visible)
    }
    #[must_use]
    pub fn get_processing(&self) -> Option<bool> {
        self.state.get(ObjectStateKey::Processing)
    }
    #[must_use]
    pub fn get_playing(&self) -> Option<bool> {
        self.state.get(ObjectStateKey::Playing)
    }
    #[must_use]
    pub fn is_processing(&self) -> bool {
        self.get_processing().unwrap_or(true)
    }
    #[must_use]
    pub fn is_visible(&self) -> bool {
        self.get_visible().unwrap_or(true)
    }
    #[must_use]
    pub fn is_playing(&self) -> bool {
        self.get_playing().unwrap_or(false)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AniSheet {
    pub anis: HashMap<String, AnimationRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AniEvent {
    Sprite {
        sprite: SpriteRef,
        frame_count: NonZeroU8,
    },
    /// Pause and wait for a AniContinueEvent to be posted to this Object.
    PausePoint,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Animation {
    pub timeline: Vec<AniEvent>,
    pub fps: u8,
    pub loops: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub fn multiply_alpha(self, bg: Color) -> Color {
        if self.a == u8::MAX {
            return self;
        }
        let inv_a = 255 - self.a;
        Color {
            r: ((self.r as u16 * self.a as u16 + bg.r as u16 * inv_a as u16) / 255) as u8,
            g: ((self.g as u16 * self.a as u16 + bg.g as u16 * inv_a as u16) / 255) as u8,
            b: ((self.b as u16 * self.a as u16 + bg.b as u16 * inv_a as u16) / 255) as u8,
            a: 255,
        }
    }
}

macro_rules! impl_color_ops1 {
    ($((($lhs:ty) ($op_t:ident [$f_name:ident $op:tt]) ($rhs:ty) -> $out:ty)),* $(,)?) => {
        paste::paste! {
            $(
                impl std::ops::$op_t<$rhs> for $lhs {
                    type Output = $out;
                    fn $f_name (self, rhs: $rhs) -> $out {
                        $out {
                            r: self.r $op rhs.r,
                            g: self.g $op rhs.g,
                            b: self.b $op rhs.b,
                            a: self.a $op rhs.a,
                        }
                    }
                }

                impl std::ops::[<$op_t Assign>] <$rhs> for $lhs {
                    fn [<$f_name _assign>] (&mut self, rhs: $rhs) {
                        *self = $out {
                            r: self.r $op rhs.r,
                            g: self.g $op rhs.g,
                            b: self.b $op rhs.b,
                            a: self.a $op rhs.a,
                        };
                    }
                }
            )*
        }
    };
}

macro_rules! impl_color_ops2 {
    ($((($lhs:ty) ($op_t:ident [$f_name:ident $op:tt]) ($rhs:ty) -> $out:ty)),* $(,)?) => {
        paste::paste! {
            $(
                impl std::ops::$op_t<$rhs> for $lhs {
                    type Output = $out;
                    fn $f_name (self, rhs: $rhs) -> $out {
                        $out {
                            r: (self.r as $rhs $op rhs) as u8,
                            g: (self.g as $rhs $op rhs) as u8,
                            b: (self.b as $rhs $op rhs) as u8,
                            a: (self.a as $rhs $op rhs) as u8,
                        }
                    }
                }

                impl std::ops::[<$op_t Assign>] <$rhs> for $lhs {
                    fn [<$f_name _assign>] (&mut self, rhs: $rhs) {
                        *self = $out {
                            r: (self.r as $rhs $op rhs) as u8,
                            g: (self.g as $rhs $op rhs) as u8,
                            b: (self.b as $rhs $op rhs) as u8,
                            a: (self.a as $rhs $op rhs) as u8,
                        };
                    }
                }
            )*
        }
    };
}

impl_color_ops1! {
    ((Color) (Add [add +]) (Color) -> Color),
    ((Color) (Sub [sub -]) (Color) -> Color),
}

impl_color_ops2! {
    ((Color) (Mul [mul *]) (u8) -> Color),
    ((Color) (Div [div /]) (u8) -> Color),
    ((Color) (Mul [mul *]) (u16) -> Color),
    ((Color) (Div [div /]) (u16) -> Color),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Sprite {
    pub width: u16,
    pub height: u16,
    pub data: Vec<Color>,
}

impl Sprite {
    pub fn get_size(&self) -> Offset2 {
        Offset2 {
            x: self.width as f32,
            y: self.height as f32,
        }
    }
}

impl Index<(u16, u16)> for Sprite {
    type Output = Color;
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        &self.data[index.0 as usize + (index.1 as usize * self.width as usize)]
    }
}

impl IndexMut<(u16, u16)> for Sprite {
    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        &mut self.data[index.0 as usize + (index.1 as usize * self.width as usize)]
    }
}

#[derive(derive_more::Debug)]
pub struct Audio {
    #[debug("{{opaque source object}}")]
    pub source: Option<Box<dyn Source + Send + 'static>>,
    /// The 3D location of the audio source where (0, 0, 0) is the listener with
    /// the left and right ear 0.1 offset from the center in the X axis.
    #[debug("(x: {}, y: {}, z: {})", loc.0, loc.1, loc.2)]
    pub loc: (f32, f32, f32),
    #[debug(skip)]
    pub(crate) player: Option<SpatialPlayer>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Font {
    pub char_index_map: HashMap<char, usize>,
    /// u16 is the width of that character
    pub sprites: Vec<(SpriteRef, u16)>,
    pub line_height: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DisplayedText {
    pub contents: LocalTextRef,
    /// Screen-space, NOT world-space.
    pub loc: Vec2,
    /// Each character is rotated by this, not the entire text. Essentially poor
    /// mans italic.
    pub char_rot: f32,
    pub font: FontRef,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageData {
    pub strings: HashMap<LocalTextRef, String>,
}
