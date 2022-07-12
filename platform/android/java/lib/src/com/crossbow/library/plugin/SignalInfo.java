package com.crossbow.library.plugin;

import android.text.TextUtils;
import java.util.Arrays;
import androidx.annotation.NonNull;
import com.crossbow.library.JNIUtil;

/**
 * Store information about a {@link CrossbowPlugin}'s signal.
 */
public final class SignalInfo {
	private final String name;
	private final Class<?>[] paramTypes;
	private final String[] paramTypesNames;

	public SignalInfo(@NonNull String signalName, Class<?>... paramTypes) {
		if (TextUtils.isEmpty(signalName)) {
			throw new IllegalArgumentException("Invalid signal name: " + signalName);
		}

		this.name = signalName;
		this.paramTypes = paramTypes == null ? new Class<?>[ 0 ] : paramTypes;
		this.paramTypesNames = new String[this.paramTypes.length];
		for (int i = 0; i < this.paramTypes.length; i++) {
			this.paramTypesNames[i] = JNIUtil.getJNIClassSignature(this.paramTypes[i]);
		}
	}

	public String getName() {
		return name;
	}

	Class<?>[] getParamTypes() {
		return paramTypes;
	}

	String[] getParamTypesNames() {
		return paramTypesNames;
	}

	@Override
	public String toString() {
		return "SignalInfo{"
				+
				"name='" + name + '\'' +
				", paramsTypes=" + Arrays.toString(paramTypes) +
				'}';
	}

	@Override
	public boolean equals(Object o) {
		if (this == o) {
			return true;
		}
		if (!(o instanceof SignalInfo)) {
			return false;
		}

		SignalInfo that = (SignalInfo)o;

		return name.equals(that.name);
	}

	@Override
	public int hashCode() {
		return name.hashCode();
	}
}
