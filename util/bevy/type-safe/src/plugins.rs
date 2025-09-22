use bevy::prelude::{App, Plugin};
use std::marker::PhantomData;

/// Trait for adding plugins conditionally
/// If the plugin is already added, it will not be added again
/// If the plugin is not added, it will be added
/// This is useful for adding plugins conditionally
/// For example, if you want to add a plugin only if a certain condition is met
/// You can use this trait to add the plugin conditionally
/// This is useful for adding plugins conditionally
pub trait MaybePlugin {
	fn maybe_add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self;
}

/// Implementation of the MaybePlugin trait for the App type
impl MaybePlugin for App {
	fn maybe_add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
		if !self.is_plugin_added::<T>() {
			self.add_plugins(plugin);
		}

		self
	}
}

/// Errors thrown by the [Graph].
#[derive(Debug, thiserror::Error)]
pub enum SafePluginError {
	#[error("Plugin not added: {0}")]
	PluginNotAdded(String),
}

/// Marks a safe plugin mapping runtime safety to type safety.
#[derive(Debug)]
pub struct SafePluginMarker<T: Plugin> {
	marker: PhantomData<T>,
}

/// Trait for adding safe plugins to the app.
pub trait SafePluginTrait {
	fn try_safe_plugin<T: Plugin>(&self) -> Result<SafePluginMarker<T>, SafePluginError>;
}

/// Implementation of the SafePluginTrait for the App type
impl SafePluginTrait for App {
	fn try_safe_plugin<T: Plugin>(&self) -> Result<SafePluginMarker<T>, SafePluginError> {
		if !self.is_plugin_added::<T>() {
			return Err(SafePluginError::PluginNotAdded(format!(
				"Plugin {} not added",
				core::any::type_name::<T>()
			)));
		}

		Ok(SafePluginMarker { marker: PhantomData })
	}
}

/// Macro for checking multiple plugins are added to the app.
#[macro_export]
macro_rules! try_safe_plugins {
    ($app:expr, $($plugin:ty),+ $(,)?) => {{
        (
            $(
                {
                    if !$app.is_plugin_added::<$plugin>() {
                        return Err($crate::SafePluginError::PluginNotAdded(
                            <$plugin>::NAME.to_string()
                        ));
                    }
                    $crate::SafePlugin::<$plugin>::new(Default::default())
                }
            ),+
        )
    }};
}
