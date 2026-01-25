package rs.padauk.core

import android.graphics.RenderNode
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun PadaukRenderer(widget: AndroidUiNode, onEvent: ((String, String?) -> Unit)? = null) {
    when (widget) {
        is AndroidUiNode.Scaffold -> {
            Scaffold(
                modifier = widget.modifiers.toCompose(),
                topBar = {
                    // Check if the vector has items
                    if (widget.appBar.isNotEmpty()) {
                        PadaukRenderer(widget.appBar.first())
                    }
                },
                floatingActionButton = {
                    if (widget.floatingActionButton.isNotEmpty()) {
                        PadaukRenderer(widget.floatingActionButton.first())
                    }
                }
            ) { innerPadding ->
                // IMPORTANT: We apply the innerPadding to the body
                // This ensures content doesn't go behind the AppBar
                Box(modifier = Modifier.padding(innerPadding)) {
                    if (widget.body.isNotEmpty()) {
                        PadaukRenderer(widget.body.first())
                    }
                }
            }
        }
        is AndroidUiNode.AppBar -> {
            TopAppBar(
                title = { Text(text = widget.title) },
                modifier = widget.modifiers.toCompose(),
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.primaryContainer,
                    titleContentColor = MaterialTheme.colorScheme.onPrimaryContainer,
                )
            )
        }

        is AndroidUiNode.Column -> {
            Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = widget.modifier.toCompose()) {
                widget.children.forEach { PadaukRenderer(it, onEvent) }
            }
        }
        is AndroidUiNode.Row -> {
            Row(modifier = widget.modifiers.toCompose()) {
                widget.children.forEach { PadaukRenderer(it, onEvent) }
            }
        }
        is AndroidUiNode.Stack -> {
            Box(modifier = widget.modifiers.toCompose()) {
                widget.children.forEach { PadaukRenderer(it, onEvent) }
            }
        }
        is AndroidUiNode.Text -> {
            Text(text = widget.text, fontSize = widget.spSize.sp, modifier = widget.modifier.toCompose())
        }
        is AndroidUiNode.Button -> {
            Button( modifier = widget.modifier.toCompose(), onClick = { onEvent?.invoke(widget.actionId, null) }) {
                PadaukRenderer (widget.content.first())
            }
        }
        is AndroidUiNode.Image -> {
            val context = LocalContext.current
            val resId = context.resources.getIdentifier(widget.resourceName, "drawable", context.packageName)
            if (resId != 0) {
                androidx.compose.foundation.Image(
                    painter = painterResource(id = resId),
                    contentDescription = null,
                    modifier = widget.modifiers.toCompose()
                )
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



fun Modifiers.toCompose(): Modifier {
    var m : Modifier = Modifier
    this.padding.let { m = m.padding(it.dp) }
//    this.width?.let { m = m.width(it.dp) }
//    this.height?.let { m = m.height(it.dp) }
    this.backgroundColor?.let {
        m = m.then(Modifier.background(it.toComposeColor()))
    }
    // Add clickable, etc here
    return m
}

fun String.toComposeColor(): Color {
    return try {
        Color(android.graphics.Color.parseColor(this))
    } catch (e: Exception) {
        Color.Black
    }
}