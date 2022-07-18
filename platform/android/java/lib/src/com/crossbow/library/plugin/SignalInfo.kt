package com.crossbow.library.plugin

import java.lang.Class
import android.text.TextUtils
import java.lang.IllegalArgumentException
import com.crossbow.library.JNIUtil
import java.util.Arrays

/**
 * Store information about a [CrossbowPlugin]'s signal.
 */
class SignalInfo(signalName: String, vararg argParamTypes: Class<*>) {
    val name: String
    val paramTypes: Array<Class<*>>
    val paramTypesNames: Array<String>

    init {
        require(!TextUtils.isEmpty(signalName)) { "Invalid signal name: $signalName" }
        name = signalName
        paramTypes = arrayOf(*argParamTypes)
        val tmpArray = arrayOfNulls<String>(paramTypes.size)
        for (i in paramTypes.indices) {
            val tmp = JNIUtil.getJNIClassSignature(paramTypes[i])
            if (tmp !== null) {
                tmpArray[i] = tmp
            }
        }
        paramTypesNames = tmpArray.filterNotNull().toTypedArray()
    }

    override fun toString(): String {
        return ("SignalInfo{"
                +
                "name='" + name + '\'' +
                ", paramsTypes=" + Arrays.toString(paramTypes) +
                '}')
    }

    override fun equals(o: Any?): Boolean {
        if (this === o) {
            return true
        }
        if (o !is SignalInfo) {
            return false
        }
        return name == o.name
    }

    override fun hashCode(): Int {
        return name.hashCode()
    }
}
