package com.dodorare.crossbow

import java.util.HashMap
import java.lang.reflect.Method
import java.lang.StringBuilder
import java.lang.Class

val Any.TAG: String
    get() {
        val tag = javaClass.simpleName
        return if (tag.length <= 23) tag else tag.substring(0, 23)
    }

internal object JNIUtil {
    private val PRIMITIVE_SIGNATURES: MutableMap<Any?, String> = HashMap()

    init {
        PRIMITIVE_SIGNATURES[Boolean::class.javaPrimitiveType] = "Z"
        PRIMITIVE_SIGNATURES[Byte::class.javaPrimitiveType] = "B"
        PRIMITIVE_SIGNATURES[Char::class.javaPrimitiveType] = "C"
        PRIMITIVE_SIGNATURES[Double::class.javaPrimitiveType] = "D"
        PRIMITIVE_SIGNATURES[Float::class.javaPrimitiveType] = "F"
        PRIMITIVE_SIGNATURES[Int::class.javaPrimitiveType] = "I"
        PRIMITIVE_SIGNATURES[Long::class.javaPrimitiveType] = "J"
        PRIMITIVE_SIGNATURES[Short::class.javaPrimitiveType] = "S"
        PRIMITIVE_SIGNATURES[Void.TYPE] = "V"
    }

    /**
     * Build JNI signature for a method
     * @param m
     * @return
     */
    @JvmStatic
    fun getJNIMethodSignature(m: Method): String {
        val sb = StringBuilder("(")
        for (p in m.parameterTypes) {
            sb.append(getJNIClassSignature(p))
        }
        sb.append(')').append(getJNIClassSignature(m.returnType))
        return sb.toString()
    }

    /**
     * Build JNI signature from a class
     * @param c
     * @return
     */
    @JvmStatic
    fun getJNIClassSignature(c: Class<*>): String? {
        return if (c.isArray) {
            val ct = c.componentType
            if (ct === null) {
                return null
            }
            '['.toString() + getJNIClassSignature(ct)
        } else if (c.isPrimitive) {
            PRIMITIVE_SIGNATURES[c]
        } else {
            'L'.toString() + c.name.replace('.', '/') + ';'
        }
    }
}
