use core::{
    arch::asm, cell::OnceCell, fmt::Debug, marker::PhantomData, ops::Sub, sync::atomic::{AtomicBool, Ordering}, time::Duration
};

use alloc::sync::Arc;
use nx::{
    diag::{abort, log},
    gpu::{
        self, SCREEN_HEIGHT, SCREEN_WIDTH,
        canvas::{Canvas, RGBA8},
    },
    input,
    result::*,
    service::hid::{NpadButton, NpadStyleTag},
    svc,
    sync::{Mutex, RwLock},
    util,
};

use crate::{objs::World, rt, log, trace};

#[inline(always)]
extern "C" fn get_system_tick() -> u64 {
    let out: u64;
    unsafe {
        asm!(
            "mrs {}, cntpct_el0",
            out(reg) out
        );
    }
    out
}

/// Frequency in hertz.
#[inline(always)]
extern "C" fn get_system_tick_freq() -> u64 {
    let out: u64;
    unsafe {
        asm!(
            "mrs {}, cntfrq_el0",
            out(reg) out
        );
    }
    out
}

pub struct Instant {
    tick: u64,
}

impl Instant {
    pub fn now() -> Self {
        Self {
            tick: get_system_tick(),
        }
    }
    pub fn elapsed(self) -> Duration {
        Self::now() - self
    }
}

impl Debug for Instant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Instant").finish_non_exhaustive()
    }
}

impl Sub for Instant {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        let ticks_out = (self.tick - rhs.tick) as u128;
        let freq_hz = get_system_tick_freq() as u128;

        let nanos = (ticks_out * 1_000_000_000) / freq_hz;

        Duration::from_nanos_u128(nanos)
    }
}

/// A context needed to draw the screen. Technically could be bypassed, but
/// please don't :c
///
/// Everything is in world coordinates, except for specific scenarios like text.
pub struct DrawContext(Vec2, Offset2, Vec<Font>, Vec<Sprite>);

impl DrawContext {
    pub fn new(
        camera_pos: Vec2,
        screen_size: Offset2,
        fonts: Vec<Font>,
        sprites: Vec<Sprite>,
    ) -> Self {
        Self(camera_pos, screen_size, fonts, sprites)
    }
    pub fn draw_sprite(&mut self, sprite: Sprite, scale: Offset2, pos: Vec2, rot: f32) {
        self.draw_sprite_screen(sprite, scale, (pos - self.0).into(), rot);
    }
    pub fn draw_sprite_screen(&self, sprite: Sprite, scale: Offset2, top_left: Vec2, rot: f32) {
        let sprite = sprite.scale(scale);

        let bottom_right = top_left + sprite.get_size();

        if bottom_right < Vec2::ZERO || top_left > self.1.into() {
            return;
        }

        let center = top_left - (sprite.get_size() / 2.0);

        let mut data = ManuallyDrop::new(sprite.data);

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
        for ch in text.contents.chars() {
            match ch {
                '\r' => {}
                '\n' => {
                    pos.x = 0.0;
                    pos.y += font.line_height as f32;
                }
                '\t' => {
                    pos.x += (font.sprites[font.char_index_map[&' ']].1 * 4) as f32;
                }
                _ => {
                    let (sprite_r, x_off) = font.sprites[font.char_index_map[&ch]];
                    let sprite = self.3[sprite_r.index].clone();
                    self.draw_sprite_screen(sprite, text.scale, text.loc + pos, text.char_rot);

                    pos.x += x_off as f32;
                }
            }
        }
    }
}

pub fn main(window_title: impl ToString, resizable: bool, mut world: World) -> ! {
    log!(crate::log::Level::Info: false, "On switch. Using nx.");

    let supported_style_tags = NpadStyleTag::Handheld()
        | NpadStyleTag::FullKey()
        | NpadStyleTag::JoyDual()
        | NpadStyleTag::JoyLeft()
        | NpadStyleTag::JoyRight();

    let input = input::Context::new(supported_style_tags, 1).unwrap();
    let mut player = input.get_player(nx::service::hid::NpadIdType::No1);

    let gpu = Arc::new(RwLock::new(
        gpu::Context::new(
            gpu::NvDrvServiceKind::Applet,
            gpu::ViServiceKind::System,
            (SCREEN_WIDTH * SCREEN_HEIGHT * 4) as usize,
        )
        .unwrap(),
    ));

    let mut surface = gpu::canvas::CanvasManager::new_stray(
        gpu.clone(),
        gpu::surface::DisplayName::Default,
        2,
        gpu::BlockLinearHeights::FourGobs,
    )
    .unwrap();

    trace!("Done initalizing nx backend.");

    let mut timer = Duration::ZERO;

    loop {
        rt::frame(&mut timer, delta, &mut world, &mut surface);

        surface.wait_vsync_event(None).unwrap();
        // Sleep 10ms (aka 10'000'000 ns)
        // svc::sleep_thread(10_000_000).unwrap();
    }
}
