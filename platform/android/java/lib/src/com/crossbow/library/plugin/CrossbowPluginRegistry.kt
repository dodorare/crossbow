package com.crossbow.library.plugin

import com.crossbow.library.Crossbow
import java.util.concurrent.ConcurrentHashMap
import com.crossbow.library.plugin.CrossbowPlugin
import android.app.Activity
import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import android.os.Bundle
import com.crossbow.library.plugin.CrossbowPluginRegistry
import android.util.Log
import android.text.TextUtils
import java.lang.Class
import java.lang.reflect.Constructor
import java.lang.ClassNotFoundException
import java.lang.IllegalAccessException
import java.lang.NoSuchMethodException
import java.lang.reflect.InvocationTargetException
import android.content.pm.PackageManager.NameNotFoundException
import kotlin.Throws
import java.lang.IllegalStateException

/**
 * Registry used to load and access the registered Crossbow Android plugins.
 */
class CrossbowPluginRegistry private constructor(crossbow: Crossbow) {
    private val registry: ConcurrentHashMap<String, CrossbowPlugin>

    init {
        registry = ConcurrentHashMap()
        loadPlugins(crossbow)
    }

    /**
     * Retrieve the plugin tied to the given plugin name.
     * @param pluginName Name of the plugin
     * @return [CrossbowPlugin] handle if it exists, null otherwise.
     */
    fun getPlugin(pluginName: String): CrossbowPlugin? {
        return registry[pluginName]
    }

    /**
     * Retrieve the full set of loaded plugins.
     */
    val allPlugins: Collection<CrossbowPlugin>
        get() = registry.values

    private fun loadPlugins(crossbow: Crossbow) {
        try {
            @Suppress("DEPRECATION")
            val activity = crossbow.activity
            val appInfo = activity
                .packageManager
                .getApplicationInfo(
                    activity.packageName,
                    PackageManager.GET_META_DATA
                )
            val metaData = appInfo.metaData
            if (metaData == null || metaData.isEmpty) {
                return
            }
            val crossbowPluginV1NamePrefixLength = CROSSBOW_PLUGIN_V1_NAME_PREFIX.length
            for (metaDataName in metaData.keySet()) {
                // Parse the meta-data looking for entry with the Crossbow plugin name prefix.
                if (metaDataName.startsWith(CROSSBOW_PLUGIN_V1_NAME_PREFIX)) {
                    val pluginName =
                        metaDataName.substring(crossbowPluginV1NamePrefixLength).trim { it <= ' ' }
                    Log.i(TAG, "Initializing Crossbow plugin $pluginName")

                    // Retrieve the plugin class full name.
                    val pluginHandleClassFullName = metaData.getString(metaDataName)
                    if (!TextUtils.isEmpty(pluginHandleClassFullName)) {
                        try {
                            // Attempt to create the plugin init class via reflection.
                            @Suppress("UNCHECKED_CAST")
                            val pluginClass = Class
                                .forName(pluginHandleClassFullName!!) as Class<CrossbowPlugin>
                            val pluginConstructor = pluginClass
                                .getConstructor(Crossbow::class.java)
                            val pluginHandle = pluginConstructor.newInstance(crossbow)

                            // Load the plugin initializer into the registry using the plugin name as key.
                            if (pluginName != pluginHandle.pluginName) {
                                Log.w(
                                    TAG,
                                    "Meta-data plugin name does not match the value returned by the plugin handle: " + pluginName + " =/= " + pluginHandle.pluginName
                                )
                            }
                            registry[pluginName] = pluginHandle
                            Log.i(
                                TAG,
                                "Initialization of Crossbow plugin completed " + pluginHandle.pluginName
                            )
                        } catch (e: ClassNotFoundException) {
                            Log.w(TAG, "Unable to load Crossbow plugin $pluginName", e)
                        } catch (e: IllegalAccessException) {
                            Log.w(TAG, "Unable to load Crossbow plugin $pluginName", e)
                        } catch (e: InstantiationException) {
                            Log.w(TAG, "Unable to load Crossbow plugin $pluginName", e)
                        } catch (e: NoSuchMethodException) {
                            Log.w(TAG, "Unable to load Crossbow plugin $pluginName", e)
                        } catch (e: InvocationTargetException) {
                            Log.w(TAG, "Unable to load Crossbow plugin $pluginName", e)
                        }
                    } else {
                        Log.w(TAG, "Invalid plugin loader class for $pluginName")
                    }
                }
            }
        } catch (e: NameNotFoundException) {
            Log.e(TAG, "Unable load Crossbow Android plugins from the manifest file.", e)
        }
    }

    companion object {
        private val TAG = CrossbowPluginRegistry::class.java.simpleName
        private const val CROSSBOW_PLUGIN_V1_NAME_PREFIX = "com.crossbow.plugin.v1."
        private var instance: CrossbowPluginRegistry? = null

        /**
         * Parse the manifest file and load all included Crossbow Android plugins.
         *
         *
         * A plugin manifest entry is a '<meta-data>' tag setup as described in the [CrossbowPlugin]
         * documentation.
         *
         * @param crossbow Crossbow instance
         * @return A singleton instance of [CrossbowPluginRegistry]. This ensures that only one instance
         * of each Crossbow Android plugins is available at runtime.
        </meta-data> */
        fun initializePluginRegistry(crossbow: Crossbow): CrossbowPluginRegistry? {
            if (instance == null) {
                instance = CrossbowPluginRegistry(crossbow)
            }
            return instance
        }

        /**
         * Return the plugin registry if it's initialized.
         * Throws a [IllegalStateException] exception if not.
         *
         * @throws IllegalStateException if [CrossbowPluginRegistry.initializePluginRegistry] has not been called prior to calling this method.
         */
        @get:Throws(IllegalStateException::class)
        val pluginRegistry: CrossbowPluginRegistry?
            get() {
                checkNotNull(instance) { "Plugin registry hasn't been initialized." }
                return instance
            }
    }
}
