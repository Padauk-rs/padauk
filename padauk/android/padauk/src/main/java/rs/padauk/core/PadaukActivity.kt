package rs.padauk.core

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.BackHandler
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.material3.Text
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import rs.padauk.core.resource.AndroidResourceLoader

open class PadaukActivity : ComponentActivity() {
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

        val loader = AndroidResourceLoader(this)
        registerResourceLoader(loader)


        // 2. Setup Re-rendering Listener
        // We use a mutable state integer as a trigger. Changing it forces recomposition.
        val refreshTrigger = mutableIntStateOf(0)

        val callback = object : RenderCallback {
            override fun onUpdate() {
                Log.d("Padauk", "Received update from Rust.")
                runOnUiThread {
                    refreshTrigger.intValue += 1
                }
            }
        }
        // Register this activity as the renderer
        registerRenderCallback(callback)


        setContent {
            // Read value to subscribe to updates
            val trigger = refreshTrigger.intValue

            // Intercept system back to pop navigator stack when possible.
            val canPop = padaukNavCanPop()
            BackHandler(enabled = canPop) {
                Log.d("Padauk", "System back -> Navigator.pop()")
                padaukNavPop()
            }


            // Fetch the latest tree from Rust
            val root = padaukRenderRoot()
            PadaukRenderer(root)
        }
    }
}
