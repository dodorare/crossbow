package com.dodorare.crossbow

import com.dodorare.crossbow.Crossbow

/**
 * Denotate a component (e.g: Activity, Fragment) that hosts the [Crossbow] fragment.
 */
interface CrossbowHost {
    /**
     * Provides a set of command line parameters to setup the engine.
     */
    val commandLine: List<String?>?
        get() = emptyList<String>()

    /**
     * Invoked on the render thread when the Crossbow setup is complete.
     */
    fun onCrossbowSetupCompleted() {}

    /**
     * Invoked on the render thread when the Crossbow main loop has started.
     */
    fun onCrossbowMainLoopStarted() {}

    /**
     * Invoked on the UI thread as the last step of the Crossbow instance clean up phase.
     */
    fun onCrossbowForceQuit(instance: Crossbow?) {}

    /**
     * Invoked on the UI thread when the Crossbow instance wants to be restarted. It's up to the host
     * to perform the appropriate action(s).
     */
    fun onCrossbowRestartRequested(instance: Crossbow?) {}

    /**
     * Invoked on the UI thread when a new Crossbow instance is requested. It's up to the host to
     * perform the appropriate action(s).
     *
     * @param args Arguments used to initialize the new instance.
     */
    fun onNewCrossbowInstanceRequested(args: Array<String?>?) {}
}
