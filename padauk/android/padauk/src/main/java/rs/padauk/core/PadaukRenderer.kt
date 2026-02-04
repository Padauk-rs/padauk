package rs.padauk.core

import android.annotation.SuppressLint
import android.graphics.Color.*
import android.graphics.RenderNode
import android.util.Log
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.Button
import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.LargeTopAppBar
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MediumTopAppBar
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
import rs.padauk.core.widget.PadaukImage
import rs.padauk.core.widget.toCompose

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun PadaukRenderer(widget: AndroidUiNode) {
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
            val navIcon: @Composable () -> Unit = {
                if (widget.leading.isNotEmpty()) {
                    val leading = widget.leading.first()
                    val backActionId = extractBackActionId(leading)
                    if (backActionId != null) {
                        IconButton(onClick = { padaukDispatchAction(backActionId) }) {
                            Icon(
                                imageVector = Icons.AutoMirrored.Filled.ArrowBack,
                                contentDescription = "Back"
                            )
                        }
                    } else {
                        PadaukRenderer(leading)
                    }
                }
            }

            val colors = TopAppBarDefaults.topAppBarColors(
                containerColor = MaterialTheme.colorScheme.primaryContainer,
                titleContentColor = MaterialTheme.colorScheme.onPrimaryContainer,
            )

            when (widget.style) {
                AppBarStyle.SMALL -> TopAppBar(
                    title = { Text(text = widget.title) },
                    modifier = widget.modifiers.toCompose(),
                    colors = colors,
                    navigationIcon = navIcon
                )
                AppBarStyle.CENTER_ALIGNED -> CenterAlignedTopAppBar(
                    title = { Text(text = widget.title) },
                    modifier = widget.modifiers.toCompose(),
                    colors = colors,
                    navigationIcon = navIcon
                )
                AppBarStyle.MEDIUM -> MediumTopAppBar(
                    title = { Text(text = widget.title) },
                    modifier = widget.modifiers.toCompose(),
                    colors = colors,
                    navigationIcon = navIcon
                )
                AppBarStyle.LARGE -> LargeTopAppBar(
                    title = { Text(text = widget.title) },
                    modifier = widget.modifiers.toCompose(),
                    colors = colors,
                    navigationIcon = navIcon
                )
            }
        }

        is AndroidUiNode.Column -> {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                modifier = widget.modifiers.toCompose()
            ) {
                widget.children.forEach { PadaukRenderer(it) }
            }
        }

        is AndroidUiNode.Row -> {
            Row(modifier = widget.modifiers.toCompose()) {
                widget.children.forEach { PadaukRenderer(it) }
            }
        }

        is AndroidUiNode.Stack -> {
            Box(modifier = widget.modifiers.toCompose()) {
                widget.children.forEach { PadaukRenderer(it) }
            }
        }

        is AndroidUiNode.Text -> {
            Text(
                text = widget.text,
                fontSize = widget.spSize.sp,
                modifier = widget.modifiers.toCompose()
            )
        }

        is AndroidUiNode.Button -> {
            Button(
                modifier = widget.modifiers.toCompose(),
                onClick = {
                    Log.d("Padauk", "Button click: ${widget.actionId}")
                    padaukDispatchAction(widget.actionId)
                }) {
                PadaukRenderer(widget.content.first())
            }
        }

        is AndroidUiNode.Image -> {
            PadaukImage(
                source = widget.source,
                fit = widget.fit,
                modifier = widget.modifiers.toCompose()
            )
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

private fun extractBackActionId(node: AndroidUiNode): String? {
    if (node is AndroidUiNode.Button) {
        val first = node.content.firstOrNull()
        if (first is AndroidUiNode.Text && first.text == "<") {
            return node.actionId
        }
    }
    return null
}
