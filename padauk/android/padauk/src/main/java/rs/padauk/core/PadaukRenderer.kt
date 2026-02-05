package rs.padauk.core

import android.annotation.SuppressLint
import android.graphics.Color.*
import android.graphics.RenderNode
import android.util.Log
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Favorite
import androidx.compose.material.icons.filled.Menu
import androidx.compose.material.icons.filled.Person
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.Button
import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.FilledIconButton
import androidx.compose.material3.FilledTonalIconButton
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.OutlinedIconButton
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ExtendedFloatingActionButton
import androidx.compose.material3.FilledTonalButton
import androidx.compose.material3.ElevatedButton
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.TextButton
import androidx.compose.material3.Checkbox
import androidx.compose.material3.CheckboxDefaults
import androidx.compose.material3.AssistChip
import androidx.compose.material3.FilterChip
import androidx.compose.material3.InputChip
import androidx.compose.material3.SuggestionChip
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.SmallFloatingActionButton
import androidx.compose.material3.LargeFloatingActionButton
import androidx.compose.material3.LargeTopAppBar
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MediumTopAppBar
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.material3.Card
import androidx.compose.material3.ElevatedCard
import androidx.compose.material3.OutlinedCard
import androidx.compose.material3.CardDefaults
import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.AssistChipDefaults
import androidx.compose.material3.FilterChipDefaults
import androidx.compose.material3.InputChipDefaults
import androidx.compose.material3.SuggestionChipDefaults
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
import rs.padauk.core.widget.toComposeColor

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
            val onClick = {
                Log.d("Padauk", "Button click: ${widget.actionId}")
                padaukDispatchAction(widget.actionId)
            }
            when (widget.style) {
                ButtonStyle.FILLED -> Button(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { PadaukRenderer(widget.content.first()) }
                ButtonStyle.FILLED_TONAL -> FilledTonalButton(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { PadaukRenderer(widget.content.first()) }
                ButtonStyle.ELEVATED -> ElevatedButton(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { PadaukRenderer(widget.content.first()) }
                ButtonStyle.OUTLINED -> OutlinedButton(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { PadaukRenderer(widget.content.first()) }
                ButtonStyle.TEXT -> TextButton(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { PadaukRenderer(widget.content.first()) }
            }
        }

        is AndroidUiNode.IconButton -> {
            val onClick = {
                Log.d("Padauk", "Icon button click: ${widget.actionId}")
                padaukDispatchAction(widget.actionId)
            }
            val icon = iconVector(widget.icon)
            when (widget.style) {
                IconButtonStyle.STANDARD -> IconButton(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { Icon(icon, contentDescription = null) }
                IconButtonStyle.FILLED -> FilledIconButton(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { Icon(icon, contentDescription = null) }
                IconButtonStyle.FILLED_TONAL -> FilledTonalIconButton(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { Icon(icon, contentDescription = null) }
                IconButtonStyle.OUTLINED -> OutlinedIconButton(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { Icon(icon, contentDescription = null) }
            }
        }

        is AndroidUiNode.Card -> {
            val content: @Composable () -> Unit = {
                Column(
                    modifier = Modifier
                        .padding(16.dp)
                ) {
                    widget.children.forEach { PadaukRenderer(it) }
                }
            }
            val onClick = widget.actionId?.let { id ->
                {
                    Log.d("Padauk", "Card click: $id")
                    padaukDispatchAction(id)
                }
            }
            when (widget.style) {
                CardStyle.FILLED -> {
                    if (onClick != null) {
                        Card(
                            modifier = widget.modifiers.toCompose(),
                            colors = CardDefaults.cardColors(
                                containerColor = MaterialTheme.colorScheme.surfaceVariant
                            ),
                            onClick = onClick
                        ) { content() }
                    } else {
                        Card(
                            modifier = widget.modifiers.toCompose(),
                            colors = CardDefaults.cardColors(
                                containerColor = MaterialTheme.colorScheme.surfaceVariant
                            )
                        ) { content() }
                    }
                }
                CardStyle.ELEVATED -> {
                    if (onClick != null) {
                        ElevatedCard(
                            modifier = widget.modifiers.toCompose(),
                            onClick = onClick
                        ) { content() }
                    } else {
                        ElevatedCard(
                            modifier = widget.modifiers.toCompose()
                        ) { content() }
                    }
                }
                CardStyle.OUTLINED -> {
                    if (onClick != null) {
                        OutlinedCard(
                            modifier = widget.modifiers.toCompose(),
                            onClick = onClick
                        ) { content() }
                    } else {
                        OutlinedCard(
                            modifier = widget.modifiers.toCompose()
                        ) { content() }
                    }
                }
            }
        }

        is AndroidUiNode.Checkbox -> {
            val colors = CheckboxDefaults.colors(
                checkedColor = widget.colorChecked?.toComposeColor()
                    ?: MaterialTheme.colorScheme.primary,
                uncheckedColor = widget.colorUnchecked?.toComposeColor()
                    ?: MaterialTheme.colorScheme.onSurfaceVariant,
                checkmarkColor = widget.colorCheckmark?.toComposeColor()
                    ?: MaterialTheme.colorScheme.onPrimary
            )
            Checkbox(
                modifier = widget.modifiers.toCompose(),
                checked = widget.checked,
                onCheckedChange = {
                    Log.d("Padauk", "Checkbox toggle: ${widget.actionId}")
                    padaukDispatchAction(widget.actionId)
                },
                enabled = widget.enabled,
                colors = colors
            )
        }

        is AndroidUiNode.Chip -> {
            val onClick = {
                Log.d("Padauk", "Chip click: ${widget.actionId}")
                padaukDispatchAction(widget.actionId)
            }
            val leading: (@Composable () -> Unit)? = widget.leadingIcon?.let { icon ->
                { Icon(iconVector(icon), contentDescription = null) }
            }
            val trailing: (@Composable () -> Unit)? = widget.trailingIcon?.let { icon ->
                {
                    val closeId = widget.closeActionId
                    val base = Modifier.size(18.dp)
                    val modifier = if (closeId != null) {
                        base.clickable { padaukDispatchAction(closeId) }
                    } else {
                        base
                    }
                    Icon(
                        imageVector = iconVector(icon),
                        contentDescription = null,
                        modifier = modifier
                    )
                }
            }

            val shape = when (widget.options.shape) {
                ChipShape.DEFAULT -> null
                ChipShape.PILL -> RoundedCornerShape(50)
            }

            val border = if (widget.options.borderColor != null && widget.options.borderWidth != null) {
                BorderStroke(
                    widget.options.borderWidth!!.dp,
                    widget.options.borderColor!!.toComposeColor()
                )
            } else {
                null
            }

            val fallbackContainer = MaterialTheme.colorScheme.surfaceVariant
            val fallbackLabel = MaterialTheme.colorScheme.onSurfaceVariant
            val fallbackIcon = MaterialTheme.colorScheme.onSurfaceVariant

            when (widget.style) {
                ChipStyle.ASSIST -> AssistChip(
                    onClick = onClick,
                    label = { Text(widget.label) },
                    leadingIcon = leading,
                    trailingIcon = trailing,
                    enabled = widget.options.enabled,
                    shape = shape ?: AssistChipDefaults.shape,
                    colors = AssistChipDefaults.assistChipColors(
                        containerColor = widget.options.containerColor?.toComposeColor()
                            ?: fallbackContainer,
                        labelColor = widget.options.labelColor?.toComposeColor()
                            ?: fallbackLabel,
                        leadingIconContentColor = widget.options.iconColor?.toComposeColor()
                            ?: fallbackIcon,
                        trailingIconContentColor = widget.options.iconColor?.toComposeColor()
                            ?: fallbackIcon
                    ),
                    elevation = widget.options.elevation?.let {
                        AssistChipDefaults.assistChipElevation(elevation = it.dp)
                    },
                    border = border,
                    modifier = widget.modifiers.toCompose()
                )
                ChipStyle.FILTER -> FilterChip(
                    selected = widget.selected,
                    onClick = onClick,
                    label = { Text(widget.label) },
                    leadingIcon = leading,
                    trailingIcon = trailing,
                    enabled = widget.options.enabled,
                    shape = shape ?: FilterChipDefaults.shape,
                    colors = FilterChipDefaults.filterChipColors(
                        containerColor = widget.options.containerColor?.toComposeColor()
                            ?: fallbackContainer,
                        labelColor = widget.options.labelColor?.toComposeColor()
                            ?: fallbackLabel,
                        iconColor = widget.options.iconColor?.toComposeColor()
                            ?: fallbackIcon,
                    ),
                    elevation = widget.options.elevation?.let {
                        FilterChipDefaults.filterChipElevation(elevation = it.dp)
                    },
                    border = border,
                    modifier = widget.modifiers.toCompose()
                )
                ChipStyle.INPUT -> InputChip(
                    selected = widget.selected,
                    onClick = onClick,
                    label = { Text(widget.label) },
                    leadingIcon = leading,
                    trailingIcon = trailing,
                    enabled = widget.options.enabled,
                    shape = shape ?: InputChipDefaults.shape,
                    colors = InputChipDefaults.inputChipColors(
                        containerColor = widget.options.containerColor?.toComposeColor()
                            ?: fallbackContainer,
                        labelColor = widget.options.labelColor?.toComposeColor()
                            ?: fallbackLabel,
                        leadingIconColor = widget.options.iconColor?.toComposeColor()
                            ?: fallbackIcon,
                        trailingIconColor = widget.options.iconColor?.toComposeColor()
                            ?: fallbackIcon
                    ),
                    elevation = widget.options.elevation?.let {
                        InputChipDefaults.inputChipElevation(elevation = it.dp)
                    },
                    border = border,
                    modifier = widget.modifiers.toCompose()
                )
                ChipStyle.SUGGESTION -> SuggestionChip(
                    onClick = onClick,
                    label = { Text(widget.label) },
                    icon = leading,
                    enabled = widget.options.enabled,
                    shape = shape ?: SuggestionChipDefaults.shape,
                    colors = SuggestionChipDefaults.suggestionChipColors(
                        containerColor = widget.options.containerColor?.toComposeColor()
                            ?: fallbackContainer,
                        labelColor = widget.options.labelColor?.toComposeColor()
                            ?: fallbackLabel,
                        iconContentColor = widget.options.iconColor?.toComposeColor()
                            ?: fallbackIcon
                    ),
                    elevation = widget.options.elevation?.let {
                        SuggestionChipDefaults.suggestionChipElevation(elevation = it.dp)
                    },
                    border = border,
                    modifier = widget.modifiers.toCompose()
                )
            }
        }

        is AndroidUiNode.Fab -> {
            val onClick = {
                Log.d("Padauk", "FAB click: ${widget.actionId}")
                padaukDispatchAction(widget.actionId)
            }
            val icon = iconVector(widget.icon)
            when (widget.style) {
                FabStyle.SMALL -> SmallFloatingActionButton(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { Icon(icon, contentDescription = null) }
                FabStyle.NORMAL -> FloatingActionButton(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { Icon(icon, contentDescription = null) }
                FabStyle.LARGE -> LargeFloatingActionButton(
                    modifier = widget.modifiers.toCompose(),
                    onClick = onClick
                ) { Icon(icon, contentDescription = null) }
                FabStyle.EXTENDED -> {
                    val label = widget.label ?: ""
                    ExtendedFloatingActionButton(
                        modifier = widget.modifiers.toCompose(),
                        onClick = onClick,
                        icon = { Icon(icon, contentDescription = null) },
                        text = { Text(label) }
                    )
                }
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

private fun iconVector(icon: IconType) = when (icon) {
    IconType.ADD -> Icons.Filled.Add
    IconType.CLOSE -> Icons.Filled.Close
    IconType.MENU -> Icons.Filled.Menu
    IconType.FAVORITE -> Icons.Filled.Favorite
    IconType.SEARCH -> Icons.Filled.Search
    IconType.PERSON -> Icons.Filled.Person
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
