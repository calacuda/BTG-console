use std::sync::Arc;

use btgc_core::{AssetId, Color, cart_to_term::Asset};
use rustc_hash::FxHashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalAPI {
    color_pallet: Arc<[Option<Color>]>,
    assets: FxHashMap<AssetId, Asset>,
}

impl Default for TerminalAPI {
    fn default() -> Self {
        Self {
            color_pallet: [].into(),
            assets: FxHashMap::default(),
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
