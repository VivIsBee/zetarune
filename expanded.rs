pub mod objs {
    //! Data model
    use std::{
        collections::{self, HashMap, HashSet},
        fmt::{Debug, Display},
        hash::Hash, num::NonZeroU8, ops::{Index, IndexMut},
        sync::{Arc, Mutex},
        time::Duration,
    };
    use rodio::{Source, SpatialPlayer};
    use crate::{ctx::*, rt::{EventTarget, InputState, InternalEvent, Key}};
    pub struct Offset2 {
        pub x: f32,
        pub y: f32,
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for Offset2 {}
    #[automatically_derived]
    impl ::core::clone::Clone for Offset2 {
        #[inline]
        fn clone(&self) -> Offset2 {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Offset2 {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Offset2 {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Offset2",
                "x",
                &self.x,
                "y",
                &&self.y,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Offset2 {
        #[inline]
        fn default() -> Offset2 {
            Offset2 {
                x: ::core::default::Default::default(),
                y: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Offset2 {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Offset2 {
        #[inline]
        fn eq(&self, other: &Offset2) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Offset2 {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Offset2,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.x, &other.x) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.y, &other.y)
                }
                cmp => cmp,
            }
        }
    }
    impl Offset2 {
        pub const ZERO: Self = Offset2 { x: 0.0, y: 0.0 };
    }
    impl From<Offset2> for Vec2 {
        fn from(value: Offset2) -> Self {
            Vec2 { x: value.x, y: value.y }
        }
    }
    pub struct Vec2 {
        pub x: f32,
        pub y: f32,
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for Vec2 {}
    #[automatically_derived]
    impl ::core::clone::Clone for Vec2 {
        #[inline]
        fn clone(&self) -> Vec2 {
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Vec2 {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Vec2 {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Vec2",
                "x",
                &self.x,
                "y",
                &&self.y,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Vec2 {
        #[inline]
        fn default() -> Vec2 {
            Vec2 {
                x: ::core::default::Default::default(),
                y: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Vec2 {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Vec2 {
        #[inline]
        fn eq(&self, other: &Vec2) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Vec2 {
        #[inline]
        fn partial_cmp(
            &self,
            other: &Vec2,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            match ::core::cmp::PartialOrd::partial_cmp(&self.x, &other.x) {
                ::core::option::Option::Some(::core::cmp::Ordering::Equal) => {
                    ::core::cmp::PartialOrd::partial_cmp(&self.y, &other.y)
                }
                cmp => cmp,
            }
        }
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
    impl std::ops::Add<Offset2> for Offset2 {
        type Output = Offset2;
        fn add(self, rhs: Offset2) -> Offset2 {
            Offset2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    ///assign
    impl std::ops::AddAssign<Offset2> for Offset2 {
        fn add_assign(&mut self, rhs: Offset2) {
            *self = Offset2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            };
        }
    }
    impl std::ops::Sub<Offset2> for Offset2 {
        type Output = Offset2;
        fn sub(self, rhs: Offset2) -> Offset2 {
            Offset2 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
    ///assign
    impl std::ops::SubAssign<Offset2> for Offset2 {
        fn sub_assign(&mut self, rhs: Offset2) {
            *self = Offset2 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            };
        }
    }
    impl std::ops::Mul<Offset2> for Offset2 {
        type Output = Offset2;
        fn mul(self, rhs: Offset2) -> Offset2 {
            Offset2 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            }
        }
    }
    ///assign
    impl std::ops::MulAssign<Offset2> for Offset2 {
        fn mul_assign(&mut self, rhs: Offset2) {
            *self = Offset2 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            };
        }
    }
    impl std::ops::Div<Offset2> for Offset2 {
        type Output = Offset2;
        fn div(self, rhs: Offset2) -> Offset2 {
            Offset2 {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
            }
        }
    }
    ///assign
    impl std::ops::DivAssign<Offset2> for Offset2 {
        fn div_assign(&mut self, rhs: Offset2) {
            *self = Offset2 {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
            };
        }
    }
    impl std::ops::Rem<Offset2> for Offset2 {
        type Output = Offset2;
        fn rem(self, rhs: Offset2) -> Offset2 {
            Offset2 {
                x: self.x % rhs.x,
                y: self.y % rhs.y,
            }
        }
    }
    ///assign
    impl std::ops::RemAssign<Offset2> for Offset2 {
        fn rem_assign(&mut self, rhs: Offset2) {
            *self = Offset2 {
                x: self.x % rhs.x,
                y: self.y % rhs.y,
            };
        }
    }
    impl std::ops::Add<Vec2> for Vec2 {
        type Output = Vec2;
        fn add(self, rhs: Vec2) -> Vec2 {
            Vec2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    ///assign
    impl std::ops::AddAssign<Vec2> for Vec2 {
        fn add_assign(&mut self, rhs: Vec2) {
            *self = Vec2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            };
        }
    }
    impl std::ops::Sub<Vec2> for Vec2 {
        type Output = Offset2;
        fn sub(self, rhs: Vec2) -> Offset2 {
            Offset2 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
    impl std::ops::Mul<Vec2> for Vec2 {
        type Output = Vec2;
        fn mul(self, rhs: Vec2) -> Vec2 {
            Vec2 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            }
        }
    }
    ///assign
    impl std::ops::MulAssign<Vec2> for Vec2 {
        fn mul_assign(&mut self, rhs: Vec2) {
            *self = Vec2 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
            };
        }
    }
    impl std::ops::Div<Vec2> for Vec2 {
        type Output = Vec2;
        fn div(self, rhs: Vec2) -> Vec2 {
            Vec2 {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
            }
        }
    }
    ///assign
    impl std::ops::DivAssign<Vec2> for Vec2 {
        fn div_assign(&mut self, rhs: Vec2) {
            *self = Vec2 {
                x: self.x / rhs.x,
                y: self.y / rhs.y,
            };
        }
    }
    impl std::ops::Rem<Vec2> for Vec2 {
        type Output = Vec2;
        fn rem(self, rhs: Vec2) -> Vec2 {
            Vec2 {
                x: self.x % rhs.x,
                y: self.y % rhs.y,
            }
        }
    }
    ///assign
    impl std::ops::RemAssign<Vec2> for Vec2 {
        fn rem_assign(&mut self, rhs: Vec2) {
            *self = Vec2 {
                x: self.x % rhs.x,
                y: self.y % rhs.y,
            };
        }
    }
    impl std::ops::Add<Offset2> for Vec2 {
        type Output = Vec2;
        fn add(self, rhs: Offset2) -> Vec2 {
            Vec2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    ///assign
    impl std::ops::AddAssign<Offset2> for Vec2 {
        fn add_assign(&mut self, rhs: Offset2) {
            *self = Vec2 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            };
        }
    }
    impl std::ops::Sub<Offset2> for Vec2 {
        type Output = Vec2;
        fn sub(self, rhs: Offset2) -> Vec2 {
            Vec2 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
    ///assign
    impl std::ops::SubAssign<Offset2> for Vec2 {
        fn sub_assign(&mut self, rhs: Offset2) {
            *self = Vec2 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            };
        }
    }
    impl std::ops::Div<f32> for Offset2 {
        type Output = Offset2;
        fn div(self, rhs: f32) -> Offset2 {
            Offset2 {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }
    ///assign
    impl std::ops::DivAssign<f32> for Offset2 {
        fn div_assign(&mut self, rhs: f32) {
            *self = Offset2 {
                x: self.x / rhs,
                y: self.y / rhs,
            };
        }
    }
    impl std::ops::Mul<f32> for Offset2 {
        type Output = Offset2;
        fn mul(self, rhs: f32) -> Offset2 {
            Offset2 {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }
    ///assign
    impl std::ops::MulAssign<f32> for Offset2 {
        fn mul_assign(&mut self, rhs: f32) {
            *self = Offset2 {
                x: self.x / rhs,
                y: self.y / rhs,
            };
        }
    }
    impl std::ops::Div<f32> for Vec2 {
        type Output = Vec2;
        fn div(self, rhs: f32) -> Vec2 {
            Vec2 {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }
    ///assign
    impl std::ops::DivAssign<f32> for Vec2 {
        fn div_assign(&mut self, rhs: f32) {
            *self = Vec2 {
                x: self.x / rhs,
                y: self.y / rhs,
            };
        }
    }
    impl std::ops::Mul<f32> for Vec2 {
        type Output = Vec2;
        fn mul(self, rhs: f32) -> Vec2 {
            Vec2 {
                x: self.x / rhs,
                y: self.y / rhs,
            }
        }
    }
    ///assign
    impl std::ops::MulAssign<f32> for Vec2 {
        fn mul_assign(&mut self, rhs: f32) {
            *self = Vec2 {
                x: self.x / rhs,
                y: self.y / rhs,
            };
        }
    }
    pub enum ColliderType {
        Rect { size: Offset2 },
        Circle { radius: f32 },
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for ColliderType {}
    #[automatically_derived]
    impl ::core::clone::Clone for ColliderType {
        #[inline]
        fn clone(&self) -> ColliderType {
            let _: ::core::clone::AssertParamIsClone<Offset2>;
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ColliderType {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ColliderType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ColliderType::Rect { size: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Rect",
                        "size",
                        &__self_0,
                    )
                }
                ColliderType::Circle { radius: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Circle",
                        "radius",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ColliderType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ColliderType {
        #[inline]
        fn eq(&self, other: &ColliderType) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        ColliderType::Rect { size: __self_0 },
                        ColliderType::Rect { size: __arg1_0 },
                    ) => __self_0 == __arg1_0,
                    (
                        ColliderType::Circle { radius: __self_0 },
                        ColliderType::Circle { radius: __arg1_0 },
                    ) => __self_0 == __arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    pub struct Collider {
        pub t: ColliderType,
        pub off: Offset2,
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for Collider {}
    #[automatically_derived]
    impl ::core::clone::Clone for Collider {
        #[inline]
        fn clone(&self) -> Collider {
            let _: ::core::clone::AssertParamIsClone<ColliderType>;
            let _: ::core::clone::AssertParamIsClone<Offset2>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Collider {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Collider {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Collider",
                "t",
                &self.t,
                "off",
                &&self.off,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Collider {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Collider {
        #[inline]
        fn eq(&self, other: &Collider) -> bool {
            self.t == other.t && self.off == other.off
        }
    }
    impl Collider {
        pub fn overlapping_with(
            self,
            self_base_pos: Vec2,
            other: Self,
            other_base_pos: Vec2,
        ) -> bool {
            match (self.t, other.t) {
                (ColliderType::Rect { size: _ }, ColliderType::Rect { size: _ }) => {
                    self.bounding_boxes_collide(self_base_pos, other, other_base_pos)
                }
                (
                    ColliderType::Circle { radius: self_radius },
                    ColliderType::Circle { radius: other_radius },
                ) => {
                    if !self.bounding_boxes_collide(self_base_pos, other, other_base_pos)
                    {
                        return false;
                    }
                    let self_center = self_base_pos + self.off;
                    let other_center = other_base_pos + other.off;
                    self_center.dist(other_center) <= (self_radius + other_radius)
                }
                (ColliderType::Circle { radius: _ }, ColliderType::Rect { size: _ }) => {
                    Self::circle_on_rect_overlapping(
                        other,
                        other_base_pos,
                        self,
                        self_base_pos,
                    )
                }
                (ColliderType::Rect { size: _ }, ColliderType::Circle { radius: _ }) => {
                    Self::circle_on_rect_overlapping(
                        self,
                        self_base_pos,
                        other,
                        other_base_pos,
                    )
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
                    {
                        ::std::io::_print(
                            format_args!(
                                "({0}, {1}): {2:?},{3:?}\n",
                                overlap_x,
                                overlap_y,
                                a_center,
                                b_center,
                            ),
                        );
                    };
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
                    if !self.bounding_boxes_collide(self_base_pos, other, other_base_pos)
                    {
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
                    Self::circle_on_rect_overlap(
                        other,
                        other_base_pos,
                        self,
                        self_base_pos,
                    )
                }
                (ColliderType::Rect { size: _ }, ColliderType::Circle { radius: _ }) => {
                    Self::circle_on_rect_overlap(
                        self,
                        self_base_pos,
                        other,
                        other_base_pos,
                    )
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
            let p = circle
                .center(circle_base_pos)
                .clamp(
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
            let p = circle
                .center(circle_base_pos)
                .clamp(
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
                ColliderType::Circle { radius } => Offset2 { x: radius, y: radius },
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
    }
    #[automatically_derived]
    impl ::core::clone::Clone for StateData {
        #[inline]
        fn clone(&self) -> StateData {
            match self {
                StateData::Vec2(__self_0) => {
                    StateData::Vec2(::core::clone::Clone::clone(__self_0))
                }
                StateData::Offset2(__self_0) => {
                    StateData::Offset2(::core::clone::Clone::clone(__self_0))
                }
                StateData::ObjectRef(__self_0) => {
                    StateData::ObjectRef(::core::clone::Clone::clone(__self_0))
                }
                StateData::RoomRef(__self_0) => {
                    StateData::RoomRef(::core::clone::Clone::clone(__self_0))
                }
                StateData::Color(__self_0) => {
                    StateData::Color(::core::clone::Clone::clone(__self_0))
                }
                StateData::Float(__self_0) => {
                    StateData::Float(::core::clone::Clone::clone(__self_0))
                }
                StateData::Int(__self_0) => {
                    StateData::Int(::core::clone::Clone::clone(__self_0))
                }
                StateData::Uint(__self_0) => {
                    StateData::Uint(::core::clone::Clone::clone(__self_0))
                }
                StateData::Bool(__self_0) => {
                    StateData::Bool(::core::clone::Clone::clone(__self_0))
                }
                StateData::Duration(__self_0) => {
                    StateData::Duration(::core::clone::Clone::clone(__self_0))
                }
                StateData::String(__self_0) => {
                    StateData::String(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StateData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                StateData::Vec2(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Vec2",
                        &__self_0,
                    )
                }
                StateData::Offset2(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Offset2",
                        &__self_0,
                    )
                }
                StateData::ObjectRef(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ObjectRef",
                        &__self_0,
                    )
                }
                StateData::RoomRef(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RoomRef",
                        &__self_0,
                    )
                }
                StateData::Color(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Color",
                        &__self_0,
                    )
                }
                StateData::Float(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Float",
                        &__self_0,
                    )
                }
                StateData::Int(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Int",
                        &__self_0,
                    )
                }
                StateData::Uint(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Uint",
                        &__self_0,
                    )
                }
                StateData::Bool(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Bool",
                        &__self_0,
                    )
                }
                StateData::Duration(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Duration",
                        &__self_0,
                    )
                }
                StateData::String(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "String",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    ///An archived [`StateData`]
    #[bytecheck(crate = ::rkyv::bytecheck)]
    #[repr(u8)]
    pub enum ArchivedStateData
    where
        Vec2: ::rkyv::Archive,
        Offset2: ::rkyv::Archive,
        ObjectRef: ::rkyv::Archive,
        RoomRef: ::rkyv::Archive,
        Color: ::rkyv::Archive,
        f32: ::rkyv::Archive,
        isize: ::rkyv::Archive,
        usize: ::rkyv::Archive,
        bool: ::rkyv::Archive,
        Duration: ::rkyv::Archive,
        String: ::rkyv::Archive,
    {
        ///The archived counterpart of [`StateData::Vec2`]
        #[allow(dead_code)]
        Vec2(
            ///The archived counterpart of [`StateData::Vec2::0`]
            <Vec2 as ::rkyv::Archive>::Archived,
        ),
        ///The archived counterpart of [`StateData::Offset2`]
        #[allow(dead_code)]
        Offset2(
            ///The archived counterpart of [`StateData::Offset2::0`]
            <Offset2 as ::rkyv::Archive>::Archived,
        ),
        ///The archived counterpart of [`StateData::ObjectRef`]
        #[allow(dead_code)]
        ObjectRef(
            ///The archived counterpart of [`StateData::ObjectRef::0`]
            <ObjectRef as ::rkyv::Archive>::Archived,
        ),
        ///The archived counterpart of [`StateData::RoomRef`]
        #[allow(dead_code)]
        RoomRef(
            ///The archived counterpart of [`StateData::RoomRef::0`]
            <RoomRef as ::rkyv::Archive>::Archived,
        ),
        ///The archived counterpart of [`StateData::Color`]
        #[allow(dead_code)]
        Color(
            ///The archived counterpart of [`StateData::Color::0`]
            <Color as ::rkyv::Archive>::Archived,
        ),
        ///The archived counterpart of [`StateData::Float`]
        #[allow(dead_code)]
        Float(
            ///The archived counterpart of [`StateData::Float::0`]
            <f32 as ::rkyv::Archive>::Archived,
        ),
        ///The archived counterpart of [`StateData::Int`]
        #[allow(dead_code)]
        Int(
            ///The archived counterpart of [`StateData::Int::0`]
            <isize as ::rkyv::Archive>::Archived,
        ),
        ///The archived counterpart of [`StateData::Uint`]
        #[allow(dead_code)]
        Uint(
            ///The archived counterpart of [`StateData::Uint::0`]
            <usize as ::rkyv::Archive>::Archived,
        ),
        ///The archived counterpart of [`StateData::Bool`]
        #[allow(dead_code)]
        Bool(
            ///The archived counterpart of [`StateData::Bool::0`]
            <bool as ::rkyv::Archive>::Archived,
        ),
        ///The archived counterpart of [`StateData::Duration`]
        #[allow(dead_code)]
        Duration(
            ///The archived counterpart of [`StateData::Duration::0`]
            <Duration as ::rkyv::Archive>::Archived,
        ),
        ///The archived counterpart of [`StateData::String`]
        #[allow(dead_code)]
        String(
            ///The archived counterpart of [`StateData::String::0`]
            <String as ::rkyv::Archive>::Archived,
        ),
    }
    const _: () = {
        #[repr(u8)]
        enum Tag {
            Vec2,
            Offset2,
            ObjectRef,
            RoomRef,
            Color,
            Float,
            Int,
            Uint,
            Bool,
            Duration,
            String,
        }
        struct Discriminant;
        #[automatically_derived]
        impl Discriminant {
            #[allow(non_upper_case_globals)]
            const Vec2: u8 = Tag::Vec2 as u8;
            #[allow(non_upper_case_globals)]
            const Offset2: u8 = Tag::Offset2 as u8;
            #[allow(non_upper_case_globals)]
            const ObjectRef: u8 = Tag::ObjectRef as u8;
            #[allow(non_upper_case_globals)]
            const RoomRef: u8 = Tag::RoomRef as u8;
            #[allow(non_upper_case_globals)]
            const Color: u8 = Tag::Color as u8;
            #[allow(non_upper_case_globals)]
            const Float: u8 = Tag::Float as u8;
            #[allow(non_upper_case_globals)]
            const Int: u8 = Tag::Int as u8;
            #[allow(non_upper_case_globals)]
            const Uint: u8 = Tag::Uint as u8;
            #[allow(non_upper_case_globals)]
            const Bool: u8 = Tag::Bool as u8;
            #[allow(non_upper_case_globals)]
            const Duration: u8 = Tag::Duration as u8;
            #[allow(non_upper_case_globals)]
            const String: u8 = Tag::String as u8;
        }
        #[repr(C)]
        struct VariantVec2(
            Tag,
            <Vec2 as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedStateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct VariantOffset2(
            Tag,
            <Offset2 as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedStateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct VariantObjectRef(
            Tag,
            <ObjectRef as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedStateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct VariantRoomRef(
            Tag,
            <RoomRef as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedStateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct VariantColor(
            Tag,
            <Color as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedStateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct VariantFloat(
            Tag,
            <f32 as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedStateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct VariantInt(
            Tag,
            <isize as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedStateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct VariantUint(
            Tag,
            <usize as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedStateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct VariantBool(
            Tag,
            <bool as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedStateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct VariantDuration(
            Tag,
            <Duration as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedStateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct VariantString(
            Tag,
            <String as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedStateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[automatically_derived]
        unsafe impl<
            __C: ::rkyv::bytecheck::rancor::Fallible + ?::core::marker::Sized,
        > ::rkyv::bytecheck::CheckBytes<__C> for ArchivedStateData
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive,
            <__C as ::rkyv::bytecheck::rancor::Fallible>::Error: ::rkyv::bytecheck::rancor::Source,
            <Vec2 as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
            <Offset2 as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
            <ObjectRef as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
            <RoomRef as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
            <Color as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
            <f32 as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
            <isize as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
            <usize as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
            <bool as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
            <Duration as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
            <String as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
        {
            unsafe fn check_bytes(
                value: *const Self,
                context: &mut __C,
            ) -> ::core::result::Result<
                (),
                <__C as ::rkyv::bytecheck::rancor::Fallible>::Error,
            > {
                let tag = *value.cast::<u8>();
                match tag {
                    Discriminant::Vec2 => {
                        let value = value.cast::<VariantVec2>();
                        <<Vec2 as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedStateData",
                                        variant_name: "Vec2",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    Discriminant::Offset2 => {
                        let value = value.cast::<VariantOffset2>();
                        <<Offset2 as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedStateData",
                                        variant_name: "Offset2",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    Discriminant::ObjectRef => {
                        let value = value.cast::<VariantObjectRef>();
                        <<ObjectRef as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedStateData",
                                        variant_name: "ObjectRef",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    Discriminant::RoomRef => {
                        let value = value.cast::<VariantRoomRef>();
                        <<RoomRef as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedStateData",
                                        variant_name: "RoomRef",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    Discriminant::Color => {
                        let value = value.cast::<VariantColor>();
                        <<Color as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedStateData",
                                        variant_name: "Color",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    Discriminant::Float => {
                        let value = value.cast::<VariantFloat>();
                        <<f32 as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedStateData",
                                        variant_name: "Float",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    Discriminant::Int => {
                        let value = value.cast::<VariantInt>();
                        <<isize as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedStateData",
                                        variant_name: "Int",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    Discriminant::Uint => {
                        let value = value.cast::<VariantUint>();
                        <<usize as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedStateData",
                                        variant_name: "Uint",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    Discriminant::Bool => {
                        let value = value.cast::<VariantBool>();
                        <<bool as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedStateData",
                                        variant_name: "Bool",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    Discriminant::Duration => {
                        let value = value.cast::<VariantDuration>();
                        <<Duration as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedStateData",
                                        variant_name: "Duration",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    Discriminant::String => {
                        let value = value.cast::<VariantString>();
                        <<String as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedStateData",
                                        variant_name: "String",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    _ => {
                        return ::core::result::Result::Err(
                            <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Source>::new(::rkyv::bytecheck::InvalidEnumDiscriminantError {
                                enum_name: "ArchivedStateData",
                                invalid_discriminant: tag,
                            }),
                        );
                    }
                }
                ::core::result::Result::Ok(())
            }
        }
    };
    #[automatically_derived]
    ///The resolver for an archived [`StateData`]
    pub enum StateDataResolver
    where
        Vec2: ::rkyv::Archive,
        Offset2: ::rkyv::Archive,
        ObjectRef: ::rkyv::Archive,
        RoomRef: ::rkyv::Archive,
        Color: ::rkyv::Archive,
        f32: ::rkyv::Archive,
        isize: ::rkyv::Archive,
        usize: ::rkyv::Archive,
        bool: ::rkyv::Archive,
        Duration: ::rkyv::Archive,
        String: ::rkyv::Archive,
    {
        ///The resolver for [`StateData::Vec2`]
        #[allow(dead_code)]
        Vec2(<Vec2 as ::rkyv::Archive>::Resolver),
        ///The resolver for [`StateData::Offset2`]
        #[allow(dead_code)]
        Offset2(<Offset2 as ::rkyv::Archive>::Resolver),
        ///The resolver for [`StateData::ObjectRef`]
        #[allow(dead_code)]
        ObjectRef(<ObjectRef as ::rkyv::Archive>::Resolver),
        ///The resolver for [`StateData::RoomRef`]
        #[allow(dead_code)]
        RoomRef(<RoomRef as ::rkyv::Archive>::Resolver),
        ///The resolver for [`StateData::Color`]
        #[allow(dead_code)]
        Color(<Color as ::rkyv::Archive>::Resolver),
        ///The resolver for [`StateData::Float`]
        #[allow(dead_code)]
        Float(<f32 as ::rkyv::Archive>::Resolver),
        ///The resolver for [`StateData::Int`]
        #[allow(dead_code)]
        Int(<isize as ::rkyv::Archive>::Resolver),
        ///The resolver for [`StateData::Uint`]
        #[allow(dead_code)]
        Uint(<usize as ::rkyv::Archive>::Resolver),
        ///The resolver for [`StateData::Bool`]
        #[allow(dead_code)]
        Bool(<bool as ::rkyv::Archive>::Resolver),
        ///The resolver for [`StateData::Duration`]
        #[allow(dead_code)]
        Duration(<Duration as ::rkyv::Archive>::Resolver),
        ///The resolver for [`StateData::String`]
        #[allow(dead_code)]
        String(<String as ::rkyv::Archive>::Resolver),
    }
    const _: () = {
        #[repr(u8)]
        enum ArchivedTag {
            Vec2,
            Offset2,
            ObjectRef,
            RoomRef,
            Color,
            Float,
            Int,
            Uint,
            Bool,
            Duration,
            String,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArchivedTag {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTag {
            #[inline]
            fn eq(&self, other: &ArchivedTag) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArchivedTag {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArchivedTag,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
            }
        }
        #[repr(C)]
        struct ArchivedVariantVec2(
            ArchivedTag,
            <Vec2 as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<StateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct ArchivedVariantOffset2(
            ArchivedTag,
            <Offset2 as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<StateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct ArchivedVariantObjectRef(
            ArchivedTag,
            <ObjectRef as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<StateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct ArchivedVariantRoomRef(
            ArchivedTag,
            <RoomRef as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<StateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct ArchivedVariantColor(
            ArchivedTag,
            <Color as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<StateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct ArchivedVariantFloat(
            ArchivedTag,
            <f32 as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<StateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct ArchivedVariantInt(
            ArchivedTag,
            <isize as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<StateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct ArchivedVariantUint(
            ArchivedTag,
            <usize as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<StateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct ArchivedVariantBool(
            ArchivedTag,
            <bool as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<StateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct ArchivedVariantDuration(
            ArchivedTag,
            <Duration as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<StateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        #[repr(C)]
        struct ArchivedVariantString(
            ArchivedTag,
            <String as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<StateData>,
        )
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive;
        impl ::rkyv::Archive for StateData
        where
            Vec2: ::rkyv::Archive,
            Offset2: ::rkyv::Archive,
            ObjectRef: ::rkyv::Archive,
            RoomRef: ::rkyv::Archive,
            Color: ::rkyv::Archive,
            f32: ::rkyv::Archive,
            isize: ::rkyv::Archive,
            usize: ::rkyv::Archive,
            bool: ::rkyv::Archive,
            Duration: ::rkyv::Archive,
            String: ::rkyv::Archive,
        {
            type Archived = ArchivedStateData;
            type Resolver = StateDataResolver;
            #[allow(clippy::unit_arg)]
            fn resolve(
                &self,
                resolver: <Self as ::rkyv::Archive>::Resolver,
                out: ::rkyv::Place<<Self as ::rkyv::Archive>::Archived>,
            ) {
                let __this = self;
                match resolver {
                    StateDataResolver::Vec2(resolver_0) => {
                        match __this {
                            StateData::Vec2(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantVec2>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::Vec2);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <Vec2 as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                    StateDataResolver::Offset2(resolver_0) => {
                        match __this {
                            StateData::Offset2(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantOffset2>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::Offset2);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <Offset2 as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                    StateDataResolver::ObjectRef(resolver_0) => {
                        match __this {
                            StateData::ObjectRef(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantObjectRef>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::ObjectRef);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <ObjectRef as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                    StateDataResolver::RoomRef(resolver_0) => {
                        match __this {
                            StateData::RoomRef(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantRoomRef>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::RoomRef);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <RoomRef as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                    StateDataResolver::Color(resolver_0) => {
                        match __this {
                            StateData::Color(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantColor>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::Color);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <Color as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                    StateDataResolver::Float(resolver_0) => {
                        match __this {
                            StateData::Float(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantFloat>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::Float);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <f32 as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                    StateDataResolver::Int(resolver_0) => {
                        match __this {
                            StateData::Int(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantInt>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::Int);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <isize as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                    StateDataResolver::Uint(resolver_0) => {
                        match __this {
                            StateData::Uint(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantUint>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::Uint);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <usize as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                    StateDataResolver::Bool(resolver_0) => {
                        match __this {
                            StateData::Bool(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantBool>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::Bool);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <bool as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                    StateDataResolver::Duration(resolver_0) => {
                        match __this {
                            StateData::Duration(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantDuration>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::Duration);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <Duration as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                    StateDataResolver::String(resolver_0) => {
                        match __this {
                            StateData::String(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantString>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::String);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <String as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                }
            }
        }
    };
    unsafe impl ::rkyv::traits::Portable for ArchivedStateData
    where
        Vec2: ::rkyv::Archive,
        Offset2: ::rkyv::Archive,
        ObjectRef: ::rkyv::Archive,
        RoomRef: ::rkyv::Archive,
        Color: ::rkyv::Archive,
        f32: ::rkyv::Archive,
        isize: ::rkyv::Archive,
        usize: ::rkyv::Archive,
        bool: ::rkyv::Archive,
        Duration: ::rkyv::Archive,
        String: ::rkyv::Archive,
        <Vec2 as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <Offset2 as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <ObjectRef as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <RoomRef as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <Color as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <f32 as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <isize as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <usize as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <bool as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <Duration as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <String as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
    {}
    #[automatically_derived]
    impl<__D: ::rkyv::rancor::Fallible + ?Sized> ::rkyv::Deserialize<StateData, __D>
    for ::rkyv::Archived<StateData>
    where
        Vec2: ::rkyv::Archive,
        <Vec2 as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<Vec2, __D>,
        Offset2: ::rkyv::Archive,
        <Offset2 as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<Offset2, __D>,
        ObjectRef: ::rkyv::Archive,
        <ObjectRef as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<ObjectRef, __D>,
        RoomRef: ::rkyv::Archive,
        <RoomRef as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<RoomRef, __D>,
        Color: ::rkyv::Archive,
        <Color as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<Color, __D>,
        f32: ::rkyv::Archive,
        <f32 as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<f32, __D>,
        isize: ::rkyv::Archive,
        <isize as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<isize, __D>,
        usize: ::rkyv::Archive,
        <usize as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<usize, __D>,
        bool: ::rkyv::Archive,
        <bool as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<bool, __D>,
        Duration: ::rkyv::Archive,
        <Duration as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<Duration, __D>,
        String: ::rkyv::Archive,
        <String as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<String, __D>,
    {
        fn deserialize(
            &self,
            deserializer: &mut __D,
        ) -> ::core::result::Result<
            StateData,
            <__D as ::rkyv::rancor::Fallible>::Error,
        > {
            let __this = self;
            ::core::result::Result::Ok(
                match __this {
                    Self::Vec2(_0, ..) => {
                        StateData::Vec2(
                            <<Vec2 as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                Vec2,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                    Self::Offset2(_0, ..) => {
                        StateData::Offset2(
                            <<Offset2 as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                Offset2,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                    Self::ObjectRef(_0, ..) => {
                        StateData::ObjectRef(
                            <<ObjectRef as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                ObjectRef,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                    Self::RoomRef(_0, ..) => {
                        StateData::RoomRef(
                            <<RoomRef as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                RoomRef,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                    Self::Color(_0, ..) => {
                        StateData::Color(
                            <<Color as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                Color,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                    Self::Float(_0, ..) => {
                        StateData::Float(
                            <<f32 as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                f32,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                    Self::Int(_0, ..) => {
                        StateData::Int(
                            <<isize as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                isize,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                    Self::Uint(_0, ..) => {
                        StateData::Uint(
                            <<usize as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                usize,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                    Self::Bool(_0, ..) => {
                        StateData::Bool(
                            <<bool as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                bool,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                    Self::Duration(_0, ..) => {
                        StateData::Duration(
                            <<Duration as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                Duration,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                    Self::String(_0, ..) => {
                        StateData::String(
                            <<String as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                String,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                },
            )
        }
    }
    #[automatically_derived]
    impl<__S: ::rkyv::rancor::Fallible + ?Sized> ::rkyv::Serialize<__S> for StateData
    where
        Vec2: ::rkyv::Serialize<__S>,
        Offset2: ::rkyv::Serialize<__S>,
        ObjectRef: ::rkyv::Serialize<__S>,
        RoomRef: ::rkyv::Serialize<__S>,
        Color: ::rkyv::Serialize<__S>,
        f32: ::rkyv::Serialize<__S>,
        isize: ::rkyv::Serialize<__S>,
        usize: ::rkyv::Serialize<__S>,
        bool: ::rkyv::Serialize<__S>,
        Duration: ::rkyv::Serialize<__S>,
        String: ::rkyv::Serialize<__S>,
    {
        fn serialize(
            &self,
            serializer: &mut __S,
        ) -> ::core::result::Result<
            <Self as ::rkyv::Archive>::Resolver,
            <__S as ::rkyv::rancor::Fallible>::Error,
        > {
            let __this = self;
            ::core::result::Result::Ok(
                match __this {
                    StateData::Vec2(_0, ..) => {
                        StateDataResolver::Vec2(
                            <Vec2 as ::rkyv::Serialize<__S>>::serialize(_0, serializer)?,
                        )
                    }
                    StateData::Offset2(_0, ..) => {
                        StateDataResolver::Offset2(
                            <Offset2 as ::rkyv::Serialize<
                                __S,
                            >>::serialize(_0, serializer)?,
                        )
                    }
                    StateData::ObjectRef(_0, ..) => {
                        StateDataResolver::ObjectRef(
                            <ObjectRef as ::rkyv::Serialize<
                                __S,
                            >>::serialize(_0, serializer)?,
                        )
                    }
                    StateData::RoomRef(_0, ..) => {
                        StateDataResolver::RoomRef(
                            <RoomRef as ::rkyv::Serialize<
                                __S,
                            >>::serialize(_0, serializer)?,
                        )
                    }
                    StateData::Color(_0, ..) => {
                        StateDataResolver::Color(
                            <Color as ::rkyv::Serialize<__S>>::serialize(_0, serializer)?,
                        )
                    }
                    StateData::Float(_0, ..) => {
                        StateDataResolver::Float(
                            <f32 as ::rkyv::Serialize<__S>>::serialize(_0, serializer)?,
                        )
                    }
                    StateData::Int(_0, ..) => {
                        StateDataResolver::Int(
                            <isize as ::rkyv::Serialize<__S>>::serialize(_0, serializer)?,
                        )
                    }
                    StateData::Uint(_0, ..) => {
                        StateDataResolver::Uint(
                            <usize as ::rkyv::Serialize<__S>>::serialize(_0, serializer)?,
                        )
                    }
                    StateData::Bool(_0, ..) => {
                        StateDataResolver::Bool(
                            <bool as ::rkyv::Serialize<__S>>::serialize(_0, serializer)?,
                        )
                    }
                    StateData::Duration(_0, ..) => {
                        StateDataResolver::Duration(
                            <Duration as ::rkyv::Serialize<
                                __S,
                            >>::serialize(_0, serializer)?,
                        )
                    }
                    StateData::String(_0, ..) => {
                        StateDataResolver::String(
                            <String as ::rkyv::Serialize<
                                __S,
                            >>::serialize(_0, serializer)?,
                        )
                    }
                },
            )
        }
    }
    impl StateData {
        fn deserialize<D: rkyv::rancor::Fallible + ?Sized>(
            &self,
            deserializer: &mut D,
            ctx: &Ctx,
        ) -> Result<Self, D::Error> {}
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
                Float(v) => v.to_bits().hash(hasher),
            }
        }
    }
    impl From<Vec2> for StateData {
        fn from(v: Vec2) -> StateData {
            StateData::Vec2(v)
        }
    }
    impl TryFrom<StateData> for Vec2 {
        type Error = ();
        fn try_from(v: StateData) -> Result<Vec2, ()> {
            match v {
                StateData::Vec2(v) => Ok(v),
                _ => Err(()),
            }
        }
    }
    impl From<Offset2> for StateData {
        fn from(v: Offset2) -> StateData {
            StateData::Offset2(v)
        }
    }
    impl TryFrom<StateData> for Offset2 {
        type Error = ();
        fn try_from(v: StateData) -> Result<Offset2, ()> {
            match v {
                StateData::Offset2(v) => Ok(v),
                _ => Err(()),
            }
        }
    }
    impl From<ObjectRef> for StateData {
        fn from(v: ObjectRef) -> StateData {
            StateData::ObjectRef(v)
        }
    }
    impl TryFrom<StateData> for ObjectRef {
        type Error = ();
        fn try_from(v: StateData) -> Result<ObjectRef, ()> {
            match v {
                StateData::ObjectRef(v) => Ok(v),
                _ => Err(()),
            }
        }
    }
    impl From<RoomRef> for StateData {
        fn from(v: RoomRef) -> StateData {
            StateData::RoomRef(v)
        }
    }
    impl TryFrom<StateData> for RoomRef {
        type Error = ();
        fn try_from(v: StateData) -> Result<RoomRef, ()> {
            match v {
                StateData::RoomRef(v) => Ok(v),
                _ => Err(()),
            }
        }
    }
    impl From<Color> for StateData {
        fn from(v: Color) -> StateData {
            StateData::Color(v)
        }
    }
    impl TryFrom<StateData> for Color {
        type Error = ();
        fn try_from(v: StateData) -> Result<Color, ()> {
            match v {
                StateData::Color(v) => Ok(v),
                _ => Err(()),
            }
        }
    }
    impl From<f32> for StateData {
        fn from(v: f32) -> StateData {
            StateData::Float(v)
        }
    }
    impl TryFrom<StateData> for f32 {
        type Error = ();
        fn try_from(v: StateData) -> Result<f32, ()> {
            match v {
                StateData::Float(v) => Ok(v),
                _ => Err(()),
            }
        }
    }
    impl From<isize> for StateData {
        fn from(v: isize) -> StateData {
            StateData::Int(v)
        }
    }
    impl TryFrom<StateData> for isize {
        type Error = ();
        fn try_from(v: StateData) -> Result<isize, ()> {
            match v {
                StateData::Int(v) => Ok(v),
                _ => Err(()),
            }
        }
    }
    impl From<usize> for StateData {
        fn from(v: usize) -> StateData {
            StateData::Uint(v)
        }
    }
    impl TryFrom<StateData> for usize {
        type Error = ();
        fn try_from(v: StateData) -> Result<usize, ()> {
            match v {
                StateData::Uint(v) => Ok(v),
                _ => Err(()),
            }
        }
    }
    impl From<bool> for StateData {
        fn from(v: bool) -> StateData {
            StateData::Bool(v)
        }
    }
    impl TryFrom<StateData> for bool {
        type Error = ();
        fn try_from(v: StateData) -> Result<bool, ()> {
            match v {
                StateData::Bool(v) => Ok(v),
                _ => Err(()),
            }
        }
    }
    impl From<String> for StateData {
        fn from(v: String) -> StateData {
            StateData::String(v)
        }
    }
    impl TryFrom<StateData> for String {
        type Error = ();
        fn try_from(v: StateData) -> Result<String, ()> {
            match v {
                StateData::String(v) => Ok(v),
                _ => Err(()),
            }
        }
    }
    impl From<Duration> for StateData {
        fn from(v: Duration) -> StateData {
            StateData::Duration(v)
        }
    }
    impl TryFrom<StateData> for Duration {
        type Error = ();
        fn try_from(v: StateData) -> Result<Duration, ()> {
            match v {
                StateData::Duration(v) => Ok(v),
                _ => Err(()),
            }
        }
    }
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
    #[automatically_derived]
    impl ::core::clone::Clone for ObjectStateKey {
        #[inline]
        fn clone(&self) -> ObjectStateKey {
            match self {
                ObjectStateKey::Pos => ObjectStateKey::Pos,
                ObjectStateKey::ZLayer => ObjectStateKey::ZLayer,
                ObjectStateKey::Rotate => ObjectStateKey::Rotate,
                ObjectStateKey::Scale => ObjectStateKey::Scale,
                ObjectStateKey::Visible => ObjectStateKey::Visible,
                ObjectStateKey::Processing => ObjectStateKey::Processing,
                ObjectStateKey::Animation => ObjectStateKey::Animation,
                ObjectStateKey::AniFrame => ObjectStateKey::AniFrame,
                ObjectStateKey::AniFrameTimer => ObjectStateKey::AniFrameTimer,
                ObjectStateKey::Playing => ObjectStateKey::Playing,
                ObjectStateKey::Other(__self_0) => {
                    ObjectStateKey::Other(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ObjectStateKey {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                ObjectStateKey::Pos => ::core::fmt::Formatter::write_str(f, "Pos"),
                ObjectStateKey::ZLayer => ::core::fmt::Formatter::write_str(f, "ZLayer"),
                ObjectStateKey::Rotate => ::core::fmt::Formatter::write_str(f, "Rotate"),
                ObjectStateKey::Scale => ::core::fmt::Formatter::write_str(f, "Scale"),
                ObjectStateKey::Visible => {
                    ::core::fmt::Formatter::write_str(f, "Visible")
                }
                ObjectStateKey::Processing => {
                    ::core::fmt::Formatter::write_str(f, "Processing")
                }
                ObjectStateKey::Animation => {
                    ::core::fmt::Formatter::write_str(f, "Animation")
                }
                ObjectStateKey::AniFrame => {
                    ::core::fmt::Formatter::write_str(f, "AniFrame")
                }
                ObjectStateKey::AniFrameTimer => {
                    ::core::fmt::Formatter::write_str(f, "AniFrameTimer")
                }
                ObjectStateKey::Playing => {
                    ::core::fmt::Formatter::write_str(f, "Playing")
                }
                ObjectStateKey::Other(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Other",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObjectStateKey {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObjectStateKey {
        #[inline]
        fn eq(&self, other: &ObjectStateKey) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        ObjectStateKey::Other(__self_0),
                        ObjectStateKey::Other(__arg1_0),
                    ) => __self_0 == __arg1_0,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObjectStateKey {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ObjectStateKey {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state);
            match self {
                ObjectStateKey::Other(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                _ => {}
            }
        }
    }
    #[automatically_derived]
    ///An archived [`ObjectStateKey`]
    #[bytecheck(crate = ::rkyv::bytecheck)]
    #[repr(u8)]
    pub enum ArchivedObjectStateKey
    where
        String: ::rkyv::Archive,
    {
        ///The archived counterpart of [`ObjectStateKey::Pos`]
        #[allow(dead_code)]
        Pos,
        ///The archived counterpart of [`ObjectStateKey::ZLayer`]
        #[allow(dead_code)]
        ZLayer,
        ///The archived counterpart of [`ObjectStateKey::Rotate`]
        #[allow(dead_code)]
        Rotate,
        ///The archived counterpart of [`ObjectStateKey::Scale`]
        #[allow(dead_code)]
        Scale,
        ///The archived counterpart of [`ObjectStateKey::Visible`]
        #[allow(dead_code)]
        Visible,
        ///The archived counterpart of [`ObjectStateKey::Processing`]
        #[allow(dead_code)]
        Processing,
        ///The archived counterpart of [`ObjectStateKey::Animation`]
        #[allow(dead_code)]
        Animation,
        ///The archived counterpart of [`ObjectStateKey::AniFrame`]
        #[allow(dead_code)]
        AniFrame,
        ///The archived counterpart of [`ObjectStateKey::AniFrameTimer`]
        #[allow(dead_code)]
        AniFrameTimer,
        ///The archived counterpart of [`ObjectStateKey::Playing`]
        #[allow(dead_code)]
        Playing,
        ///The archived counterpart of [`ObjectStateKey::Other`]
        #[allow(dead_code)]
        Other(
            ///The archived counterpart of [`ObjectStateKey::Other::0`]
            <String as ::rkyv::Archive>::Archived,
        ),
    }
    const _: () = {
        #[repr(u8)]
        enum Tag {
            Pos,
            ZLayer,
            Rotate,
            Scale,
            Visible,
            Processing,
            Animation,
            AniFrame,
            AniFrameTimer,
            Playing,
            Other,
        }
        struct Discriminant;
        #[automatically_derived]
        impl Discriminant {
            #[allow(non_upper_case_globals)]
            const Pos: u8 = Tag::Pos as u8;
            #[allow(non_upper_case_globals)]
            const ZLayer: u8 = Tag::ZLayer as u8;
            #[allow(non_upper_case_globals)]
            const Rotate: u8 = Tag::Rotate as u8;
            #[allow(non_upper_case_globals)]
            const Scale: u8 = Tag::Scale as u8;
            #[allow(non_upper_case_globals)]
            const Visible: u8 = Tag::Visible as u8;
            #[allow(non_upper_case_globals)]
            const Processing: u8 = Tag::Processing as u8;
            #[allow(non_upper_case_globals)]
            const Animation: u8 = Tag::Animation as u8;
            #[allow(non_upper_case_globals)]
            const AniFrame: u8 = Tag::AniFrame as u8;
            #[allow(non_upper_case_globals)]
            const AniFrameTimer: u8 = Tag::AniFrameTimer as u8;
            #[allow(non_upper_case_globals)]
            const Playing: u8 = Tag::Playing as u8;
            #[allow(non_upper_case_globals)]
            const Other: u8 = Tag::Other as u8;
        }
        #[repr(C)]
        struct VariantOther(
            Tag,
            <String as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ArchivedObjectStateKey>,
        )
        where
            String: ::rkyv::Archive;
        #[automatically_derived]
        unsafe impl<
            __C: ::rkyv::bytecheck::rancor::Fallible + ?::core::marker::Sized,
        > ::rkyv::bytecheck::CheckBytes<__C> for ArchivedObjectStateKey
        where
            String: ::rkyv::Archive,
            <__C as ::rkyv::bytecheck::rancor::Fallible>::Error: ::rkyv::bytecheck::rancor::Source,
            <String as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
        {
            unsafe fn check_bytes(
                value: *const Self,
                context: &mut __C,
            ) -> ::core::result::Result<
                (),
                <__C as ::rkyv::bytecheck::rancor::Fallible>::Error,
            > {
                let tag = *value.cast::<u8>();
                match tag {
                    Discriminant::Pos => {}
                    Discriminant::ZLayer => {}
                    Discriminant::Rotate => {}
                    Discriminant::Scale => {}
                    Discriminant::Visible => {}
                    Discriminant::Processing => {}
                    Discriminant::Animation => {}
                    Discriminant::AniFrame => {}
                    Discriminant::AniFrameTimer => {}
                    Discriminant::Playing => {}
                    Discriminant::Other => {
                        let value = value.cast::<VariantOther>();
                        <<String as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                            __C,
                        >>::check_bytes(&raw const (*value).1, context)
                            .map_err(|e| {
                                <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                                    e,
                                    ::rkyv::bytecheck::UnnamedEnumVariantCheckContext {
                                        enum_name: "ArchivedObjectStateKey",
                                        variant_name: "Other",
                                        field_index: 1,
                                    },
                                )
                            })?;
                    }
                    _ => {
                        return ::core::result::Result::Err(
                            <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Source>::new(::rkyv::bytecheck::InvalidEnumDiscriminantError {
                                enum_name: "ArchivedObjectStateKey",
                                invalid_discriminant: tag,
                            }),
                        );
                    }
                }
                ::core::result::Result::Ok(())
            }
        }
    };
    #[automatically_derived]
    ///The resolver for an archived [`ObjectStateKey`]
    pub enum ObjectStateKeyResolver
    where
        String: ::rkyv::Archive,
    {
        ///The resolver for [`ObjectStateKey::Pos`]
        #[allow(dead_code)]
        Pos,
        ///The resolver for [`ObjectStateKey::ZLayer`]
        #[allow(dead_code)]
        ZLayer,
        ///The resolver for [`ObjectStateKey::Rotate`]
        #[allow(dead_code)]
        Rotate,
        ///The resolver for [`ObjectStateKey::Scale`]
        #[allow(dead_code)]
        Scale,
        ///The resolver for [`ObjectStateKey::Visible`]
        #[allow(dead_code)]
        Visible,
        ///The resolver for [`ObjectStateKey::Processing`]
        #[allow(dead_code)]
        Processing,
        ///The resolver for [`ObjectStateKey::Animation`]
        #[allow(dead_code)]
        Animation,
        ///The resolver for [`ObjectStateKey::AniFrame`]
        #[allow(dead_code)]
        AniFrame,
        ///The resolver for [`ObjectStateKey::AniFrameTimer`]
        #[allow(dead_code)]
        AniFrameTimer,
        ///The resolver for [`ObjectStateKey::Playing`]
        #[allow(dead_code)]
        Playing,
        ///The resolver for [`ObjectStateKey::Other`]
        #[allow(dead_code)]
        Other(<String as ::rkyv::Archive>::Resolver),
    }
    const _: () = {
        #[repr(u8)]
        enum ArchivedTag {
            Pos,
            ZLayer,
            Rotate,
            Scale,
            Visible,
            Processing,
            Animation,
            AniFrame,
            AniFrameTimer,
            Playing,
            Other,
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ArchivedTag {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTag {
            #[inline]
            fn eq(&self, other: &ArchivedTag) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
            }
        }
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for ArchivedTag {
            #[inline]
            fn partial_cmp(
                &self,
                other: &ArchivedTag,
            ) -> ::core::option::Option<::core::cmp::Ordering> {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
            }
        }
        #[repr(C)]
        struct ArchivedVariantOther(
            ArchivedTag,
            <String as ::rkyv::Archive>::Archived,
            ::core::marker::PhantomData<ObjectStateKey>,
        )
        where
            String: ::rkyv::Archive;
        impl ::rkyv::Archive for ObjectStateKey
        where
            String: ::rkyv::Archive,
        {
            type Archived = ArchivedObjectStateKey;
            type Resolver = ObjectStateKeyResolver;
            #[allow(clippy::unit_arg)]
            fn resolve(
                &self,
                resolver: <Self as ::rkyv::Archive>::Resolver,
                out: ::rkyv::Place<<Self as ::rkyv::Archive>::Archived>,
            ) {
                let __this = self;
                match resolver {
                    ObjectStateKeyResolver::Pos => {
                        let out = unsafe { out.cast_unchecked::<ArchivedTag>() };
                        unsafe {
                            out.write_unchecked(ArchivedTag::Pos);
                        }
                    }
                    ObjectStateKeyResolver::ZLayer => {
                        let out = unsafe { out.cast_unchecked::<ArchivedTag>() };
                        unsafe {
                            out.write_unchecked(ArchivedTag::ZLayer);
                        }
                    }
                    ObjectStateKeyResolver::Rotate => {
                        let out = unsafe { out.cast_unchecked::<ArchivedTag>() };
                        unsafe {
                            out.write_unchecked(ArchivedTag::Rotate);
                        }
                    }
                    ObjectStateKeyResolver::Scale => {
                        let out = unsafe { out.cast_unchecked::<ArchivedTag>() };
                        unsafe {
                            out.write_unchecked(ArchivedTag::Scale);
                        }
                    }
                    ObjectStateKeyResolver::Visible => {
                        let out = unsafe { out.cast_unchecked::<ArchivedTag>() };
                        unsafe {
                            out.write_unchecked(ArchivedTag::Visible);
                        }
                    }
                    ObjectStateKeyResolver::Processing => {
                        let out = unsafe { out.cast_unchecked::<ArchivedTag>() };
                        unsafe {
                            out.write_unchecked(ArchivedTag::Processing);
                        }
                    }
                    ObjectStateKeyResolver::Animation => {
                        let out = unsafe { out.cast_unchecked::<ArchivedTag>() };
                        unsafe {
                            out.write_unchecked(ArchivedTag::Animation);
                        }
                    }
                    ObjectStateKeyResolver::AniFrame => {
                        let out = unsafe { out.cast_unchecked::<ArchivedTag>() };
                        unsafe {
                            out.write_unchecked(ArchivedTag::AniFrame);
                        }
                    }
                    ObjectStateKeyResolver::AniFrameTimer => {
                        let out = unsafe { out.cast_unchecked::<ArchivedTag>() };
                        unsafe {
                            out.write_unchecked(ArchivedTag::AniFrameTimer);
                        }
                    }
                    ObjectStateKeyResolver::Playing => {
                        let out = unsafe { out.cast_unchecked::<ArchivedTag>() };
                        unsafe {
                            out.write_unchecked(ArchivedTag::Playing);
                        }
                    }
                    ObjectStateKeyResolver::Other(resolver_0) => {
                        match __this {
                            ObjectStateKey::Other(self_0, ..) => {
                                let out = unsafe {
                                    out.cast_unchecked::<ArchivedVariantOther>()
                                };
                                let tag_ptr = unsafe { &raw mut (*out.ptr()).0 };
                                unsafe {
                                    tag_ptr.write(ArchivedTag::Other);
                                }
                                let field_ptr = unsafe { &raw mut (*out.ptr()).1 };
                                let out_field = unsafe {
                                    ::rkyv::Place::from_field_unchecked(out, field_ptr)
                                };
                                <String as ::rkyv::Archive>::resolve(
                                    self_0,
                                    resolver_0,
                                    out_field,
                                );
                            }
                            #[allow(unreachable_patterns)]
                            _ => unsafe { ::core::hint::unreachable_unchecked() }
                        }
                    }
                }
            }
        }
    };
    unsafe impl ::rkyv::traits::Portable for ArchivedObjectStateKey
    where
        String: ::rkyv::Archive,
        <String as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
    {}
    #[automatically_derived]
    impl<__D: ::rkyv::rancor::Fallible + ?Sized> ::rkyv::Deserialize<ObjectStateKey, __D>
    for ::rkyv::Archived<ObjectStateKey>
    where
        String: ::rkyv::Archive,
        <String as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<String, __D>,
    {
        fn deserialize(
            &self,
            deserializer: &mut __D,
        ) -> ::core::result::Result<
            ObjectStateKey,
            <__D as ::rkyv::rancor::Fallible>::Error,
        > {
            let __this = self;
            ::core::result::Result::Ok(
                match __this {
                    Self::Pos => ObjectStateKey::Pos,
                    Self::ZLayer => ObjectStateKey::ZLayer,
                    Self::Rotate => ObjectStateKey::Rotate,
                    Self::Scale => ObjectStateKey::Scale,
                    Self::Visible => ObjectStateKey::Visible,
                    Self::Processing => ObjectStateKey::Processing,
                    Self::Animation => ObjectStateKey::Animation,
                    Self::AniFrame => ObjectStateKey::AniFrame,
                    Self::AniFrameTimer => ObjectStateKey::AniFrameTimer,
                    Self::Playing => ObjectStateKey::Playing,
                    Self::Other(_0, ..) => {
                        ObjectStateKey::Other(
                            <<String as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                                String,
                                __D,
                            >>::deserialize(_0, deserializer)?,
                        )
                    }
                },
            )
        }
    }
    #[automatically_derived]
    impl<__S: ::rkyv::rancor::Fallible + ?Sized> ::rkyv::Serialize<__S>
    for ObjectStateKey
    where
        String: ::rkyv::Serialize<__S>,
    {
        fn serialize(
            &self,
            serializer: &mut __S,
        ) -> ::core::result::Result<
            <Self as ::rkyv::Archive>::Resolver,
            <__S as ::rkyv::rancor::Fallible>::Error,
        > {
            let __this = self;
            ::core::result::Result::Ok(
                match __this {
                    ObjectStateKey::Pos => ObjectStateKeyResolver::Pos,
                    ObjectStateKey::ZLayer => ObjectStateKeyResolver::ZLayer,
                    ObjectStateKey::Rotate => ObjectStateKeyResolver::Rotate,
                    ObjectStateKey::Scale => ObjectStateKeyResolver::Scale,
                    ObjectStateKey::Visible => ObjectStateKeyResolver::Visible,
                    ObjectStateKey::Processing => ObjectStateKeyResolver::Processing,
                    ObjectStateKey::Animation => ObjectStateKeyResolver::Animation,
                    ObjectStateKey::AniFrame => ObjectStateKeyResolver::AniFrame,
                    ObjectStateKey::AniFrameTimer => {
                        ObjectStateKeyResolver::AniFrameTimer
                    }
                    ObjectStateKey::Playing => ObjectStateKeyResolver::Playing,
                    ObjectStateKey::Other(_0, ..) => {
                        ObjectStateKeyResolver::Other(
                            <String as ::rkyv::Serialize<
                                __S,
                            >>::serialize(_0, serializer)?,
                        )
                    }
                },
            )
        }
    }
    impl<T: AsRef<str>> From<T> for ObjectStateKey {
        fn from(val: T) -> Self {
            match val.as_ref() {
                "_zr.pos" => Self::Pos,
                "_zr.rot" => Self::Rotate,
                "_zr.scl" => Self::Scale,
                "_zr.vis" => Self::Visible,
                "_zr.prc" => Self::Processing,
                "_zr.ani" => Self::Animation,
                "_zr.anf" => Self::AniFrame,
                "_zr.pla" => Self::Playing,
                "_zr.aft" => Self::AniFrameTimer,
                "_zr.zlr" => Self::ZLayer,
                v => Self::Other(v.to_string()),
            }
        }
    }
    impl Display for ObjectStateKey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Pos => f.write_str("_zr.pos"),
                Self::Rotate => f.write_str("_zr.rot"),
                Self::Scale => f.write_str("_zr.scl"),
                Self::Visible => f.write_str("_zr.vis"),
                Self::Processing => f.write_str("_zr.prc"),
                Self::Animation => f.write_str("_zr.ani"),
                Self::AniFrame => f.write_str("_zr.anf"),
                Self::Playing => f.write_str("_zr.pla"),
                Self::AniFrameTimer => f.write_str("_zr.aft"),
                Self::ZLayer => f.write_str("_zr.zlr"),
                Self::Other(v) => f.write_str(&v),
            }
        }
    }
    pub struct ObjectState(HashMap<ObjectStateKey, StateData>, bool);
    #[automatically_derived]
    impl ::core::clone::Clone for ObjectState {
        #[inline]
        fn clone(&self) -> ObjectState {
            ObjectState(
                ::core::clone::Clone::clone(&self.0),
                ::core::clone::Clone::clone(&self.1),
            )
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ObjectState {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field2_finish(
                f,
                "ObjectState",
                &self.0,
                &&self.1,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ObjectState {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ObjectState {
        #[inline]
        fn eq(&self, other: &ObjectState) -> bool {
            self.1 == other.1 && self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for ObjectState {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<HashMap<ObjectStateKey, StateData>>;
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for ObjectState {
        #[inline]
        fn default() -> ObjectState {
            ObjectState(
                ::core::default::Default::default(),
                ::core::default::Default::default(),
            )
        }
    }
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
        /// Flatten a single ObjectState into this one with the specified namespace.
        /// The namespace should not end with a ., but realisticly it doesn't really
        /// matter.
        #[track_caller]
        pub(crate) fn flatten(&mut self, mut namespace: String, other: &ObjectState) {
            if !self.1 {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("May not flatten non-flattening state"),
                    );
                }
            }
            namespace.push('.');
            for (k, v) in other.iter() {
                self.set(
                    ObjectStateKey::Other(namespace.clone() + &k.to_string()),
                    v.clone(),
                );
            }
        }
        pub(crate) fn unflatten(
            &self,
            namespace: impl AsRef<str>,
            out: &mut ObjectState,
        ) {
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
            if !self.1 && let ObjectStateKey::Other(v) = &key && v.contains('.') {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Object state keys may not include dots due to flattening rules",
                        ),
                    );
                }
            }
            self.0.insert(key, val.into());
        }
        pub fn iter<'a>(
            &'a self,
        ) -> impl Iterator<Item = (&'a ObjectStateKey, &'a StateData)> + 'a {
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
    #[allow(deprecated)]
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl derive_more::core::fmt::Debug for World {
        #[inline]
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            let current_room = &self.current_room;
            let ctx = &self.ctx;
            let extra_objs = &self.extra_objs;
            let player = &self.player;
            let room_transition = &self.room_transition;
            let callbacks = &self.callbacks;
            let state = &self.state;
            let camera_obj = &self.camera_obj;
            let input_mappings = &self.input_mappings;
            let lang = &self.lang;
            let text = &self.text;
            let event_queue = &self.event_queue;
            let internal_event_queue = &self.internal_event_queue;
            let current_frame_presses = &self.current_frame_presses;
            let audio_handle = &self.audio_handle;
            derive_more::core::fmt::DebugStruct::finish_non_exhaustive(
                derive_more::core::fmt::DebugStruct::field(
                    derive_more::core::fmt::DebugStruct::field(
                        derive_more::core::fmt::DebugStruct::field(
                            derive_more::core::fmt::DebugStruct::field(
                                derive_more::core::fmt::DebugStruct::field(
                                    derive_more::core::fmt::DebugStruct::field(
                                        derive_more::core::fmt::DebugStruct::field(
                                            derive_more::core::fmt::DebugStruct::field(
                                                derive_more::core::fmt::DebugStruct::field(
                                                    derive_more::core::fmt::DebugStruct::field(
                                                        &mut derive_more::core::fmt::Formatter::debug_struct(
                                                            __derive_more_f,
                                                            "World",
                                                        ),
                                                        "current_room",
                                                        &current_room,
                                                    ),
                                                    "ctx",
                                                    &ctx,
                                                ),
                                                "extra_objs",
                                                &extra_objs,
                                            ),
                                            "player",
                                            &player,
                                        ),
                                        "room_transition",
                                        &room_transition,
                                    ),
                                    "callbacks",
                                    &callbacks,
                                ),
                                "state",
                                &state,
                            ),
                            "camera_obj",
                            &camera_obj,
                        ),
                        "input_mappings",
                        &input_mappings,
                    ),
                    "lang",
                    &lang,
                ),
            )
        }
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
        ) -> Self {
            let mut handle = rodio::DeviceSinkBuilder::open_default_sink()
                .expect("open default audio stream");
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
                event_queue: ::alloc::vec::Vec::new(),
                internal_event_queue: ::alloc::vec::Vec::new(),
                current_frame_presses: HashMap::new(),
                audio_handle: handle,
                text: HashSet::new(),
            }
        }
        pub fn show_text(&mut self, text: TextRef) {
            self.text.insert(text);
        }
        pub fn hide_text(&mut self, text: TextRef) {
            self.text.remove(&text);
        }
        pub fn transition_room(&mut self, entry: EntryRef) {
            self.internal_event_queue.push(InternalEvent::RoomTransition(entry));
        }
        /// Begin/resume playing some audio. When beginning playing it, the source
        /// will be consumed and replaced with `None`!
        pub fn play_audio(&mut self, audio: AudioRef) {
            self.internal_event_queue.push(InternalEvent::PlayAudio(audio));
        }
        /// Pause audio. It can be resumed with [`play_audio`](World::play_world).
        pub fn pause_audio(&mut self, audio: AudioRef) {
            self.internal_event_queue.push(InternalEvent::PauseAudio(audio));
        }
        /// Stop audio. It CANNOT be resumed after this and the audio is consumed!
        pub fn stop_audio(&mut self, audio: AudioRef) {
            self.internal_event_queue.push(InternalEvent::StopAudio(audio));
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
            self.event_queue.push((target, Box::new(event_producer) as Box<_>))
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
            #[allow(non_exhaustive_omitted_patterns)]
            match self.input_state(action) {
                InputState::Pressed | InputState::NewlyPressed => true,
                _ => false,
            }
        }
        pub fn action_up(&self, action: ActionRef) -> bool {
            #[allow(non_exhaustive_omitted_patterns)]
            match self.input_state(action) {
                InputState::Released | InputState::NewlyReleased => true,
                _ => false,
            }
        }
        pub fn action_new_down(&self, action: ActionRef) -> bool {
            #[allow(non_exhaustive_omitted_patterns)]
            match self.input_state(action) {
                InputState::NewlyPressed => true,
                _ => false,
            }
        }
        pub fn action_new_up(&self, action: ActionRef) -> bool {
            #[allow(non_exhaustive_omitted_patterns)]
            match self.input_state(action) {
                InputState::NewlyReleased => true,
                _ => false,
            }
        }
    }
    pub struct Room {
        /// The sprite and offset of the background.
        pub background: Option<(SpriteRef, Vec2)>,
        pub objects: Vec<ObjectRef>,
        pub callbacks: Option<Callbacks>,
        pub state: ObjectState,
        /// Entry points where the player will spawn when entering this room.
        pub entrypoints: HashMap<EntryRef, Vec2>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Room {
        #[inline]
        fn clone(&self) -> Room {
            Room {
                background: ::core::clone::Clone::clone(&self.background),
                objects: ::core::clone::Clone::clone(&self.objects),
                callbacks: ::core::clone::Clone::clone(&self.callbacks),
                state: ::core::clone::Clone::clone(&self.state),
                entrypoints: ::core::clone::Clone::clone(&self.entrypoints),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Room {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Room",
                "background",
                &self.background,
                "objects",
                &self.objects,
                "callbacks",
                &self.callbacks,
                "state",
                &self.state,
                "entrypoints",
                &&self.entrypoints,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Room {
        #[inline]
        fn default() -> Room {
            Room {
                background: ::core::default::Default::default(),
                objects: ::core::default::Default::default(),
                callbacks: ::core::default::Default::default(),
                state: ::core::default::Default::default(),
                entrypoints: ::core::default::Default::default(),
            }
        }
    }
    pub(crate) struct SealedRoomTransition;
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for SealedRoomTransition {}
    #[automatically_derived]
    impl ::core::clone::Clone for SealedRoomTransition {
        #[inline]
        fn clone(&self) -> SealedRoomTransition {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for SealedRoomTransition {}
    #[automatically_derived]
    impl ::core::fmt::Debug for SealedRoomTransition {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "SealedRoomTransition")
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SealedRoomTransition {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SealedRoomTransition {
        #[inline]
        fn eq(&self, other: &SealedRoomTransition) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for SealedRoomTransition {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for SealedRoomTransition {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {}
    }
    #[automatically_derived]
    impl ::core::default::Default for SealedRoomTransition {
        #[inline]
        fn default() -> SealedRoomTransition {
            SealedRoomTransition {}
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for SealedRoomTransition {
        #[inline]
        fn partial_cmp(
            &self,
            other: &SealedRoomTransition,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for SealedRoomTransition {
        #[inline]
        fn cmp(&self, other: &SealedRoomTransition) -> ::core::cmp::Ordering {
            ::core::cmp::Ordering::Equal
        }
    }
    pub enum EventName {
        /// Does NOT continue if DisableDefault.
        AniContinueEvent,
        /// Called ~20 times a second. Duration is the delta time. DisableDefault is
        /// ignored. Called before graphics tick.
        Tick,
        /// Called at most 20 times a second; Default action will attempt to push it
        /// out to where it was at the previous frame before it started colliding.
        Collide,
        /// Called the full 60 times a second, after Tick when Tick is run. Duration is delta time,
        /// DisableDefault disables rendering the current sprite/animation like usual.
        Render,
        /// DisableDefault has no effect. Called after all resources are loaded and
        /// initalized, but before this is presented to the player on-screen.
        Load,
        /// Called before this object/room/world is unloaded. On the World, functions
        /// as a callback for before the game closes or is otherwise unloaded by the
        /// engine.
        Unload,
        SaveData,
        /// After all data is loaded into the ObjectState, called before Load to adjust
        /// the data or do any game-specific setup needed after loading.
        LoadData,
        KeyPress,
        KeyRelease,
        KeyHold,
        JoystickMove,
        MousePress,
        /// Produced by the engine for the room_transition object, if one exists. Emitted
        /// directly before unloading the old room and loading the new room (i.e. on the same frame).
        RoomTransition,
    }
    #[allow(deprecated)]
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl derive_more::core::fmt::Display for EventName {
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            match self {
                Self::AniContinueEvent => __derive_more_f.write_str("AniContinueEvent"),
                Self::Tick => __derive_more_f.write_str("Tick"),
                Self::Collide => __derive_more_f.write_str("Collide"),
                Self::Render => __derive_more_f.write_str("Render"),
                Self::Load => __derive_more_f.write_str("Load"),
                Self::Unload => __derive_more_f.write_str("Unload"),
                Self::SaveData => __derive_more_f.write_str("SaveData"),
                Self::LoadData => __derive_more_f.write_str("LoadData"),
                Self::KeyPress => __derive_more_f.write_str("KeyPress"),
                Self::KeyRelease => __derive_more_f.write_str("KeyRelease"),
                Self::KeyHold => __derive_more_f.write_str("KeyHold"),
                Self::JoystickMove => __derive_more_f.write_str("JoystickMove"),
                Self::MousePress => __derive_more_f.write_str("MousePress"),
                Self::RoomTransition => __derive_more_f.write_str("RoomTransition"),
            }
        }
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for EventName {}
    #[automatically_derived]
    impl ::core::clone::Clone for EventName {
        #[inline]
        fn clone(&self) -> EventName {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for EventName {}
    #[automatically_derived]
    impl ::core::fmt::Debug for EventName {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    EventName::AniContinueEvent => "AniContinueEvent",
                    EventName::Tick => "Tick",
                    EventName::Collide => "Collide",
                    EventName::Render => "Render",
                    EventName::Load => "Load",
                    EventName::Unload => "Unload",
                    EventName::SaveData => "SaveData",
                    EventName::LoadData => "LoadData",
                    EventName::KeyPress => "KeyPress",
                    EventName::KeyRelease => "KeyRelease",
                    EventName::KeyHold => "KeyHold",
                    EventName::JoystickMove => "JoystickMove",
                    EventName::MousePress => "MousePress",
                    EventName::RoomTransition => "RoomTransition",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EventName {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EventName {
        #[inline]
        fn eq(&self, other: &EventName) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for EventName {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for EventName {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    pub enum Event<'a> {
        /// Does NOT continue if DisableDefault.
        AniContinueEvent,
        /// Called ~20 times a second. Duration is the delta time. DisableDefault is
        /// ignored. Called before graphics tick.
        Tick(Duration),
        /// Called at most 20 times a second; Default action will attempt to push it
        /// out to where it was at the previous frame before it started colliding.
        Collide { other: ObjectRef },
        /// Called the full 60 times a second, after Tick when Tick is run. Duration is delta time,
        /// DisableDefault disables rendering the current sprite/animation like usual.
        Render(Duration, #[debug(skip)] Arc<Mutex<crate::rt::DrawContext>>),
        /// DisableDefault has no effect. Called after all resources are loaded and
        /// initalized, but before this is presented to the player on-screen.
        Load,
        /// Called before this object/room/world is unloaded. On the World, functions
        /// as a callback for before the game closes or is otherwise unloaded by the
        /// engine.
        Unload,
        SaveData {
            /// A new object state NOT tied to the current object to write new data that
            /// has to be saved to or otherwise change the saved data.
            new_obj_state: &'a mut ObjectState,
        },
        /// After all data is loaded into the ObjectState, called before Load to adjust
        /// the data or do any game-specific setup needed after loading.
        LoadData,
        KeyPress { key: Key },
        KeyRelease { key: Key },
        KeyHold { key: Key },
        JoystickMove {
            axis: gilrs::Axis,
            /// In the range 0..=1
            value: f32,
        },
        MousePress {
            button: macroquad::input::MouseButton,
            /// World position of the mouse press
            pos: Vec2,
        },
        /// Produced by the engine for the room_transition object, if one exists. Emitted
        /// directly before unloading the old room and loading the new room (i.e. on the same frame).
        RoomTransition {
            /// To prevent manually constructing outside of the engine
            #[allow(private_interfaces)]
            _sealed: SealedRoomTransition,
            from: RoomRef,
            to: RoomRef,
            entry: EntryRef,
        },
    }
    #[allow(deprecated)]
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl<'a> derive_more::core::fmt::Debug for Event<'a> {
        #[inline]
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            match self {
                Self::AniContinueEvent => {
                    derive_more::core::fmt::Formatter::write_str(
                        __derive_more_f,
                        "AniContinueEvent",
                    )
                }
                Self::Tick(_0) => {
                    derive_more::__private::DebugTuple::finish(
                        derive_more::__private::DebugTuple::field(
                            &mut derive_more::__private::debug_tuple(
                                __derive_more_f,
                                "Tick",
                            ),
                            &_0,
                        ),
                    )
                }
                Self::Collide { other } => {
                    derive_more::core::fmt::DebugStruct::finish(
                        derive_more::core::fmt::DebugStruct::field(
                            &mut derive_more::core::fmt::Formatter::debug_struct(
                                __derive_more_f,
                                "Collide",
                            ),
                            "other",
                            &other,
                        ),
                    )
                }
                Self::Render(_0, _1) => {
                    derive_more::__private::DebugTuple::finish_non_exhaustive(
                        derive_more::__private::DebugTuple::field(
                            &mut derive_more::__private::debug_tuple(
                                __derive_more_f,
                                "Render",
                            ),
                            &_0,
                        ),
                    )
                }
                Self::Load => {
                    derive_more::core::fmt::Formatter::write_str(__derive_more_f, "Load")
                }
                Self::Unload => {
                    derive_more::core::fmt::Formatter::write_str(
                        __derive_more_f,
                        "Unload",
                    )
                }
                Self::SaveData { new_obj_state } => {
                    derive_more::core::fmt::DebugStruct::finish(
                        derive_more::core::fmt::DebugStruct::field(
                            &mut derive_more::core::fmt::Formatter::debug_struct(
                                __derive_more_f,
                                "SaveData",
                            ),
                            "new_obj_state",
                            &new_obj_state,
                        ),
                    )
                }
                Self::LoadData => {
                    derive_more::core::fmt::Formatter::write_str(
                        __derive_more_f,
                        "LoadData",
                    )
                }
                Self::KeyPress { key } => {
                    derive_more::core::fmt::DebugStruct::finish(
                        derive_more::core::fmt::DebugStruct::field(
                            &mut derive_more::core::fmt::Formatter::debug_struct(
                                __derive_more_f,
                                "KeyPress",
                            ),
                            "key",
                            &key,
                        ),
                    )
                }
                Self::KeyRelease { key } => {
                    derive_more::core::fmt::DebugStruct::finish(
                        derive_more::core::fmt::DebugStruct::field(
                            &mut derive_more::core::fmt::Formatter::debug_struct(
                                __derive_more_f,
                                "KeyRelease",
                            ),
                            "key",
                            &key,
                        ),
                    )
                }
                Self::KeyHold { key } => {
                    derive_more::core::fmt::DebugStruct::finish(
                        derive_more::core::fmt::DebugStruct::field(
                            &mut derive_more::core::fmt::Formatter::debug_struct(
                                __derive_more_f,
                                "KeyHold",
                            ),
                            "key",
                            &key,
                        ),
                    )
                }
                Self::JoystickMove { axis, value } => {
                    derive_more::core::fmt::DebugStruct::finish(
                        derive_more::core::fmt::DebugStruct::field(
                            derive_more::core::fmt::DebugStruct::field(
                                &mut derive_more::core::fmt::Formatter::debug_struct(
                                    __derive_more_f,
                                    "JoystickMove",
                                ),
                                "axis",
                                &axis,
                            ),
                            "value",
                            &value,
                        ),
                    )
                }
                Self::MousePress { button, pos } => {
                    derive_more::core::fmt::DebugStruct::finish(
                        derive_more::core::fmt::DebugStruct::field(
                            derive_more::core::fmt::DebugStruct::field(
                                &mut derive_more::core::fmt::Formatter::debug_struct(
                                    __derive_more_f,
                                    "MousePress",
                                ),
                                "button",
                                &button,
                            ),
                            "pos",
                            &pos,
                        ),
                    )
                }
                Self::RoomTransition { _sealed, from, to, entry } => {
                    derive_more::core::fmt::DebugStruct::finish(
                        derive_more::core::fmt::DebugStruct::field(
                            derive_more::core::fmt::DebugStruct::field(
                                derive_more::core::fmt::DebugStruct::field(
                                    derive_more::core::fmt::DebugStruct::field(
                                        &mut derive_more::core::fmt::Formatter::debug_struct(
                                            __derive_more_f,
                                            "RoomTransition",
                                        ),
                                        "_sealed",
                                        &_sealed,
                                    ),
                                    "from",
                                    &from,
                                ),
                                "to",
                                &to,
                            ),
                            "entry",
                            &entry,
                        ),
                    )
                }
            }
        }
    }
    impl From<&Event<'_>> for EventName {
        fn from(value: &Event<'_>) -> Self {
            match value {
                Event::AniContinueEvent => EventName::AniContinueEvent,
                Event::Tick(..) => EventName::Tick,
                Event::Collide { .. } => EventName::Collide,
                Event::Render(..) => EventName::Render,
                Event::Load => EventName::Load,
                Event::Unload => EventName::Unload,
                Event::SaveData { .. } => EventName::SaveData,
                Event::LoadData => EventName::LoadData,
                Event::KeyPress { .. } => EventName::KeyPress,
                Event::KeyRelease { .. } => EventName::KeyRelease,
                Event::KeyHold { .. } => EventName::KeyHold,
                Event::JoystickMove { .. } => EventName::JoystickMove,
                Event::MousePress { .. } => EventName::MousePress,
                Event::RoomTransition { .. } => EventName::RoomTransition,
            }
        }
    }
    pub struct EventArgs<'a> {
        pub room: Option<RoomRef>,
        pub obj: Option<ObjectRef>,
        pub world: &'a mut World,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for EventArgs<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "EventArgs",
                "room",
                &self.room,
                "obj",
                &self.obj,
                "world",
                &&self.world,
            )
        }
    }
    #[must_use = "An event result should not be ignored"]
    pub enum EventResult {
        Default,
        DisableDefault,
        Result(StateData),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for EventResult {
        #[inline]
        fn clone(&self) -> EventResult {
            match self {
                EventResult::Default => EventResult::Default,
                EventResult::DisableDefault => EventResult::DisableDefault,
                EventResult::Result(__self_0) => {
                    EventResult::Result(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for EventResult {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                EventResult::Default => ::core::fmt::Formatter::write_str(f, "Default"),
                EventResult::DisableDefault => {
                    ::core::fmt::Formatter::write_str(f, "DisableDefault")
                }
                EventResult::Result(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Result",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for EventResult {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for EventResult {
        #[inline]
        fn eq(&self, other: &EventResult) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (EventResult::Result(__self_0), EventResult::Result(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for EventResult {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state);
            match self {
                EventResult::Result(__self_0) => {
                    ::core::hash::Hash::hash(__self_0, state)
                }
                _ => {}
            }
        }
    }
    pub struct Callbacks(
        HashMap<EventName, Arc<dyn Fn(Event, EventArgs) -> EventResult>>,
    );
    #[automatically_derived]
    impl ::core::clone::Clone for Callbacks {
        #[inline]
        fn clone(&self) -> Callbacks {
            Callbacks(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Callbacks {
        #[inline]
        fn default() -> Callbacks {
            Callbacks(::core::default::Default::default())
        }
    }
    impl Debug for Callbacks {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("Callbacks").field(&self.0.keys()).finish_non_exhaustive()
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
        pub(crate) fn trigger(
            &self,
            event: Event,
            args: EventArgs,
        ) -> Option<EventResult> {
            self.0.get(&(&event).into()).map(|v| v(event, args))
        }
    }
    pub struct Object {
        pub collider: Vec<Collider>,
        /// If true, will never be moved by the engine (but may be by the game).
        pub static_body: bool,
        pub sheet: Option<AniSheetRef>,
        pub state: ObjectState,
        pub callbacks: Option<Callbacks>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Object {
        #[inline]
        fn clone(&self) -> Object {
            Object {
                collider: ::core::clone::Clone::clone(&self.collider),
                static_body: ::core::clone::Clone::clone(&self.static_body),
                sheet: ::core::clone::Clone::clone(&self.sheet),
                state: ::core::clone::Clone::clone(&self.state),
                callbacks: ::core::clone::Clone::clone(&self.callbacks),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Object {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "Object",
                "collider",
                &self.collider,
                "static_body",
                &self.static_body,
                "sheet",
                &self.sheet,
                "state",
                &self.state,
                "callbacks",
                &&self.callbacks,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Object {
        #[inline]
        fn default() -> Object {
            Object {
                collider: ::core::default::Default::default(),
                static_body: ::core::default::Default::default(),
                sheet: ::core::default::Default::default(),
                state: ::core::default::Default::default(),
                callbacks: ::core::default::Default::default(),
            }
        }
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
    pub struct AniSheet {
        pub anis: HashMap<String, AnimationRef>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AniSheet {
        #[inline]
        fn clone(&self) -> AniSheet {
            AniSheet {
                anis: ::core::clone::Clone::clone(&self.anis),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AniSheet {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "AniSheet",
                "anis",
                &&self.anis,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AniSheet {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AniSheet {
        #[inline]
        fn eq(&self, other: &AniSheet) -> bool {
            self.anis == other.anis
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for AniSheet {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<HashMap<String, AnimationRef>>;
        }
    }
    pub enum AniEvent {
        Sprite { sprite: SpriteRef, frame_count: NonZeroU8 },
        /// Pause and wait for a AniContinueEvent to be posted to this Object.
        PausePoint,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AniEvent {
        #[inline]
        fn clone(&self) -> AniEvent {
            match self {
                AniEvent::Sprite { sprite: __self_0, frame_count: __self_1 } => {
                    AniEvent::Sprite {
                        sprite: ::core::clone::Clone::clone(__self_0),
                        frame_count: ::core::clone::Clone::clone(__self_1),
                    }
                }
                AniEvent::PausePoint => AniEvent::PausePoint,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AniEvent {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                AniEvent::Sprite { sprite: __self_0, frame_count: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Sprite",
                        "sprite",
                        __self_0,
                        "frame_count",
                        &__self_1,
                    )
                }
                AniEvent::PausePoint => {
                    ::core::fmt::Formatter::write_str(f, "PausePoint")
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AniEvent {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AniEvent {
        #[inline]
        fn eq(&self, other: &AniEvent) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (
                        AniEvent::Sprite { sprite: __self_0, frame_count: __self_1 },
                        AniEvent::Sprite { sprite: __arg1_0, frame_count: __arg1_1 },
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    _ => true,
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for AniEvent {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<SpriteRef>;
            let _: ::core::cmp::AssertParamIsEq<NonZeroU8>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for AniEvent {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state);
            match self {
                AniEvent::Sprite { sprite: __self_0, frame_count: __self_1 } => {
                    ::core::hash::Hash::hash(__self_0, state);
                    ::core::hash::Hash::hash(__self_1, state)
                }
                _ => {}
            }
        }
    }
    pub struct Animation {
        pub timeline: Vec<AniEvent>,
        pub fps: u8,
        pub loops: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Animation {
        #[inline]
        fn clone(&self) -> Animation {
            Animation {
                timeline: ::core::clone::Clone::clone(&self.timeline),
                fps: ::core::clone::Clone::clone(&self.fps),
                loops: ::core::clone::Clone::clone(&self.loops),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Animation {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Animation",
                "timeline",
                &self.timeline,
                "fps",
                &self.fps,
                "loops",
                &&self.loops,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Animation {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Animation {
        #[inline]
        fn eq(&self, other: &Animation) -> bool {
            self.fps == other.fps && self.loops == other.loops
                && self.timeline == other.timeline
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Animation {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<Vec<AniEvent>>;
            let _: ::core::cmp::AssertParamIsEq<u8>;
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Animation {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
            ::core::hash::Hash::hash(&self.timeline, state);
            ::core::hash::Hash::hash(&self.fps, state);
            ::core::hash::Hash::hash(&self.loops, state)
        }
    }
    #[repr(C)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }
    #[automatically_derived]
    #[doc(hidden)]
    unsafe impl ::core::clone::TrivialClone for Color {}
    #[automatically_derived]
    impl ::core::clone::Clone for Color {
        #[inline]
        fn clone(&self) -> Color {
            let _: ::core::clone::AssertParamIsClone<u8>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Color {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Color {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Color",
                "r",
                &self.r,
                "g",
                &self.g,
                "b",
                &self.b,
                "a",
                &&self.a,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Color {
        #[inline]
        fn default() -> Color {
            Color {
                r: ::core::default::Default::default(),
                g: ::core::default::Default::default(),
                b: ::core::default::Default::default(),
                a: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Color {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Color {
        #[inline]
        fn eq(&self, other: &Color) -> bool {
            self.r == other.r && self.g == other.g && self.b == other.b
                && self.a == other.a
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Color {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<u8>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Color {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
            ::core::hash::Hash::hash(&self.r, state);
            ::core::hash::Hash::hash(&self.g, state);
            ::core::hash::Hash::hash(&self.b, state);
            ::core::hash::Hash::hash(&self.a, state)
        }
    }
    #[automatically_derived]
    ///An archived [`Color`]
    #[bytecheck(crate = ::rkyv::bytecheck)]
    #[repr(C)]
    pub struct ArchivedColor
    where
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
    {
        ///The archived counterpart of [`Color::r`]
        pub r: <u8 as ::rkyv::Archive>::Archived,
        ///The archived counterpart of [`Color::g`]
        pub g: <u8 as ::rkyv::Archive>::Archived,
        ///The archived counterpart of [`Color::b`]
        pub b: <u8 as ::rkyv::Archive>::Archived,
        ///The archived counterpart of [`Color::a`]
        pub a: <u8 as ::rkyv::Archive>::Archived,
    }
    #[automatically_derived]
    unsafe impl<
        __C: ::rkyv::bytecheck::rancor::Fallible + ?::core::marker::Sized,
    > ::rkyv::bytecheck::CheckBytes<__C> for ArchivedColor
    where
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        <__C as ::rkyv::bytecheck::rancor::Fallible>::Error: ::rkyv::bytecheck::rancor::Trace,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::bytecheck::CheckBytes<__C>,
    {
        unsafe fn check_bytes(
            value: *const Self,
            context: &mut __C,
        ) -> ::core::result::Result<
            (),
            <__C as ::rkyv::bytecheck::rancor::Fallible>::Error,
        > {
            <<u8 as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                __C,
            >>::check_bytes(&raw const (*value).r, context)
                .map_err(|e| {
                    <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                        e,
                        ::rkyv::bytecheck::StructCheckContext {
                            struct_name: "ArchivedColor",
                            field_name: "r",
                        },
                    )
                })?;
            <<u8 as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                __C,
            >>::check_bytes(&raw const (*value).g, context)
                .map_err(|e| {
                    <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                        e,
                        ::rkyv::bytecheck::StructCheckContext {
                            struct_name: "ArchivedColor",
                            field_name: "g",
                        },
                    )
                })?;
            <<u8 as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                __C,
            >>::check_bytes(&raw const (*value).b, context)
                .map_err(|e| {
                    <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                        e,
                        ::rkyv::bytecheck::StructCheckContext {
                            struct_name: "ArchivedColor",
                            field_name: "b",
                        },
                    )
                })?;
            <<u8 as ::rkyv::Archive>::Archived as ::rkyv::bytecheck::CheckBytes<
                __C,
            >>::check_bytes(&raw const (*value).a, context)
                .map_err(|e| {
                    <<__C as ::rkyv::bytecheck::rancor::Fallible>::Error as ::rkyv::bytecheck::rancor::Trace>::trace(
                        e,
                        ::rkyv::bytecheck::StructCheckContext {
                            struct_name: "ArchivedColor",
                            field_name: "a",
                        },
                    )
                })?;
            ::core::result::Result::Ok(())
        }
    }
    #[automatically_derived]
    ///The resolver for an archived [`Color`]
    pub struct ColorResolver
    where
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
    {
        r: <u8 as ::rkyv::Archive>::Resolver,
        g: <u8 as ::rkyv::Archive>::Resolver,
        b: <u8 as ::rkyv::Archive>::Resolver,
        a: <u8 as ::rkyv::Archive>::Resolver,
    }
    impl ::rkyv::Archive for Color
    where
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
    {
        type Archived = ArchivedColor;
        type Resolver = ColorResolver;
        const COPY_OPTIMIZATION: ::rkyv::traits::CopyOptimization<Self> = unsafe {
            ::rkyv::traits::CopyOptimization::enable_if(
                0 + ::core::mem::size_of::<u8>() + ::core::mem::size_of::<u8>()
                    + ::core::mem::size_of::<u8>() + ::core::mem::size_of::<u8>()
                    == ::core::mem::size_of::<Color>()
                    && <u8 as ::rkyv::Archive>::COPY_OPTIMIZATION.is_enabled()
                    && const { builtin # offset_of(Color, r) }
                        == const { builtin # offset_of(ArchivedColor, r) }
                    && <u8 as ::rkyv::Archive>::COPY_OPTIMIZATION.is_enabled()
                    && const { builtin # offset_of(Color, g) }
                        == const { builtin # offset_of(ArchivedColor, g) }
                    && <u8 as ::rkyv::Archive>::COPY_OPTIMIZATION.is_enabled()
                    && const { builtin # offset_of(Color, b) }
                        == const { builtin # offset_of(ArchivedColor, b) }
                    && <u8 as ::rkyv::Archive>::COPY_OPTIMIZATION.is_enabled()
                    && const { builtin # offset_of(Color, a) }
                        == const { builtin # offset_of(ArchivedColor, a) },
            )
        };
        #[allow(clippy::unit_arg)]
        fn resolve(&self, resolver: Self::Resolver, out: ::rkyv::Place<Self::Archived>) {
            let field_ptr = unsafe { &raw mut (*out.ptr()).r };
            let field_out = unsafe {
                ::rkyv::Place::from_field_unchecked(out, field_ptr)
            };
            <u8 as ::rkyv::Archive>::resolve(&self.r, resolver.r, field_out);
            let field_ptr = unsafe { &raw mut (*out.ptr()).g };
            let field_out = unsafe {
                ::rkyv::Place::from_field_unchecked(out, field_ptr)
            };
            <u8 as ::rkyv::Archive>::resolve(&self.g, resolver.g, field_out);
            let field_ptr = unsafe { &raw mut (*out.ptr()).b };
            let field_out = unsafe {
                ::rkyv::Place::from_field_unchecked(out, field_ptr)
            };
            <u8 as ::rkyv::Archive>::resolve(&self.b, resolver.b, field_out);
            let field_ptr = unsafe { &raw mut (*out.ptr()).a };
            let field_out = unsafe {
                ::rkyv::Place::from_field_unchecked(out, field_ptr)
            };
            <u8 as ::rkyv::Archive>::resolve(&self.a, resolver.a, field_out);
        }
    }
    unsafe impl ::rkyv::traits::Portable for ArchivedColor
    where
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        u8: ::rkyv::Archive,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::traits::Portable,
    {}
    #[automatically_derived]
    impl<__D: ::rkyv::rancor::Fallible + ?Sized> ::rkyv::Deserialize<Color, __D>
    for ::rkyv::Archived<Color>
    where
        u8: ::rkyv::Archive,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<u8, __D>,
        u8: ::rkyv::Archive,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<u8, __D>,
        u8: ::rkyv::Archive,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<u8, __D>,
        u8: ::rkyv::Archive,
        <u8 as ::rkyv::Archive>::Archived: ::rkyv::Deserialize<u8, __D>,
    {
        fn deserialize(
            &self,
            deserializer: &mut __D,
        ) -> ::core::result::Result<Color, <__D as ::rkyv::rancor::Fallible>::Error> {
            let __this = self;
            ::core::result::Result::Ok(Color {
                r: <<u8 as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                    u8,
                    __D,
                >>::deserialize(&__this.r, deserializer)?,
                g: <<u8 as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                    u8,
                    __D,
                >>::deserialize(&__this.g, deserializer)?,
                b: <<u8 as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                    u8,
                    __D,
                >>::deserialize(&__this.b, deserializer)?,
                a: <<u8 as ::rkyv::Archive>::Archived as ::rkyv::Deserialize<
                    u8,
                    __D,
                >>::deserialize(&__this.a, deserializer)?,
            })
        }
    }
    #[automatically_derived]
    impl<__S: ::rkyv::rancor::Fallible + ?Sized> ::rkyv::Serialize<__S> for Color
    where
        u8: ::rkyv::Serialize<__S>,
        u8: ::rkyv::Serialize<__S>,
        u8: ::rkyv::Serialize<__S>,
        u8: ::rkyv::Serialize<__S>,
    {
        fn serialize(
            &self,
            serializer: &mut __S,
        ) -> ::core::result::Result<
            <Self as ::rkyv::Archive>::Resolver,
            <__S as ::rkyv::rancor::Fallible>::Error,
        > {
            let __this = self;
            ::core::result::Result::Ok(ColorResolver {
                r: <u8 as ::rkyv::Serialize<__S>>::serialize(&__this.r, serializer)?,
                g: <u8 as ::rkyv::Serialize<__S>>::serialize(&__this.g, serializer)?,
                b: <u8 as ::rkyv::Serialize<__S>>::serialize(&__this.b, serializer)?,
                a: <u8 as ::rkyv::Serialize<__S>>::serialize(&__this.a, serializer)?,
            })
        }
    }
    impl Color {
        pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
        pub fn multiply_alpha(self, bg: Color) -> Color {
            if self.a == u8::MAX {
                return self;
            }
            let inv_a = 255 - self.a;
            Color {
                r: ((self.r as u16 * self.a as u16 + bg.r as u16 * inv_a as u16) / 255)
                    as u8,
                g: ((self.g as u16 * self.a as u16 + bg.g as u16 * inv_a as u16) / 255)
                    as u8,
                b: ((self.b as u16 * self.a as u16 + bg.b as u16 * inv_a as u16) / 255)
                    as u8,
                a: 255,
            }
        }
    }
    impl std::ops::Add<Color> for Color {
        type Output = Color;
        fn add(self, rhs: Color) -> Color {
            Color {
                r: self.r + rhs.r,
                g: self.g + rhs.g,
                b: self.b + rhs.b,
                a: self.a + rhs.a,
            }
        }
    }
    impl std::ops::AddAssign<Color> for Color {
        fn add_assign(&mut self, rhs: Color) {
            *self = Color {
                r: self.r + rhs.r,
                g: self.g + rhs.g,
                b: self.b + rhs.b,
                a: self.a + rhs.a,
            };
        }
    }
    impl std::ops::Sub<Color> for Color {
        type Output = Color;
        fn sub(self, rhs: Color) -> Color {
            Color {
                r: self.r - rhs.r,
                g: self.g - rhs.g,
                b: self.b - rhs.b,
                a: self.a - rhs.a,
            }
        }
    }
    impl std::ops::SubAssign<Color> for Color {
        fn sub_assign(&mut self, rhs: Color) {
            *self = Color {
                r: self.r - rhs.r,
                g: self.g - rhs.g,
                b: self.b - rhs.b,
                a: self.a - rhs.a,
            };
        }
    }
    impl std::ops::Mul<u8> for Color {
        type Output = Color;
        fn mul(self, rhs: u8) -> Color {
            Color {
                r: (self.r as u8 * rhs) as u8,
                g: (self.g as u8 * rhs) as u8,
                b: (self.b as u8 * rhs) as u8,
                a: (self.a as u8 * rhs) as u8,
            }
        }
    }
    impl std::ops::MulAssign<u8> for Color {
        fn mul_assign(&mut self, rhs: u8) {
            *self = Color {
                r: (self.r as u8 * rhs) as u8,
                g: (self.g as u8 * rhs) as u8,
                b: (self.b as u8 * rhs) as u8,
                a: (self.a as u8 * rhs) as u8,
            };
        }
    }
    impl std::ops::Div<u8> for Color {
        type Output = Color;
        fn div(self, rhs: u8) -> Color {
            Color {
                r: (self.r as u8 / rhs) as u8,
                g: (self.g as u8 / rhs) as u8,
                b: (self.b as u8 / rhs) as u8,
                a: (self.a as u8 / rhs) as u8,
            }
        }
    }
    impl std::ops::DivAssign<u8> for Color {
        fn div_assign(&mut self, rhs: u8) {
            *self = Color {
                r: (self.r as u8 / rhs) as u8,
                g: (self.g as u8 / rhs) as u8,
                b: (self.b as u8 / rhs) as u8,
                a: (self.a as u8 / rhs) as u8,
            };
        }
    }
    impl std::ops::Mul<u16> for Color {
        type Output = Color;
        fn mul(self, rhs: u16) -> Color {
            Color {
                r: (self.r as u16 * rhs) as u8,
                g: (self.g as u16 * rhs) as u8,
                b: (self.b as u16 * rhs) as u8,
                a: (self.a as u16 * rhs) as u8,
            }
        }
    }
    impl std::ops::MulAssign<u16> for Color {
        fn mul_assign(&mut self, rhs: u16) {
            *self = Color {
                r: (self.r as u16 * rhs) as u8,
                g: (self.g as u16 * rhs) as u8,
                b: (self.b as u16 * rhs) as u8,
                a: (self.a as u16 * rhs) as u8,
            };
        }
    }
    impl std::ops::Div<u16> for Color {
        type Output = Color;
        fn div(self, rhs: u16) -> Color {
            Color {
                r: (self.r as u16 / rhs) as u8,
                g: (self.g as u16 / rhs) as u8,
                b: (self.b as u16 / rhs) as u8,
                a: (self.a as u16 / rhs) as u8,
            }
        }
    }
    impl std::ops::DivAssign<u16> for Color {
        fn div_assign(&mut self, rhs: u16) {
            *self = Color {
                r: (self.r as u16 / rhs) as u8,
                g: (self.g as u16 / rhs) as u8,
                b: (self.b as u16 / rhs) as u8,
                a: (self.a as u16 / rhs) as u8,
            };
        }
    }
    pub struct Sprite {
        pub width: u16,
        pub height: u16,
        pub data: Vec<Color>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Sprite {
        #[inline]
        fn clone(&self) -> Sprite {
            Sprite {
                width: ::core::clone::Clone::clone(&self.width),
                height: ::core::clone::Clone::clone(&self.height),
                data: ::core::clone::Clone::clone(&self.data),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Sprite {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Sprite",
                "width",
                &self.width,
                "height",
                &self.height,
                "data",
                &&self.data,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Sprite {
        #[inline]
        fn default() -> Sprite {
            Sprite {
                width: ::core::default::Default::default(),
                height: ::core::default::Default::default(),
                data: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Sprite {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Sprite {
        #[inline]
        fn eq(&self, other: &Sprite) -> bool {
            self.width == other.width && self.height == other.height
                && self.data == other.data
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Sprite {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<u16>;
            let _: ::core::cmp::AssertParamIsEq<Vec<Color>>;
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Sprite {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) {
            ::core::hash::Hash::hash(&self.width, state);
            ::core::hash::Hash::hash(&self.height, state);
            ::core::hash::Hash::hash(&self.data, state)
        }
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
    #[allow(deprecated)]
    #[allow(unreachable_code)]
    #[automatically_derived]
    impl derive_more::core::fmt::Debug for Audio {
        #[inline]
        fn fmt(
            &self,
            __derive_more_f: &mut derive_more::core::fmt::Formatter<'_>,
        ) -> derive_more::core::fmt::Result {
            let source = &self.source;
            let loc = &self.loc;
            let player = &self.player;
            derive_more::core::fmt::DebugStruct::finish_non_exhaustive(
                derive_more::core::fmt::DebugStruct::field(
                    derive_more::core::fmt::DebugStruct::field(
                        &mut derive_more::core::fmt::Formatter::debug_struct(
                            __derive_more_f,
                            "Audio",
                        ),
                        "source",
                        &format_args!("{{opaque source object}}"),
                    ),
                    "loc",
                    &format_args!("(x: {0}, y: {1}, z: {2})", loc.0, loc.1, loc.2),
                ),
            )
        }
    }
    pub struct Font {
        pub char_index_map: HashMap<char, usize>,
        /// u16 is the width of that character
        pub sprites: Vec<(SpriteRef, u16)>,
        pub line_height: u16,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Font {
        #[inline]
        fn clone(&self) -> Font {
            Font {
                char_index_map: ::core::clone::Clone::clone(&self.char_index_map),
                sprites: ::core::clone::Clone::clone(&self.sprites),
                line_height: ::core::clone::Clone::clone(&self.line_height),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Font {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Font",
                "char_index_map",
                &self.char_index_map,
                "sprites",
                &self.sprites,
                "line_height",
                &&self.line_height,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Font {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Font {
        #[inline]
        fn eq(&self, other: &Font) -> bool {
            self.line_height == other.line_height
                && self.char_index_map == other.char_index_map
                && self.sprites == other.sprites
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Font {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_fields_are_eq(&self) {
            let _: ::core::cmp::AssertParamIsEq<HashMap<char, usize>>;
            let _: ::core::cmp::AssertParamIsEq<Vec<(SpriteRef, u16)>>;
            let _: ::core::cmp::AssertParamIsEq<u16>;
        }
    }
    pub struct DisplayedText {
        pub contents: LocalTextRef,
        /// Screen-space, NOT world-space.
        pub loc: Vec2,
        /// Each character is rotated by this, not the entire text. Essentially poor
        /// mans italic.
        pub char_rot: f32,
        pub font: FontRef,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for DisplayedText {
        #[inline]
        fn clone(&self) -> DisplayedText {
            DisplayedText {
                contents: ::core::clone::Clone::clone(&self.contents),
                loc: ::core::clone::Clone::clone(&self.loc),
                char_rot: ::core::clone::Clone::clone(&self.char_rot),
                font: ::core::clone::Clone::clone(&self.font),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for DisplayedText {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "DisplayedText",
                "contents",
                &self.contents,
                "loc",
                &self.loc,
                "char_rot",
                &self.char_rot,
                "font",
                &&self.font,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for DisplayedText {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for DisplayedText {
        #[inline]
        fn eq(&self, other: &DisplayedText) -> bool {
            self.char_rot == other.char_rot && self.contents == other.contents
                && self.loc == other.loc && self.font == other.font
        }
    }
    pub struct LanguageData {
        pub strings: HashMap<LocalTextRef, String>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for LanguageData {
        #[inline]
        fn clone(&self) -> LanguageData {
            LanguageData {
                strings: ::core::clone::Clone::clone(&self.strings),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LanguageData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "LanguageData",
                "strings",
                &&self.strings,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for LanguageData {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for LanguageData {
        #[inline]
        fn eq(&self, other: &LanguageData) -> bool {
            self.strings == other.strings
        }
    }
}
