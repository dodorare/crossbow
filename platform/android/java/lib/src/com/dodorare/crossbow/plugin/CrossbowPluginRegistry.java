package com.dodorare.crossbow.plugin;

import com.dodorare.crossbow.Crossbow;

import android.app.Activity;
import android.content.pm.ApplicationInfo;
import android.content.pm.PackageManager;
import android.os.Bundle;
import android.text.TextUtils;
import android.util.Log;

import androidx.annotation.Nullable;

import java.lang.reflect.Constructor;
import java.lang.reflect.InvocationTargetException;
import java.util.Collection;
import java.util.concurrent.ConcurrentHashMap;

/**
 * Registry used to load and access the registered Crossbow Android plugins.
 */
public final class CrossbowPluginRegistry {
	private static final String TAG = CrossbowPluginRegistry.class.getSimpleName();

	private static final String CROSSBOW_PLUGIN_V1_NAME_PREFIX = "com.crossbow.plugin.v1.";

	private static CrossbowPluginRegistry instance;
	private final ConcurrentHashMap<String, CrossbowPlugin> registry;

	private CrossbowPluginRegistry(Crossbow crossbow) {
		registry = new ConcurrentHashMap<>();
		loadPlugins(crossbow);
	}

	/**
	 * Retrieve the plugin tied to the given plugin name.
	 * @param pluginName Name of the plugin
	 * @return {@link CrossbowPlugin} handle if it exists, null otherwise.
	 */
	@Nullable
	public CrossbowPlugin getPlugin(String pluginName) {
		return registry.get(pluginName);
	}

	/**
	 * Retrieve the full set of loaded plugins.
	 */
	public Collection<CrossbowPlugin> getAllPlugins() {
		return registry.values();
	}

	/**
	 * Parse the manifest file and load all included Crossbow Android plugins.
	 * <p>
	 * A plugin manifest entry is a '<meta-data>' tag setup as described in the {@link CrossbowPlugin}
	 * documentation.
	 *
	 * @param crossbow Crossbow instance
	 * @return A singleton instance of {@link CrossbowPluginRegistry}. This ensures that only one instance
	 * of each Crossbow Android plugins is available at runtime.
	 */
	public static CrossbowPluginRegistry initializePluginRegistry(Crossbow crossbow) {
		if (instance == null) {
			instance = new CrossbowPluginRegistry(crossbow);
		}

		return instance;
	}

	/**
	 * Return the plugin registry if it's initialized.
	 * Throws a {@link IllegalStateException} exception if not.
	 *
	 * @throws IllegalStateException if {@link CrossbowPluginRegistry#initializePluginRegistry(Crossbow)} has not been called prior to calling this method.
	 */
	public static CrossbowPluginRegistry getPluginRegistry() throws IllegalStateException {
		if (instance == null) {
			throw new IllegalStateException("Plugin registry hasn't been initialized.");
		}

		return instance;
	}

	private void loadPlugins(Crossbow crossbow) {
		try {
			final Activity activity = crossbow.getActivity();
			ApplicationInfo appInfo = activity
											  .getPackageManager()
											  .getApplicationInfo(activity.getPackageName(),
													  PackageManager.GET_META_DATA);
			Bundle metaData = appInfo.metaData;
			if (metaData == null || metaData.isEmpty()) {
				return;
			}

			int crossbowPluginV1NamePrefixLength = CROSSBOW_PLUGIN_V1_NAME_PREFIX.length();
			for (String metaDataName : metaData.keySet()) {
				// Parse the meta-data looking for entry with the Crossbow plugin name prefix.
				if (metaDataName.startsWith(CROSSBOW_PLUGIN_V1_NAME_PREFIX)) {
					String pluginName = metaDataName.substring(crossbowPluginV1NamePrefixLength).trim();
					Log.i(TAG, "Initializing Crossbow plugin " + pluginName);

					// Retrieve the plugin class full name.
					String pluginHandleClassFullName = metaData.getString(metaDataName);
					if (!TextUtils.isEmpty(pluginHandleClassFullName)) {
						try {
							// Attempt to create the plugin init class via reflection.
							@SuppressWarnings("unchecked")
							Class<CrossbowPlugin> pluginClass = (Class<CrossbowPlugin>)Class
																	 .forName(pluginHandleClassFullName);
							Constructor<CrossbowPlugin> pluginConstructor = pluginClass
																				 .getConstructor(Crossbow.class);
							CrossbowPlugin pluginHandle = pluginConstructor.newInstance(crossbow);

							// Load the plugin initializer into the registry using the plugin name as key.
							if (!pluginName.equals(pluginHandle.getPluginName())) {
								Log.w(TAG,
										"Meta-data plugin name does not match the value returned by the plugin handle: " + pluginName + " =/= " + pluginHandle.getPluginName());
							}
							registry.put(pluginName, pluginHandle);
							Log.i(TAG, "Completed initialization for Crossbow plugin " + pluginHandle.getPluginName());
						} catch (ClassNotFoundException e) {
							Log.w(TAG, "Unable to load Crossbow plugin " + pluginName, e);
						} catch (IllegalAccessException e) {
							Log.w(TAG, "Unable to load Crossbow plugin " + pluginName, e);
						} catch (InstantiationException e) {
							Log.w(TAG, "Unable to load Crossbow plugin " + pluginName, e);
						} catch (NoSuchMethodException e) {
							Log.w(TAG, "Unable to load Crossbow plugin " + pluginName, e);
						} catch (InvocationTargetException e) {
							Log.w(TAG, "Unable to load Crossbow plugin " + pluginName, e);
						}
					} else {
						Log.w(TAG, "Invalid plugin loader class for " + pluginName);
					}
				}
			}
		} catch (PackageManager.NameNotFoundException e) {
			Log.e(TAG, "Unable load Crossbow Android plugins from the manifest file.", e);
		}
	}
}
