package com.example.padauk

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.material3.Text
import rs.padauk.app.padaukInit
import rs.padauk.core.PadaukRenderer
import rs.padauk.core.padaukRenderRoot

class MainActivity : ComponentActivity() {
    // Initialize the Rust Controller

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        padaukInit()

        // Configure logger
        rs.padauk.core.initLogging()

        setContent {
            if (padaukRenderRoot() != null) {
                PadaukRenderer(padaukRenderRoot()!!)
            } else {
                Text("Rust not initialized")
            }
        }
    }
}
