package com.crossbow.library.plugin

import kotlin.annotation.Retention
import kotlin.annotation.AnnotationRetention

/**
 * Annotation to indicate methods that should be exposed to external call.
 *
 * At runtime, annotated plugin methods are detected and automatically registered.
 */
@Target(
    AnnotationTarget.FUNCTION,
    AnnotationTarget.PROPERTY_GETTER,
    AnnotationTarget.PROPERTY_SETTER
)
@Retention(AnnotationRetention.RUNTIME)
annotation class ExposedToCrossbow
