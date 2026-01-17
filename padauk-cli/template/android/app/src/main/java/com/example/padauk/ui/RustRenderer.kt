package com.example.padauk.ui

import android.graphics.RenderNode
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import uniffi.rust_native.Modifiers
import uniffi.rust_native.AndroidUiNode

@Composable
fun RustRenderer(widget: AndroidUiNode, onEvent: ((String, String?) -> Unit)? = null) {
    val modifiers = getModifiers(widget)

    val composeModifier = Modifier
        .padding(modifiers.padding.dp)
        .then(if (modifiers.backgroundColor != null)
            Modifier.background(Color(android.graphics.Color.parseColor(modifiers.backgroundColor)))
        else Modifier)

    when (widget) {
        is AndroidUiNode.Column -> {
            Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = composeModifier) {
                widget.children.forEach { RustRenderer(it, onEvent) }
            }
        }
//        is AndroidUiNode.Row -> {
//            Row(modifier = composeModifier) {
//                widget.children.forEach { RustRenderer(it, onEvent) }
//            }
//        }
        is AndroidUiNode.Text -> {
            Text(text = widget.text, fontSize = widget.spSize.sp, modifier = composeModifier)
        }
        is AndroidUiNode.Button -> {
            Button( modifier = composeModifier, onClick = { onEvent?.invoke(widget.onClick, null) }) {
                RustRenderer (widget.content.first())
            }
        }
//        is AndroidUiNode.TextField -> {
//            OutlinedTextField(
//                value = widget.value,
//                onValueChange = { newValue ->
//                    // Send the new string back to Rust immediately
//                    onEvent(widget.onChangeEvent, newValue)
//                },
//                label = { Text(widget.placeholder) },
//                modifier = composeModifier
//            )
//        }
    }
}



fun getModifiers(widget: AndroidUiNode): Modifiers {
    return when (widget) {
        is AndroidUiNode.Column -> widget.modifier
//        is AndroidUiNode.Row -> widget.modifier
        is AndroidUiNode.Text -> widget.modifier
        is AndroidUiNode.Button -> widget.modifier
//        is UiNode.TextField -> widget.modifiers
    }
}