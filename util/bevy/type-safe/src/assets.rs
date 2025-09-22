use bevy::prelude::{App, Asset, AssetServer, Assets};
use std::marker::PhantomData;

/// Errors thrown by the [SafeAssetTrait].
#[derive(Debug, thiserror::Error)]
pub enum SafeAssetError {
	#[error("Asset not initialized: {0}")]
	AssetNotInitialized(String),
}

/// Marker for a safe asset
#[derive(Debug)]
pub struct SafeAssetMarker<A: Asset> {
	marker: PhantomData<A>,
}

pub trait SafeAssetTrait {
	fn try_safe_asset<A: Asset>(&self) -> Result<SafeAssetMarker<A>, SafeAssetError>;
}

impl SafeAssetTrait for App {
	fn try_safe_asset<A: Asset>(&self) -> Result<SafeAssetMarker<A>, SafeAssetError> {
		// if the world does not contain the asset server, then the asset cannot have been initialized
		if !self.world().contains_resource::<AssetServer>() {
			return Err(SafeAssetError::AssetNotInitialized(
				core::any::type_name::<A>().to_string(),
			));
		}

		if !self.world().contains_resource::<Assets<A>>() {
			return Err(SafeAssetError::AssetNotInitialized(
				core::any::type_name::<A>().to_string(),
			));
		}
		Ok(SafeAssetMarker { marker: PhantomData })
	}
}
