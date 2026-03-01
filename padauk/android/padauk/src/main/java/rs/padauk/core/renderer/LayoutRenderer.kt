package rs.padauk.core.renderer

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.LargeTopAppBar
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.MediumTopAppBar
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.NavigationBarItemDefaults
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import rs.padauk.core.AndroidUiNode
import rs.padauk.core.AppBarStyle
import rs.padauk.core.PadaukRenderer
import rs.padauk.core.padaukDispatchAction
import rs.padauk.core.widget.toCompose
import rs.padauk.core.widget.toComposeColor

@Composable
internal fun renderScaffold(widget: AndroidUiNode.Scaffold) {
    Scaffold(
        modifier = widget.modifiers.toCompose(),
        topBar = {
            // Check if the vector has items
            if (widget.appBar.isNotEmpty()) {
                PadaukRenderer(widget.appBar.first())
            }
        },
        bottomBar = {
            if (widget.bottomBar.isNotEmpty()) {
                PadaukRenderer(widget.bottomBar.first())
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

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun renderAppBar(widget: AndroidUiNode.AppBar) {
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
        containerColor = widget.options.containerColor?.toComposeColor()
            ?: MaterialTheme.colorScheme.primaryContainer,
        titleContentColor = widget.options.titleColor?.toComposeColor()
            ?: MaterialTheme.colorScheme.onPrimaryContainer,
        navigationIconContentColor = widget.options.navIconColor?.toComposeColor()
            ?: MaterialTheme.colorScheme.onPrimaryContainer
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

@Composable
internal fun renderNavigationBar(widget: AndroidUiNode.NavigationBar) {
    NavigationBar(
        modifier = widget.modifiers.toCompose(),
        containerColor = widget.options.containerColor?.toComposeColor()
            ?: MaterialTheme.colorScheme.surfaceContainer,
        contentColor = widget.options.contentColor?.toComposeColor()
            ?: MaterialTheme.colorScheme.onSurface
    ) {
        widget.destinations.forEach { destination ->
            NavigationBarItem(
                selected = destination.selected,
                onClick = { padaukDispatchAction(destination.actionId) },
                icon = {
                    Icon(
                        imageVector = iconVector(destination.icon),
                        contentDescription = destination.label
                    )
                },
                label = { Text(destination.label) },
                alwaysShowLabel = widget.options.alwaysShowLabel,
                colors = NavigationBarItemDefaults.colors(
                    selectedIconColor = widget.options.selectedIconColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.onSecondaryContainer,
                    selectedTextColor = widget.options.selectedTextColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.onSurface,
                    indicatorColor = widget.options.indicatorColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.secondaryContainer,
                    unselectedIconColor = widget.options.unselectedIconColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.onSurfaceVariant,
                    unselectedTextColor = widget.options.unselectedTextColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.onSurfaceVariant
                )
            )
        }
    }
}

@Composable
internal fun renderColumn(widget: AndroidUiNode.Column) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
        modifier = widget.modifiers.toCompose()
    ) {
        val columnScope = this
        widget.children.forEach { child ->
            val childModifiers = child.modifiersOrNull()
            val weight = childModifiers?.weight
            if (weight != null) {
                val fill = childModifiers.weightFill ?: true
                Box(modifier = with(columnScope) { Modifier.weight(weight, fill) }) {
                    PadaukRenderer(child)
                }
            } else {
                PadaukRenderer(child)
            }
        }
    }
}

@Composable
internal fun renderRow(widget: AndroidUiNode.Row) {
    Row(modifier = widget.modifiers.toCompose()) {
        val rowScope = this
        widget.children.forEach { child ->
            val childModifiers = child.modifiersOrNull()
            val weight = childModifiers?.weight
            if (weight != null) {
                val fill = childModifiers.weightFill ?: true
                Box(modifier = with(rowScope) { Modifier.weight(weight, fill) }) {
                    PadaukRenderer(child)
                }
            } else {
                PadaukRenderer(child)
            }
        }
    }
}

@Composable
internal fun renderStack(widget: AndroidUiNode.Stack) {
    Box(modifier = widget.modifiers.toCompose()) {
        widget.children.forEach { PadaukRenderer(it) }
    }
}

@Composable
internal fun renderScroll(widget: AndroidUiNode.Scroll) {
    val child = widget.child.firstOrNull()
    val scroll = rememberScrollState()
    Box(
        modifier = widget.modifiers.toCompose().verticalScroll(scroll)
    ) {
        if (child != null) {
            PadaukRenderer(child)
        }
    }
}
