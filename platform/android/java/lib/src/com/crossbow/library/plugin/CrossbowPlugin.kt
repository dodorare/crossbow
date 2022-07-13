@file:Suppress("UNUSED_PARAMETER")

package com.crossbow.library.plugin

import com.crossbow.library.Crossbow
import java.util.concurrent.ConcurrentHashMap
import com.crossbow.library.plugin.SignalInfo
import android.app.Activity
import com.crossbow.library.plugin.CrossbowPlugin
import android.view.View
import android.content.Intent
import javax.microedition.khronos.opengles.GL10
import java.lang.Runnable
import java.lang.IllegalArgumentException
import android.util.Log
import android.view.Surface
import com.crossbow.library.plugin.CrossbowPluginInfoProvider
import java.lang.NoClassDefFoundError
import java.lang.Class
import java.lang.reflect.Method
import com.crossbow.library.plugin.ExposedToCrossbow
import com.crossbow.library.JNIUtil
import java.util.HashMap
import javax.microedition.khronos.egl.EGLConfig

/**
 * Base class for the Crossbow Android plugins.
 *
 *
 * A Crossbow Android plugin is a regular Android library packaged as an aar archive file with the following caveats:
 *
 *
 * - The library must have a dependency on the Crossbow Android library (crossbow-lib.aar).
 * A stable version is available for each release.
 *
 *
 * - The library must include a <meta-data> tag in its manifest file setup as follow:
 * <meta-data android:name="com.crossbow.plugin.v1.[PluginName]" android:value="[plugin.init.ClassFullName]"></meta-data>
 * Where:
 * - 'PluginName' is the name of the plugin.
 * - 'plugin.init.ClassFullName' is the full name (package + class name) of the plugin class
 * extending [CrossbowPlugin].
</meta-data> */
abstract class CrossbowPlugin(
    /**
     * Provides access to the Crossbow engine.
     */
    protected val crossbow: Crossbow
) {
    private val registeredSignals = ConcurrentHashMap<String, SignalInfo?>()

    /**
     * Provides access to the underlying [Activity].
     */
    protected val activity: Activity?
        get() = crossbow.activity

    /**
     * Register the plugin with Crossbow native code.
     *
     * This method is invoked on the render thread.
     */
    open fun onRegisterPluginWithCrossbowNative() {
        registeredSignals.putAll(registerPluginWithCrossbowNative(this, pluginName, pluginSignals))
    }

    /**
     * Invoked once during the Crossbow Android initialization process after creation of the
     * [com.crossbow.library.CrossbowRenderView] view.
     *
     *
     * The plugin can return a non-null [View] layout in order to add it to the Crossbow view
     * hierarchy.
     *
     * Use shouldBeOnTop() to set whether the plugin's [View] should be added on top or behind
     * the main Crossbow view.
     *
     * @see Activity.onCreate
     * @return the plugin's view to be included; null if no views should be included.
     */
    open fun onMainCreate(activity: Activity): View? {
        return null
    }

    /**
     * @see Activity.onActivityResult
     */
    open fun onMainActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {}

    /**
     * @see Activity.onRequestPermissionsResult
     */
    open fun onMainRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<String>,
        grantResults: IntArray?
    ) {
    }

    /**
     * @see Activity.onPause
     */
    open fun onMainPause() {}

    /**
     * @see Activity.onResume
     */
    open fun onMainResume() {}

    /**
     * @see Activity.onDestroy
     */
    open fun onMainDestroy() {}

    /**
     * @see Activity.onBackPressed
     */
    open fun onMainBackPressed(): Boolean {
        return false
    }

    /**
     * Invoked on the render thread when the Crossbow setup is complete.
     */
    open fun onCrossbowSetupCompleted() {}

    /**
     * Invoked on the render thread when the Crossbow main loop has started.
     */
    open fun onCrossbowMainLoopStarted() {}

    /**
     * Invoked once per frame on the GL thread after the frame is drawn.
     */
    open fun onGLDrawFrame(gl: GL10?) {}

    /**
     * Called on the GL thread after the surface is created and whenever the OpenGL ES surface size
     * changes.
     */
    open fun onGLSurfaceChanged(gl: GL10?, width: Int, height: Int) {}

    /**
     * Called on the GL thread when the surface is created or recreated.
     */
    open fun onGLSurfaceCreated(gl: GL10?, config: EGLConfig?) {}

    /**
     * Invoked once per frame on the Vulkan thread after the frame is drawn.
     */
    open fun onVkDrawFrame() {}

    /**
     * Called on the Vulkan thread after the surface is created and whenever the surface size
     * changes.
     */
    open fun onVkSurfaceChanged(surface: Surface?, width: Int, height: Int) {}

    /**
     * Called on the Vulkan thread when the surface is created or recreated.
     */
    open fun onVkSurfaceCreated(surface: Surface?) {}

    /**
     * Returns the name of the plugin.
     *
     *
     * This value must match the one listed in the plugin '<meta-data>' manifest entry.
    </meta-data> */
    abstract val pluginName: String

    /**
     * Returns the list of signals to be exposed to Crossbow.
     */
    open val pluginSignals: Set<SignalInfo>
        get() = emptySet<SignalInfo>()

    /**
     * Returns whether the plugin's [View] returned in onMainCreate() should be placed on
     * top of the main Crossbow view.
     *
     * Returning false causes the plugin's [View] to be placed behind, which can be useful
     * when used with transparency in order to let the Crossbow view handle inputs.
     */
    open fun shouldBeOnTop(): Boolean {
        return true
    }

    /**
     * Runs the specified action on the UI thread. If the current thread is the UI
     * thread, then the action is executed immediately. If the current thread is
     * not the UI thread, the action is posted to the event queue of the UI thread.
     *
     * @param action the action to run on the UI thread
     */
    protected fun runOnUiThread(action: Runnable?) {
        crossbow.runOnUiThread(action!!)
    }

    /**
     * Queue the specified action to be run on the render thread.
     *
     * @param action the action to run on the render thread
     */
    // protected fun runOnRenderThread(action: Runnable?) {
    // 	crossbow.runOnRenderThread(action)
    // }

    /**
     * Emit a registered Crossbow signal.
     * @param signalName Name of the signal to emit. It will be validated against the set of registered signals.
     * @param signalArgs Arguments used to populate the emitted signal. The arguments will be validated against the [SignalInfo] matching the registered signalName parameter.
     */
    protected fun emitSignal(signalName: String, vararg signalArgs: Any?) {
        try {
            // Check that the given signal is among the registered set.
            val signalInfo = registeredSignals[signalName]
                ?: throw IllegalArgumentException(
                    "Signal $signalName is not registered for this plugin."
                )
            emitSignal(crossbow, pluginName, signalInfo, *signalArgs)
        } catch (exception: IllegalArgumentException) {
            Log.w(TAG, exception.message!!)
            // if (BuildConfig.DEBUG) {
            // 	throw exception;
            // }
        }
    }

    companion object {
        private val TAG = CrossbowPlugin::class.java.simpleName

        /**
         * Register the plugin with Crossbow native code.
         *
         * This method must be invoked on the render thread.
         */
        fun registerPluginWithCrossbowNative(
            pluginObject: Any,
            pluginInfoProvider: CrossbowPluginInfoProvider
        ) {
            try {
                registerPluginWithCrossbowNative(
                    pluginObject,
                    pluginInfoProvider.pluginName,
                    pluginInfoProvider.pluginSignals
                )
            } catch (e: NoClassDefFoundError) {
                Log.e(TAG, "Error getting declared methods", e)
            }
            // Notify that registration is complete.
            pluginInfoProvider.onPluginRegistered()
        }

        private fun registerPluginWithCrossbowNative(
            pluginObject: Any,
            pluginName: String,
            pluginSignals: Set<SignalInfo?>
        ): Map<String, SignalInfo?> {
            nativeRegisterSingleton(pluginName, pluginObject)
            val clazz: Class<*> = pluginObject.javaClass
            val methods = clazz.declaredMethods
            Log.i(TAG, "Registering plugin: " + pluginName + " (" + clazz.name + ")")
            for (method in methods) {
                // Check if the method is annotated with {@link ExposedToCrossbow}.
                if (method.getAnnotation(ExposedToCrossbow::class.java) != null) {
                    val sig = JNIUtil.getJNIMethodSignature(method)
                    nativeRegisterMethod(pluginName, method.name, sig)
                    Log.i(TAG, "Registered " + pluginName + " plugin method: " + method.name)
                }
            }

            // Register the signals for this plugin.
            val registeredSignals: MutableMap<String, SignalInfo?> = HashMap()
            for (signalInfo in pluginSignals) {
                val signalName = signalInfo!!.name
                nativeRegisterSignal(pluginName, signalName, signalInfo.paramTypesNames)
                registeredSignals[signalName] = signalInfo
            }
            return registeredSignals
        }

        /**
         * Emit a Crossbow signal.
         * @param crossbow
         * @param pluginName Name of the Crossbow plugin the signal will be emitted from. The plugin must already be registered with the Crossbow engine.
         * @param signalInfo Information about the signal to emit.
         * @param signalArgs Arguments used to populate the emitted signal. The arguments will be validated against the given [SignalInfo] parameter.
         */
        fun emitSignal(
            crossbow: Crossbow?,
            pluginName: String,
            signalInfo: SignalInfo?,
            vararg signalArgs: Any?
        ) {
            try {
                requireNotNull(signalInfo) { "Signal must be non null." }

                // Validate the arguments count.
                val signalParamTypes: Array<Class<*>> = signalInfo.paramTypes
                require(signalArgs.size == signalParamTypes.size) { "Invalid arguments count. Should be " + signalParamTypes.size + "  but is " + signalArgs.size }

                // Validate the argument's types.
                for (i in signalParamTypes.indices) {
                    require(signalParamTypes[i].isInstance(signalArgs[i])) { "Invalid type for argument #" + i + ". Should be of type " + signalParamTypes[i].name }
                }

                // crossbow.runOnRenderThread(() -> nativeEmitSignal(pluginName, signalInfo.getName(), signalArgs));
                nativeEmitSignal(pluginName, signalInfo.name, arrayOf(*signalArgs))
            } catch (exception: IllegalArgumentException) {
                Log.w(TAG, exception.message!!)
                // if (BuildConfig.DEBUG) {
                // 	throw exception;
                // }
            }
        }

        /**
         * Used to setup a [CrossbowPlugin] instance.
         * @param p_name Name of the instance.
         */
        private external fun nativeRegisterSingleton(p_name: String, `object`: Any)

        /**
         * Used to complete registration of the [CrossbowPlugin] instance's methods.
         * @param p_sname Name of the instance
         * @param p_name Name of the method to register
         * @param p_sig Signature of the registered method
         */
        private external fun nativeRegisterMethod(p_sname: String, p_name: String, p_sig: String)

        /**
         * Used to complete registration of the [CrossbowPlugin] instance's methods.
         * @param pluginName Name of the plugin
         * @param signalName Name of the signal to register
         * @param signalParamTypes Signal parameters types
         */
        private external fun nativeRegisterSignal(
            pluginName: String,
            signalName: String,
            signalParamTypes: Array<String>
        )

        /**
         * Used to emit signal by [CrossbowPlugin] instance.
         * @param pluginName Name of the plugin
         * @param signalName Name of the signal to emit
         * @param signalParams Signal parameters
         */
        private external fun nativeEmitSignal(
            pluginName: String,
            signalName: String,
            signalParams: Array<Any?>
        )
    }
}
