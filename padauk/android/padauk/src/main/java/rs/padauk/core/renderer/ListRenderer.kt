package rs.padauk.core.renderer

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.ListItem
import androidx.compose.material3.ListItemDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.VerticalDivider
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import rs.padauk.core.AndroidUiNode
import rs.padauk.core.PadaukRenderer
import rs.padauk.core.padaukDispatchAction
import rs.padauk.core.widget.toCompose
import rs.padauk.core.widget.toComposeColor

@Composable
internal fun renderList(widget: AndroidUiNode.ListView) {
    val scrollState = rememberScrollState()
    Column(modifier = widget.modifiers.toCompose().verticalScroll(scrollState)) {
        widget.items.forEach { item ->
            PadaukRenderer(item)
        }
    }
}

@Composable
internal fun renderListItem(widget: AndroidUiNode.ListItem) {
    val listModifier = widget.actionId?.let { actionId ->
        widget.modifiers
            .toCompose()
            .clickable(enabled = widget.options.enabled) { padaukDispatchAction(actionId) }
    } ?: widget.modifiers.toCompose()

    val trailingContent: (@Composable () -> Unit)? =
        if (widget.trailing.text != null || widget.trailing.icon != null) {
            {
                if (widget.trailing.text != null && widget.trailing.icon != null) {
                    Row(
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Text(
                            text = widget.trailing.text!!,
                            color = widget.options.trailingColor?.toComposeColor()
                                ?: MaterialTheme.colorScheme.onSurfaceVariant,
                            style = if (widget.options.trailingSupportingText) {
                                MaterialTheme.typography.bodyMedium
                            } else {
                                MaterialTheme.typography.labelSmall
                            },
                        )
                        Icon(
                            imageVector = iconVector(widget.trailing.icon!!),
                            contentDescription = null,
                            tint = widget.options.trailingColor?.toComposeColor()
                                ?: MaterialTheme.colorScheme.onSurfaceVariant,
                            modifier = Modifier.size(20.dp),
                        )
                    }
                } else if (widget.trailing.text != null) {
                    Text(
                        text = widget.trailing.text!!,
                        color = widget.options.trailingColor?.toComposeColor()
                            ?: MaterialTheme.colorScheme.onSurfaceVariant,
                        style = if (widget.options.trailingSupportingText) {
                            MaterialTheme.typography.bodyMedium
                        } else {
                            MaterialTheme.typography.labelSmall
                        },
                    )
                } else {
                    Icon(
                        imageVector = iconVector(widget.trailing.icon!!),
                        contentDescription = null,
                        tint = widget.options.trailingColor?.toComposeColor()
                            ?: MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.size(20.dp),
                    )
                }
            }
        } else {
            null
        }

    ListItem(
        headlineContent = {
            Text(
                text = widget.headline,
                color = widget.options.headlineColor?.toComposeColor()
                    ?: MaterialTheme.colorScheme.onSurface,
            )
        },
        modifier = listModifier,
        leadingContent = widget.leadingIcon?.let { icon ->
            {
                Icon(
                    imageVector = iconVector(icon),
                    contentDescription = null,
                    tint = widget.options.leadingColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        },
        supportingContent = widget.supportingText?.let { value ->
            {
                Text(
                    text = value,
                    color = widget.options.supportingColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        },
        overlineContent = widget.overlineText?.let { value ->
            {
                Text(
                    text = value,
                    color = widget.options.overlineColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        },
        trailingContent = trailingContent,
        tonalElevation = widget.options.tonalElevation?.dp ?: 0.dp,
        colors = ListItemDefaults.colors(
            containerColor = widget.options.containerColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.surface,
            headlineColor = widget.options.headlineColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.onSurface,
            leadingIconColor = widget.options.leadingColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.onSurfaceVariant,
            overlineColor = widget.options.overlineColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.onSurfaceVariant,
            supportingColor = widget.options.supportingColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.onSurfaceVariant,
            trailingIconColor = widget.options.trailingColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.onSurfaceVariant,
        ),
    )
}

@Composable
internal fun renderDivider(widget: AndroidUiNode.Divider) {
    val thickness = widget.options.thickness?.dp ?: 1.dp
    val start = widget.options.insetStart?.dp ?: 0.dp
    val end = widget.options.insetEnd?.dp ?: 0.dp
    val dividerModifier = widget.modifiers.toCompose().padding(start = start, end = end)

    if (widget.options.vertical) {
        VerticalDivider(
            modifier = dividerModifier,
            thickness = thickness,
            color = widget.options.color?.toComposeColor()
                ?: MaterialTheme.colorScheme.outlineVariant,
        )
    } else {
        HorizontalDivider(
            modifier = dividerModifier,
            thickness = thickness,
            color = widget.options.color?.toComposeColor()
                ?: MaterialTheme.colorScheme.outlineVariant,
        )
    }
}
