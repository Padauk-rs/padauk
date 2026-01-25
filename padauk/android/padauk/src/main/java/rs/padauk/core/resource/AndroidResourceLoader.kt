package rs.padauk.core.resource

import android.content.Context

import rs.padauk.core.ResourceLoader
import rs.padauk.core.PlatformException

class AndroidResourceLoader(private val context: Context) : ResourceLoader {
    override fun loadRawResource(name: String): ByteArray {
        // Rust passes the sanitized name (e.g., "my_data")
        val resId = context.resources.getIdentifier(name, "raw", context.packageName)

        if (resId == 0) {
            // We explicitly throw the generated error type so Rust handles it cleanly
            throw PlatformException.NotFound(name)
        }

        try {
            return context.resources.openRawResource(resId).use { it.readBytes() }
        } catch (e: Exception) {
            throw PlatformException.Generic(e.message ?: "Unknown IO error")
        }
    }
}