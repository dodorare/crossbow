package com.dodorare.crossbow;

import androidx.fragment.app.Fragment;
import android.app.Activity;

public class Crossbow extends Fragment {
    private boolean use_apk_expansion;

	private void initializeCrossbow() {
        final Activity activity = getActivity();
        CrossbowLib.initialize(activity, this, activity.getAssets(), use_apk_expansion);
    }
}
