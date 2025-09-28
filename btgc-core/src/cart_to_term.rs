use alloc::vec::Vec;

use crate::AssetId;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum CompressionStyle {
    ImageAsset(ImageAssetComp),
    AudioAsset(AudioAssetComp),
    TextAsset(TextAssetComp),
    ColorPallet(ColorPalletteComp),
    GenericBinData(GenericBinDataComp),
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum ImageAssetComp {}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum AudioAssetComp {}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum TextAssetComp {}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum GenericBinDataComp {}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct ColorPalletteComp {}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum AssetType {
    Sf2,
    Tile,
    Midi,
    Sprite,
    Dialog,
    ColorPallette,
    Custom,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum InstructFrontEnd {
    SetTileSize {
        width: u32,
        height: u32,
    },
    SetSpriteSize {
        width: u32,
        height: u32,
    },
    LoadAsset {
        uid: AssetId,
        asset_type: AssetType,
        data: Vec<u8>,
    },
    DisplayTile {
        location: (u32, u32, u32),
        tile_id: AssetId,
    },
    DisplayTiles {
        tiles: Vec<((u32, u32, u32), AssetId)>,
    },
    DisplaySprite {
        location: (u32, u32, u32),
        tile_id: AssetId,
    },
    DisplaySprites {
        sprites: Vec<((u32, u32, u32), AssetId)>,
    },
    DespawnSprite {
        location: (u32, u32, u32),
        tile_id: AssetId,
    },
    MoveSprite {
        from: (u32, u32, u32),
        tile_id: AssetId,
        to: (u32, u32, u32),
    },
    LoadMap {
        map: Vec<((u32, u32, u32), AssetId)>,
    },
    DisplayLoadingScreen,
    SetScreenSize {
        /// number of tiles wide
        width: u32,
        /// number of tiles tall
        height: u32,
    },
    DisableKeyboardInput {
        id: Option<u32>,
    },
    DisableMouseInput {
        id: Option<u32>,
    },
    DisableControllerInput {
        id: Option<u32>,
    },
    SetResolution {
        /// number of PX wide
        width: u32,
        /// number of PX tall
        height: u32,
    },
}
