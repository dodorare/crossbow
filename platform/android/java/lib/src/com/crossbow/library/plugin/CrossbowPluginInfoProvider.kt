package com.crossbow.library.plugin

import com.crossbow.library.plugin.SignalInfo

/**
 * Provides the set of information expected from a Crossbow plugin.
 */
interface CrossbowPluginInfoProvider {
    /**
     * Returns the name of the plugin.
     */
    val pluginName: String

    /**
     * Returns the list of signals to be exposed to Crossbow.
     */
    val pluginSignals: Set<SignalInfo?>
        get() = emptySet<SignalInfo>()

    /**
     * This is invoked on the render thread when the plugin described by this instance has been
     * registered.
     */
    fun onPluginRegistered() {}
}
