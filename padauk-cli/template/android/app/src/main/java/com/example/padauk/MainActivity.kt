package com.example.padauk

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import androidx.lifecycle.ViewModel
import com.example.padauk.ui.RustRenderer
import uniffi.rust_native.UiController

class MainActivity : ComponentActivity() {
    // Initialize the Rust Controller
    private val controller = UiController()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Configure logger
        uniffi.rust_native.initLogging()

        setContent {
            MainContent(
                MainViewModel(controller)
            )
        }
    }
}

class MainViewModel(private val controller: UiController) : ViewModel() {
    // The "Source of Truth" for the UI tree
    var uiTree by mutableStateOf(controller.render())
        private set

    fun onUIEvent(eventId: String, value: String?) {
        // 1. Tell Rust to do the work
        controller.handleEvent(eventId, value)
        // 2. Refresh the UI tree from Rust
        uiTree = controller.render()
    }
}

@Composable
fun MainContent(viewModel: MainViewModel) {
    // This will recompose whenever viewModel.uiTree changes
    RustRenderer(widget = viewModel.uiTree) { eventId, value ->
        viewModel.onUIEvent(eventId, value)
    }
}