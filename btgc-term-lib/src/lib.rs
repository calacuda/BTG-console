use std::sync::Arc;

use btgc_core::{
    AssetId, Color, Message, MessageId, Result,
    cart_to_term::{Asset, InstructFrontEnd},
};
pub use rustc_hash::FxHashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessedMessageRes {
    Resend(MessageId),
    SendAck(MessageId),
    TermMustHandle(InstructFrontEnd),
    Success,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalAPI {
    color_pallet: Arc<[Option<Color>]>,
    assets: FxHashMap<AssetId, Asset>,
    screen_resolution: (u32, u32),
    tile_resolution: (u32, u32),
    sprite_resolution: (u32, u32),
    tile_geometry: (u32, u32),
}

impl Default for TerminalAPI {
    fn default() -> Self {
        Self {
            color_pallet: [].into(),
            assets: FxHashMap::default(),
            screen_resolution: (0u32, 0u32),
            tile_resolution: (0u32, 0u32),
            sprite_resolution: (0u32, 0u32),
            tile_geometry: (0u32, 0u32),
        }
    }
}

impl TerminalAPI {
    pub fn get_color_pallet(&self) -> &Arc<[Option<Color>]> {
        &self.color_pallet
    }

    pub fn build_tile(&self, asset_id: AssetId) -> Vec<Color> {
        // check if has been generated before, if so return it
        // if not generate and memoize.

        let image: Option<Option<Vec<Color>>> =
            self.assets.get(&asset_id).map(|asset| match asset {
                Asset::Tile(color_codes) => Some(
                    color_codes
                        .iter()
                        .filter_map(|code| self.color_pallet[*code as usize])
                        .collect(),
                ),
                _ => None,
            });

        image
            .unwrap_or(Some(Vec::default()))
            .unwrap_or(Vec::default())
    }

    pub fn recv_asset(&mut self, uuid: AssetId, asset: Asset) {
        if let Asset::ColorPallette(pallete) = asset {
            self.color_pallet = pallete.into_iter().map(|color| Some(color)).collect();
        } else {
            self.assets.insert(uuid, asset);
        }
    }

    // pub fn register_sent(&mut self, , message: Message<InstructFrontEnd>) ->

    pub fn prosses_msg(
        &mut self,
        message: Message<InstructFrontEnd>,
    ) -> Result<ProcessedMessageRes> {
        match message {
            Message::Ack { mesg: _ } => {}
            Message::ResendPls { mesg: _ } => {
                // TODO: resend requested message
            }
            Message::New {
                id,
                mesg,
                checksum: _,
                ack_required,
            } => {
                // TODO: return Ok(ProcessedMessageRes::resend(id)) on failure of checksum
                // validation

                match mesg {
                    InstructFrontEnd::SetResolution { width, height } => {
                        self.screen_resolution = (width, height);
                    }
                    InstructFrontEnd::SetTileSize { width, height } => {
                        self.tile_resolution = (width, height);
                    }
                    InstructFrontEnd::DisplayTiles { tiles: _ } => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    }
                    InstructFrontEnd::DespawnSprite {
                        location: _,
                        tile_id: _,
                    } => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    }
                    InstructFrontEnd::DisplaySprite {
                        location: _,
                        tile_id: _,
                    } => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    }
                    InstructFrontEnd::MoveSprite {
                        from: _,
                        tile_id: _,
                        to: _,
                    } => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    }
                    InstructFrontEnd::DisableControllerInput { id: _ } => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    }
                    InstructFrontEnd::DisplayLoadingScreen => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    }
                    InstructFrontEnd::DisplaySprites { sprites: _ } => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    }
                    InstructFrontEnd::LoadMap { map: _ } => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    }
                    InstructFrontEnd::SetSpriteSize { width, height } => {
                        self.sprite_resolution = (width, height);
                    }
                    InstructFrontEnd::LoadAsset {
                        uid: _,
                        asset_type: _,
                        data: _,
                    } => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    }
                    InstructFrontEnd::SetScreenSize { width, height } => {
                        self.tile_geometry = (width, height);
                    }
                    InstructFrontEnd::DisableKeyboardInput { id: _ } => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    }
                    InstructFrontEnd::DisableMouseInput { id: _ } => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    }
                    InstructFrontEnd::DisplayTile {
                        location: _,
                        tile_id: _,
                    } => {
                        return Ok(ProcessedMessageRes::TermMustHandle(mesg));
                    } // InstructFrontEnd:: => {}
                      // InstructFrontEnd:: => {}
                }

                if ack_required {
                    return Ok(ProcessedMessageRes::SendAck(id));
                }
            }
        }

        Ok(ProcessedMessageRes::Success)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
