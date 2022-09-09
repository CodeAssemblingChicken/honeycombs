use crate::components::TextSectionConfig;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    reflect::TypeUuid,
    utils::HashMap,
};
use serde::Deserialize;

#[derive(Deserialize, TypeUuid)]
#[uuid = "055b0f06-0237-4d6b-bcd1-504c4cc51f97"]
pub struct LocaleAsset {
    pub strings: HashMap<String, String>,
    pub text_sections: HashMap<String, Vec<TextSectionConfig>>,
}

#[derive(Default)]
pub struct LocaleAssetLoader;

impl AssetLoader for LocaleAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let locale_asset = ron::de::from_bytes::<LocaleAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(locale_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["lang"]
    }
}
