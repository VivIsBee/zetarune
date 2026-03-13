//! Resource providers

use image::{DynamicImage, GenericImage};
use libgm::wad::ParsingOptions;

use crate::{
    ctx::Ctx,
    objs::{self, Audio, Color, Offset2, Sprite, Vec2},
};
use std::{
    collections::{HashMap, HashSet}, convert::Infallible, error::Error, io::Cursor, mem::ManuallyDrop, path::PathBuf
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProviderInfo {
    pub name: String,
    pub desc: Option<String>,
}

pub trait Provider {
    type Error: Error;

    /// Loads all resources this provider offers into the given `Ctx`, prefixing
    /// IDs with the given namespace..
    fn load(&mut self, ctx: &mut Ctx, namespace: &str) -> Result<(), Self::Error>;
    /// Present any menu needed to load the assets.
    fn present(&mut self) -> Result<(), Self::Error>;
    fn info(&self) -> ProviderInfo;
}

pub struct UncompressedStaticAssetProvider {
    sprites: &'static [(&'static str, Sprite)],
}

impl UncompressedStaticAssetProvider {
    pub const fn new(sprites: &'static [(&'static str, Sprite)]) -> Self {
        Self { sprites }
    }
}

impl Provider for UncompressedStaticAssetProvider {
    type Error = Infallible;
    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "uncompressed static asset provider".to_string(),
            desc: Some(
                "loads uncompressed assets provided statically at compile time, counterpart to CompressedStaticAssetProvider"
                .to_string()
            ),
        }
    }
    fn present(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn load(&mut self, ctx: &mut Ctx, namespace: &str) -> Result<(), Self::Error> {
        for (id, sprite) in self.sprites {
            ctx.add_sprite(format!("{namespace}:{id}"), sprite.clone());
        }
        Ok(())
    }
}

pub struct CompressedStaticAssetProvider {
    sprites: &'static [(&'static str, &'static [u8])],
}

impl CompressedStaticAssetProvider {
    pub const fn new(sprites: &'static [(&'static str, &'static [u8])]) -> Self {
        Self { sprites }
    }
}

impl Provider for CompressedStaticAssetProvider {
    type Error = qoi::Error;
    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "compressed static asset provider".to_string(),
            desc: Some(
                "loads QOI compressed assets provided statically at compile time, counterpart to UncompressedStaticAssetProvider"
                .to_string()
            ),
        }
    }
    fn present(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn load(&mut self, ctx: &mut Ctx, namespace: &str) -> Result<(), Self::Error> {
        for (id, data) in self.sprites {
            let (header, bytes) = qoi::decode_to_vec(data)?;

            let sprite = Sprite {
                width: header.width as u16,
                height: header.height as u16,
                data: match header.channels {
                    qoi::Channels::Rgba => {
                        let mut data = ManuallyDrop::new(bytes);

                        let len = data.len();
                        let cap = data.capacity();
                        let ptr = data.as_mut_ptr();

                        unsafe { Vec::from_raw_parts(ptr as *mut objs::Color, len * 4, cap * 4) } /* SAFETY: Color is repr(C) */
                    }
                    qoi::Channels::Rgb => bytes
                        .chunks_exact(3)
                        .into_iter()
                        .map(|v| Color {
                            r: v[0],
                            g: v[1],
                            b: v[2],
                            a: 255,
                        })
                        .collect(),
                },
            };

            ctx.add_sprite(format!("{namespace}:{id}"), sprite);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub struct SpriteSheet {
    /// Source ID
    pub id: &'static str,
    /// Pairs of (location, size, ID).
    pub sections: &'static [(Vec2, Offset2, &'static str)],
}

pub struct SpriteSheetProvider {
    sheets: &'static [SpriteSheet],
}

impl SpriteSheetProvider {
    pub const fn new(sheets: &'static [SpriteSheet]) -> Self {
        Self { sheets }
    }
}

impl Provider for SpriteSheetProvider {
    type Error = Infallible;
    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "spritesheet provider".to_string(),
            desc: Some("cuts previously loaded textures into sections".to_string()),
        }
    }
    fn present(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
    fn load(&mut self, ctx: &mut Ctx, namespace: &str) -> Result<(), Self::Error> {
        for SpriteSheet { id, sections } in self.sheets {
            let sprite_r = ctx.get_sprite_id_ref(id).unwrap();

            for (loc, size, id) in *sections {
                let sprite = ctx.get_sprite(sprite_r);
                let out_sprite = sprite.slice(*loc, *size).unwrap();

                ctx.add_sprite(format!("{namespace}:{id}"), out_sprite);
            }
        }
        Ok(())
    }
}

pub struct GamemakerDataProvider {
    opts: ParsingOptions,
    data: Option<libgm::wad::GMData>,
    source_game: &'static str,
    sprites_to_load: HashMap<&'static str, HashSet<usize>>,
    audio_to_load: HashSet<&'static str>,
}

impl GamemakerDataProvider {
    pub const fn new(
        source_game: &'static str,
        sprites_to_load: HashMap<&'static str, HashSet<usize>>,
        audio_to_load: HashSet<&'static str>,
    ) -> Self {
        Self {
            opts: ParsingOptions::new().allow_unknown_chunks(false),
            data: None,
            source_game: source_game,
            sprites_to_load,
            audio_to_load,
        }
    }
    fn get_sprite_from_page_item(
        &self,
        page_item: &libgm::wad::elements::texture_page_item::GMTexturePageItem,
    ) -> Sprite {
        let tex = page_item
            .texture_page
            .resolve(&self.data.as_ref().unwrap().embedded_textures.texture_pages)
            .unwrap();
        let mut image = (*tex.image.as_ref().unwrap().to_dynamic_image().unwrap()).clone();
        image = image.crop_imm(
            page_item.source_x as u32,
            page_item.source_y as u32,
            page_item.source_width as u32,
            page_item.source_height as u32,
        );
        image = image.resize_exact(
            page_item.target_width as u32,
            page_item.target_height as u32,
            image::imageops::FilterType::Nearest,
        );

        let mut out_image = DynamicImage::new(
            page_item.bounding_width as u32,
            page_item.bounding_height as u32,
            image.color(),
        );

        out_image
            .copy_from(&image, page_item.target_x as u32, page_item.target_y as u32)
            .unwrap();

        let mut data = ManuallyDrop::new(out_image.to_rgba8().to_vec());

        let len = data.len();
        let cap = data.capacity();
        let ptr = data.as_mut_ptr();

        Sprite {
            width: out_image.width() as u16,
            height: out_image.height() as u16,
            data: unsafe { Vec::from_raw_parts(ptr as *mut objs::Color, len * 4, cap * 4) },
        }
    }
}

impl Provider for GamemakerDataProvider {
    type Error = libgm::Error;
    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "gamemaker data provider".to_string(),
            desc: Some("loads data from a gamemaker data file".to_string()),
        }
    }
    fn present(&mut self) -> Result<(), Self::Error> {
        let file = rfd::FileDialog::new()
            .add_filter("Gamemaker Data File", &["win", "unx", "droid", "ios"])
            .set_title(format!("Select {} Data File", self.source_game))
            .set_can_create_directories(false)
            .set_directory(
                std::env::current_exe()
                    .ok()
                    .map(|v| v.parent().unwrap().to_owned())
                    .or_else(|| std::env::current_dir().ok())
                    .unwrap_or(PathBuf::from(".")),
            )
            .pick_file();

        let file = file.expect("path to data file must be provided");

        let data = self.data.insert(self.opts.parse_file(file).unwrap());

        data.deserialize_textures().unwrap();
        data.validate_names().unwrap();

        Ok(())
    }
    fn load(&mut self, ctx: &mut Ctx, namespace: &str) -> Result<(), Self::Error> {
        let data = self.data.as_ref().unwrap();

        for sprite in &data.sprites.sprites {
            if !self.sprites_to_load.contains_key(sprite.name.as_str()) {
                continue;
            }
            let name = sprite.name.clone();

            let i_all = self.sprites_to_load.get(name.as_str()).unwrap();
            for i in i_all.iter().copied() {
                let tex = sprite.textures[i].unwrap();
                let page = tex
                    .resolve(&data.texture_page_items.texture_page_items)
                    .unwrap();
                let sprite = self.get_sprite_from_page_item(page);

                ctx.add_sprite(format!("{namespace}:{name}"), sprite);
            }
        }

        for sound in &data.sounds.sounds {
            if !self.audio_to_load.contains(sound.name.as_str()) {
                continue;
            }
            let name = sound.name.clone();

            let cursor = Cursor::new(if sound.flags.embedded {
                let audio = sound.audio_file.unwrap().resolve(&data.audios.audios).unwrap();

                audio.audio_data.clone()
            } else {
                let path = data.location.as_ref().unwrap().clone().join(&sound.file);

                std::fs::read(path).unwrap()
            });

            let source = Box::new(match sound.audio_type {
                libgm::wad::elements::sound::AudioType::Mp3 => rodio::Decoder::new_mp3(cursor).unwrap(),
                libgm::wad::elements::sound::AudioType::Unknown => rodio::Decoder::new(cursor).unwrap(),
                libgm::wad::elements::sound::AudioType::Wav => rodio::Decoder::new_wav(cursor).unwrap(),
                libgm::wad::elements::sound::AudioType::Ogg => rodio::Decoder::new_vorbis(cursor).unwrap(),
            });

            let audio = Audio::new(source, (0.0, 0.0, 0.0));

            ctx.add_audio(format!("{namespace}:{name}"), audio);
        }

        Ok(())
    }
}
