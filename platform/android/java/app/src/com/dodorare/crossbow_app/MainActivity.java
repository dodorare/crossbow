package com.dodorare.crossbow_app;

import com.dodorare.crossbow.CrossbowGame;

import android.os.Bundle;

/**
 * Template activity for Crossbow Android custom builds.
 * Feel free to extend and modify this class for your custom logic.
 */
public class CrossbowApp extends CrossbowGame {
    @Override
	public void onCreate(Bundle savedInstanceState) {
		setTheme(R.style.GodotAppMainTheme);
		super.onCreate(savedInstanceState);
	}
}
