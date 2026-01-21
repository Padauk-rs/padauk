package rs.padauk.core

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.material3.Text

open class PadaukActivity: ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()

        // 1. Hide the handshake behind reflection
        try {
            // First, load the user's native library
            System.loadLibrary("padauk")

            // Second, find the generated Kotlin class
            // Note: UniFFI appends "Kt" to the filename for top-level functions
            val rustClass = Class.forName("rs.padauk.app.RustKt")

            // Third, find and invoke the padaukInit() function
            val initMethod = rustClass.getMethod("padaukInit")
            initMethod.invoke(null) // null because it's a static method

            android.util.Log.d("Padauk", "Handshake successful via reflection")
        } catch (e: Exception) {
            android.util.Log.e("Padauk", "Failed to find user handshake: ${e.message}")
            // Optional: Show an error screen to the user
        }

        // Configure logger
        initLogging()

        setContent {
            if (padaukRenderRoot() != null) {
                PadaukRenderer(padaukRenderRoot()!!)
            } else {
                Text("Rust not initialized")
            }
        }
    }
}