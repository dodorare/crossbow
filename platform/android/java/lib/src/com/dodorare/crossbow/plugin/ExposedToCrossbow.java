package com.dodorare.crossbow.plugin;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

/**
 * Annotation to indicate methods that should be exposed to external call.
 *
 * At runtime, annotated plugin methods are detected and automatically registered.
 */
@Target({ ElementType.METHOD })
@Retention(RetentionPolicy.RUNTIME)
public @interface ExposedToCrossbow {}
