package com.example.minimal;

import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;

 class MainActivity : Activity() {
    override fun onCreate(savedInstanceState:Bundle?) {
        super.onCreate(savedInstanceState);

        val label = TextView(this);
        label.text = "Hello Kotlin!";
        setContentView(label);
    }
}
