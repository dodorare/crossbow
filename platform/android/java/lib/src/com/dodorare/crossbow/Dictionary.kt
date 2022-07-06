package com.dodorare.crossbow

import java.util.HashMap

class Dictionary : HashMap<String?, Any?>() {
    protected var keys_cache: Array<String>? = null

    fun get_keys(): Array<String?> {
        val ret = arrayOfNulls<String>(size)
        var i = 0
        val keys: MutableSet<String?> = keys
        for (key in keys) {
            ret[i] = key
            i++
        }
        return ret
    }

    fun get_values(): Array<Any?> {
        val ret = arrayOfNulls<Any>(size)
        var i = 0
        val keys: MutableSet<String?> = keys
        for (key in keys) {
            ret[i] = get(key)
            i++
        }
        return ret
    }

    fun set_keys(keys: Array<String>?) {
        keys_cache = keys
    }

    fun set_values(vals: Array<Any?>) {
        var i = 0
        for (key in keys_cache!!) {
            put(key, vals[i])
            i++
        }
        keys_cache = null
    }
}
