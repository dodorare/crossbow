package com.crossbow.library.plugin;

import com.crossbow.library.Crossbow;
import com.crossbow.library.JNIUtil;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.util.Log;
import android.view.Surface;
import android.view.View;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;

import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;

import javax.microedition.khronos.egl.EGLConfig;
import javax.microedition.khronos.opengles.GL10;

/**
 * Base class for the Crossbow Android plugins.
 * <p>
 * A Crossbow Android plugin is a regular Android library packaged as an aar archive file with the following caveats:
 * <p>
 * - The library must have a dependency on the Crossbow Android library (crossbow-lib.aar).
 * A stable version is available for each release.
 * <p>
 * - The library must include a <meta-data> tag in its manifest file setup as follow:
 * <meta-data android:name="com.crossbow.plugin.v1.[PluginName]" android:value="[plugin.init.ClassFullName]" />
 * Where:
 * - 'PluginName' is the name of the plugin.
 * - 'plugin.init.ClassFullName' is the full name (package + class name) of the plugin class
 * extending {@link CrossbowPlugin}.
 */
public abstract class CrossbowPlugin {
	private static final String TAG = CrossbowPlugin.class.getSimpleName();

	private final Crossbow crossbow;
	private final ConcurrentHashMap<String, SignalInfo> registeredSignals = new ConcurrentHashMap<>();

	public CrossbowPlugin(Crossbow crossbow) {
		this.crossbow = crossbow;
	}

	/**
	 * Provides access to the Crossbow engine.
	 */
	protected Crossbow getCrossbow() {
		return crossbow;
	}

	/**
	 * Provides access to the underlying {@link Activity}.
	 */
	@Nullable
	protected Activity getActivity() {
		return crossbow.getActivity();
	}

	/**
	 * Register the plugin with Crossbow native code.
	 *
	 * This method is invoked on the render thread.
	 */
	public final void onRegisterPluginWithCrossbowNative() {
		registeredSignals.putAll(registerPluginWithCrossbowNative(this, getPluginName(), getPluginSignals()));
	}

	/**
	 * Register the plugin with Crossbow native code.
	 *
	 * This method must be invoked on the render thread.
	 */
	public static void registerPluginWithCrossbowNative(Object pluginObject, CrossbowPluginInfoProvider pluginInfoProvider) {
		try {
			registerPluginWithCrossbowNative(pluginObject, pluginInfoProvider.getPluginName(), pluginInfoProvider.getPluginSignals());
		} catch (NoClassDefFoundError e) {
			Log.e(TAG, "Error getting declared methods", e);
		}
		// Notify that registration is complete.
		pluginInfoProvider.onPluginRegistered();
	}

	private static Map<String, SignalInfo> registerPluginWithCrossbowNative(Object pluginObject, String pluginName, Set<SignalInfo> pluginSignals) {
		nativeRegisterSingleton(pluginName, pluginObject);

		Class<?> clazz = pluginObject.getClass();
		Method[] methods = clazz.getDeclaredMethods();
		Log.i(TAG, "Registering plugin: " + pluginName + " (" + clazz.getName() + ")");

		for (Method method : methods) {
			// Check if the method is annotated with {@link ExposedToCrossbow}.
			if (method.getAnnotation(ExposedToCrossbow.class) != null) {
				String sig = JNIUtil.getJNIMethodSignature(method);
				nativeRegisterMethod(pluginName, method.getName(), sig);
				Log.i(TAG, "Registered " + pluginName + " plugin method: " + method.getName());
			}
		}

		// Register the signals for this plugin.
		Map<String, SignalInfo> registeredSignals = new HashMap<>();
		for (SignalInfo signalInfo : pluginSignals) {
			String signalName = signalInfo.getName();
			nativeRegisterSignal(pluginName, signalName, signalInfo.getParamTypesNames());
			registeredSignals.put(signalName, signalInfo);
		}

		return registeredSignals;
	}

	/**
	 * Invoked once during the Crossbow Android initialization process after creation of the
	 * {@link com.crossbow.library.CrossbowRenderView} view.
	 * <p>
	 * The plugin can return a non-null {@link View} layout in order to add it to the Crossbow view
	 * hierarchy.
	 *
	 * Use shouldBeOnTop() to set whether the plugin's {@link View} should be added on top or behind
	 * the main Crossbow view.
	 *
	 * @see Activity#onCreate(Bundle)
	 * @return the plugin's view to be included; null if no views should be included.
	 */
	@Nullable
	public View onMainCreate(Activity activity) {
		return null;
	}

	/**
	 * @see Activity#onActivityResult(int, int, Intent)
	 */
	public void onMainActivityResult(int requestCode, int resultCode, Intent data) {
	}

	/**
	 * @see Activity#onRequestPermissionsResult(int, String[], int[])
	 */
	public void onMainRequestPermissionsResult(int requestCode, String[] permissions, int[] grantResults) {
	}

	/**
	 * @see Activity#onPause()
	 */
	public void onMainPause() {}

	/**
	 * @see Activity#onResume()
	 */
	public void onMainResume() {}

	/**
	 * @see Activity#onDestroy()
	 */
	public void onMainDestroy() {}

	/**
	 * @see Activity#onBackPressed()
	 */
	public boolean onMainBackPressed() { return false; }

	/**
	 * Invoked on the render thread when the Crossbow setup is complete.
	 */
	public void onCrossbowSetupCompleted() {}

	/**
	 * Invoked on the render thread when the Crossbow main loop has started.
	 */
	public void onCrossbowMainLoopStarted() {}

	/**
	 * Invoked once per frame on the GL thread after the frame is drawn.
	 */
	public void onGLDrawFrame(GL10 gl) {}

	/**
	 * Called on the GL thread after the surface is created and whenever the OpenGL ES surface size
	 * changes.
	 */
	public void onGLSurfaceChanged(GL10 gl, int width, int height) {}

	/**
	 * Called on the GL thread when the surface is created or recreated.
	 */
	public void onGLSurfaceCreated(GL10 gl, EGLConfig config) {}

	/**
	 * Invoked once per frame on the Vulkan thread after the frame is drawn.
	 */
	public void onVkDrawFrame() {}

	/**
	 * Called on the Vulkan thread after the surface is created and whenever the surface size
	 * changes.
	 */
	public void onVkSurfaceChanged(Surface surface, int width, int height) {}

	/**
	 * Called on the Vulkan thread when the surface is created or recreated.
	 */
	public void onVkSurfaceCreated(Surface surface) {}

	/**
	 * Returns the name of the plugin.
	 * <p>
	 * This value must match the one listed in the plugin '<meta-data>' manifest entry.
	 */
	@NonNull
	public abstract String getPluginName();

	/**
	 * Returns the list of signals to be exposed to Crossbow.
	 */
	@NonNull
	public Set<SignalInfo> getPluginSignals() {
		return Collections.emptySet();
	}

	/**
	 * Returns whether the plugin's {@link View} returned in onMainCreate() should be placed on
	 * top of the main Crossbow view.
	 *
	 * Returning false causes the plugin's {@link View} to be placed behind, which can be useful
	 * when used with transparency in order to let the Crossbow view handle inputs.
	 */
	public boolean shouldBeOnTop() {
		return true;
	}

	/**
	 * Runs the specified action on the UI thread. If the current thread is the UI
	 * thread, then the action is executed immediately. If the current thread is
	 * not the UI thread, the action is posted to the event queue of the UI thread.
	 *
	 * @param action the action to run on the UI thread
	 */
	protected void runOnUiThread(Runnable action) {
		crossbow.runOnUiThread(action);
	}

	// /**
	//  * Queue the specified action to be run on the render thread.
	//  *
	//  * @param action the action to run on the render thread
	//  */
	// protected void runOnRenderThread(Runnable action) {
	// 	crossbow.runOnRenderThread(action);
	// }

	/**
	 * Emit a registered Crossbow signal.
	 * @param signalName Name of the signal to emit. It will be validated against the set of registered signals.
	 * @param signalArgs Arguments used to populate the emitted signal. The arguments will be validated against the {@link SignalInfo} matching the registered signalName parameter.
	 */
	protected void emitSignal(final String signalName, final Object... signalArgs) {
		try {
			// Check that the given signal is among the registered set.
			SignalInfo signalInfo = registeredSignals.get(signalName);
			if (signalInfo == null) {
				throw new IllegalArgumentException(
						"Signal " + signalName + " is not registered for this plugin.");
			}
			emitSignal(getCrossbow(), getPluginName(), signalInfo, signalArgs);
		} catch (IllegalArgumentException exception) {
			Log.w(TAG, exception.getMessage());
			// if (BuildConfig.DEBUG) {
			// 	throw exception;
			// }
		}
	}

	/**
	 * Emit a Crossbow signal.
	 * @param crossbow
	 * @param pluginName Name of the Crossbow plugin the signal will be emitted from. The plugin must already be registered with the Crossbow engine.
	 * @param signalInfo Information about the signal to emit.
	 * @param signalArgs Arguments used to populate the emitted signal. The arguments will be validated against the given {@link SignalInfo} parameter.
	 */
	public static void emitSignal(Crossbow crossbow, String pluginName, SignalInfo signalInfo, final Object... signalArgs) {
		try {
			if (signalInfo == null) {
				throw new IllegalArgumentException("Signal must be non null.");
			}

			// Validate the arguments count.
			Class<?>[] signalParamTypes = signalInfo.getParamTypes();
			if (signalArgs.length != signalParamTypes.length) {
				throw new IllegalArgumentException(
						"Invalid arguments count. Should be " + signalParamTypes.length + "  but is " + signalArgs.length);
			}

			// Validate the argument's types.
			for (int i = 0; i < signalParamTypes.length; i++) {
				if (!signalParamTypes[i].isInstance(signalArgs[i])) {
					throw new IllegalArgumentException(
							"Invalid type for argument #" + i + ". Should be of type " + signalParamTypes[i].getName());
				}
			}

			// crossbow.runOnRenderThread(() -> nativeEmitSignal(pluginName, signalInfo.getName(), signalArgs));
			nativeEmitSignal(pluginName, signalInfo.getName(), signalArgs);

		} catch (IllegalArgumentException exception) {
			Log.w(TAG, exception.getMessage());
			// if (BuildConfig.DEBUG) {
			// 	throw exception;
			// }
		}
	}

	/**
	 * Used to setup a {@link CrossbowPlugin} instance.
	 * @param p_name Name of the instance.
	 */
	private static native void nativeRegisterSingleton(String p_name, Object object);

	/**
	 * Used to complete registration of the {@link CrossbowPlugin} instance's methods.
	 * @param p_sname Name of the instance
	 * @param p_name Name of the method to register
	 * @param p_sig Signature of the registered method
	 */
	private static native void nativeRegisterMethod(String p_sname, String p_name, String p_sig);

	/**
	 * Used to complete registration of the {@link CrossbowPlugin} instance's methods.
	 * @param pluginName Name of the plugin
	 * @param signalName Name of the signal to register
	 * @param signalParamTypes Signal parameters types
	 */
	private static native void nativeRegisterSignal(String pluginName, String signalName, String[] signalParamTypes);

	/**
	 * Used to emit signal by {@link CrossbowPlugin} instance.
	 * @param pluginName Name of the plugin
	 * @param signalName Name of the signal to emit
	 * @param signalParams Signal parameters
	 */
	private static native void nativeEmitSignal(String pluginName, String signalName, Object[] signalParams);
}
