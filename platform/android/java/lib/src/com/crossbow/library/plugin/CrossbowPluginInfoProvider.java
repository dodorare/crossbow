package com.crossbow.library.plugin;

import androidx.annotation.NonNull;

import java.util.Collections;
import java.util.Set;

/**
 * Provides the set of information expected from a Crossbow plugin.
 */
public interface CrossbowPluginInfoProvider {
	/**
	 * Returns the name of the plugin.
	 */
	@NonNull
	String getPluginName();

	/**
	 * Returns the list of signals to be exposed to Crossbow.
	 */
	@NonNull
	default Set<SignalInfo> getPluginSignals() {
		return Collections.emptySet();
	}

	/**
	 * This is invoked on the render thread when the plugin described by this instance has been
	 * registered.
	 */
	default void onPluginRegistered() {
	}
}
