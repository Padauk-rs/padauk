package rs.padauk.core.renderer

import androidx.activity.compose.BackHandler
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
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
import androidx.compose.material3.ModalDrawerSheet
import androidx.compose.material3.ModalNavigationDrawer
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.NavigationBarItemDefaults
import androidx.compose.material3.NavigationDrawerItem
import androidx.compose.material3.NavigationDrawerItemDefaults
import androidx.compose.material3.NavigationRail
import androidx.compose.material3.NavigationRailItem
import androidx.compose.material3.NavigationRailItemDefaults
import androidx.compose.material3.PermanentDrawerSheet
import androidx.compose.material3.PermanentNavigationDrawer
import androidx.compose.material3.Scaffold
import androidx.compose.material3.ScrollableTabRow
import androidx.compose.material3.Tab
import androidx.compose.material3.TabRow
import androidx.compose.material3.TabRowDefaults
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.material3.LeadingIconTab
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.TabRowDefaults.tabIndicatorOffset
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import rs.padauk.core.AndroidUiNode
import rs.padauk.core.AppBarStyle
import rs.padauk.core.NavigationDrawerType
import rs.padauk.core.PadaukRenderer
import rs.padauk.core.TabsStyle
import rs.padauk.core.padaukDispatchAction
import rs.padauk.core.widget.toCompose
import rs.padauk.core.widget.toComposeColor
import kotlinx.coroutines.launch
import androidx.compose.material3.DismissibleDrawerSheet
import androidx.compose.material3.DismissibleNavigationDrawer
import androidx.compose.material3.DrawerValue
import androidx.compose.material3.rememberDrawerState

@Composable
internal fun renderScaffold(widget: AndroidUiNode.Scaffold) {
    val drawerNode = widget.drawer.firstOrNull()
    if (drawerNode is AndroidUiNode.NavigationDrawer) {
        val drawerState = rememberDrawerState(initialValue = DrawerValue.Closed)
        val scope = rememberCoroutineScope()
        val toggleDrawer: (() -> Unit)? = if (drawerNode.drawerType == NavigationDrawerType.PERMANENT) {
            null
        } else {
            {
                scope.launch {
                    if (drawerState.isClosed) {
                        drawerState.open()
                    } else {
                        drawerState.close()
                    }
                }
            }
        }

        BackHandler(enabled = drawerState.isOpen) {
            scope.launch { drawerState.close() }
        }

        when (drawerNode.drawerType) {
            NavigationDrawerType.MODAL -> ModalNavigationDrawer(
                drawerState = drawerState,
                gesturesEnabled = drawerNode.options.gesturesEnabled,
                drawerContent = { renderNavigationDrawerSheet(drawerNode) }
            ) {
                renderScaffoldContent(widget, toggleDrawer)
            }

            NavigationDrawerType.DISMISSIBLE -> DismissibleNavigationDrawer(
                drawerState = drawerState,
                gesturesEnabled = drawerNode.options.gesturesEnabled,
                drawerContent = { renderNavigationDrawerSheet(drawerNode, dismissible = true) }
            ) {
                renderScaffoldContent(widget, toggleDrawer)
            }

            NavigationDrawerType.PERMANENT -> PermanentNavigationDrawer(
                drawerContent = { renderNavigationDrawerSheet(drawerNode, permanent = true) }
            ) {
                renderScaffoldContent(widget, null)
            }
        }
    } else {
        renderScaffoldContent(widget, null)
    }
}

@Composable
private fun renderScaffoldContent(
    widget: AndroidUiNode.Scaffold,
    openDrawer: (() -> Unit)? = null
) {
    val railNode = widget.rail.firstOrNull() as? AndroidUiNode.NavigationRail
    var railExpanded by remember(railNode?.options?.expanded) {
        mutableStateOf(railNode?.options?.expanded ?: false)
    }
    val railToggle: (() -> Unit)? = if (railNode?.options?.allowToggle == true) {
        { railExpanded = !railExpanded }
    } else {
        null
    }
    val topBarAction = openDrawer ?: railToggle

    val scaffoldContent: @Composable () -> Unit = {
        Scaffold(
            modifier = widget.modifiers.toCompose(),
            topBar = {
                if (widget.appBar.isNotEmpty()) {
                    val top = widget.appBar.first()
                    if (top is AndroidUiNode.AppBar) {
                        renderAppBar(top, topBarAction)
                    } else {
                        PadaukRenderer(top)
                    }
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
            Box(modifier = Modifier.padding(innerPadding)) {
                if (widget.body.isNotEmpty()) {
                    PadaukRenderer(widget.body.first())
                }
            }
        }
    }

    if (railNode != null) {
        Row(modifier = Modifier.fillMaxWidth()) {
            val rowScope = this
            renderNavigationRail(railNode, railExpanded)
            Box(modifier = with(rowScope) { Modifier.weight(1f) }) {
                scaffoldContent()
            }
        }
    } else {
        scaffoldContent()
    }
}

@Composable
internal fun renderNavigationRail(widget: AndroidUiNode.NavigationRail, expanded: Boolean) {
    if (expanded) {
        PermanentDrawerSheet(
            modifier = Modifier.fillMaxHeight().then(widget.modifiers.toCompose()),
            drawerContainerColor = widget.options.containerColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.surfaceContainer,
            drawerContentColor = widget.options.contentColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.onSurface
        ) {
            widget.destinations.forEach { destination ->
                NavigationDrawerItem(
                    label = { Text(destination.label) },
                    selected = destination.selected,
                    onClick = { padaukDispatchAction(destination.actionId) },
                    icon = {
                        Icon(
                            imageVector = iconVector(destination.icon),
                            contentDescription = destination.label
                        )
                    },
                    colors = NavigationDrawerItemDefaults.colors(
                        selectedContainerColor = widget.options.indicatorColor?.toComposeColor()
                            ?: MaterialTheme.colorScheme.secondaryContainer,
                        selectedIconColor = widget.options.selectedIconColor?.toComposeColor()
                            ?: MaterialTheme.colorScheme.onSecondaryContainer,
                        selectedTextColor = widget.options.selectedTextColor?.toComposeColor()
                            ?: MaterialTheme.colorScheme.onSecondaryContainer,
                        unselectedIconColor = widget.options.unselectedIconColor?.toComposeColor()
                            ?: MaterialTheme.colorScheme.onSurfaceVariant,
                        unselectedTextColor = widget.options.unselectedTextColor?.toComposeColor()
                            ?: MaterialTheme.colorScheme.onSurfaceVariant
                    ),
                    modifier = Modifier.padding(horizontal = 12.dp, vertical = 4.dp)
                )
            }
        }
    } else {
        NavigationRail(
            modifier = Modifier.fillMaxHeight().then(widget.modifiers.toCompose()),
            containerColor = widget.options.containerColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.surfaceContainer,
            contentColor = widget.options.contentColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.onSurface
        ) {
            widget.destinations.forEach { destination ->
                NavigationRailItem(
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
                    colors = NavigationRailItemDefaults.colors(
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
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun renderAppBar(widget: AndroidUiNode.AppBar, onMenuClick: (() -> Unit)? = null) {
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
        } else if (onMenuClick != null) {
            IconButton(onClick = onMenuClick) {
                Icon(
                    imageVector = iconVector(rs.padauk.core.IconType.MENU),
                    contentDescription = "Menu"
                )
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
internal fun renderNavigationDrawer(widget: AndroidUiNode.NavigationDrawer) {
    // Rendering this node directly is supported, but the preferred integration is Scaffold.drawer(...)
    renderNavigationDrawerSheet(widget)
}

@Composable
private fun renderNavigationDrawerSheet(
    widget: AndroidUiNode.NavigationDrawer,
    dismissible: Boolean = false,
    permanent: Boolean = false
) {
    val drawerContent: @Composable () -> Unit = {
        if (!widget.title.isNullOrEmpty()) {
            Text(
                text = widget.title,
                modifier = Modifier.padding(start = 28.dp, top = 28.dp, end = 28.dp, bottom = 12.dp),
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
        widget.destinations.forEach { destination ->
            NavigationDrawerItem(
                label = { Text(destination.label) },
                selected = destination.selected,
                onClick = { padaukDispatchAction(destination.actionId) },
                icon = { Icon(imageVector = iconVector(destination.icon), contentDescription = destination.label) },
                badge = destination.badge?.let { { Text(it) } },
                colors = NavigationDrawerItemDefaults.colors(
                    selectedContainerColor = widget.options.indicatorColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.secondaryContainer,
                    selectedIconColor = widget.options.selectedIconColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.onSecondaryContainer,
                    selectedTextColor = widget.options.selectedTextColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.onSecondaryContainer,
                    unselectedIconColor = widget.options.unselectedIconColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.onSurfaceVariant,
                    unselectedTextColor = widget.options.unselectedTextColor?.toComposeColor()
                        ?: MaterialTheme.colorScheme.onSurfaceVariant
                ),
                modifier = Modifier.padding(horizontal = 12.dp, vertical = 4.dp)
            )
        }
    }

    when {
        permanent -> PermanentDrawerSheet(
            modifier = widget.modifiers.toCompose(),
            drawerContainerColor = widget.options.containerColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.surfaceContainerLow,
            drawerContentColor = widget.options.contentColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.onSurfaceVariant
        ) {
            drawerContent()
        }

        dismissible -> DismissibleDrawerSheet(
            modifier = widget.modifiers.toCompose(),
            drawerContainerColor = widget.options.containerColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.surfaceContainerLow,
            drawerContentColor = widget.options.contentColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.onSurfaceVariant
        ) {
            drawerContent()
        }

        else -> ModalDrawerSheet(
            modifier = widget.modifiers.toCompose(),
            drawerContainerColor = widget.options.containerColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.surfaceContainerLow,
            drawerContentColor = widget.options.contentColor?.toComposeColor()
                ?: MaterialTheme.colorScheme.onSurfaceVariant
        ) {
            drawerContent()
        }
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
internal fun renderTabs(widget: AndroidUiNode.Tabs) {
    if (widget.destinations.isEmpty()) return

    val selectedIndex = widget.destinations.indexOfFirst { it.selected }.let { index ->
        if (index >= 0) index else 0
    }
    val isPrimary = widget.options.style == TabsStyle.PRIMARY
    val containerColor = widget.options.containerColor?.toComposeColor()
        ?: if (isPrimary) MaterialTheme.colorScheme.surfaceContainer else MaterialTheme.colorScheme.surface
    val contentColor = widget.options.contentColor?.toComposeColor()
        ?: MaterialTheme.colorScheme.onSurface
    val indicatorColor = widget.options.indicatorColor?.toComposeColor()
        ?: if (isPrimary) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.onSurface
    val selectedContentColor = widget.options.selectedContentColor?.toComposeColor()
        ?: if (isPrimary) MaterialTheme.colorScheme.primary else MaterialTheme.colorScheme.onSurface
    val unselectedContentColor = widget.options.unselectedContentColor?.toComposeColor()
        ?: MaterialTheme.colorScheme.onSurfaceVariant
    val dividerColor = widget.options.dividerColor?.toComposeColor()
        ?: MaterialTheme.colorScheme.outlineVariant

    val indicator: @Composable (tabPositions: List<androidx.compose.material3.TabPosition>) -> Unit = {
        tabPositions ->
        if (tabPositions.isNotEmpty()) {
            TabRowDefaults.Indicator(
                modifier = Modifier.tabIndicatorOffset(
                    tabPositions[selectedIndex.coerceIn(0, tabPositions.lastIndex)]
                ),
                color = indicatorColor
            )
        }
    }

    val divider: @Composable () -> Unit = {
        HorizontalDivider(color = dividerColor)
    }

    val tabsContent: @Composable () -> Unit = {
        widget.destinations.forEach { destination ->
            val icon = destination.icon
            if (icon != null) {
                LeadingIconTab(
                    selected = destination.selected,
                    onClick = { padaukDispatchAction(destination.actionId) },
                    text = { Text(destination.label) },
                    icon = {
                        Icon(
                            imageVector = iconVector(icon),
                            contentDescription = destination.label
                        )
                    },
                    selectedContentColor = selectedContentColor,
                    unselectedContentColor = unselectedContentColor
                )
            } else {
                Tab(
                    selected = destination.selected,
                    onClick = { padaukDispatchAction(destination.actionId) },
                    text = { Text(destination.label) },
                    selectedContentColor = selectedContentColor,
                    unselectedContentColor = unselectedContentColor
                )
            }
        }
    }

    if (widget.options.scrollable) {
        ScrollableTabRow(
            selectedTabIndex = selectedIndex,
            modifier = widget.modifiers.toCompose(),
            containerColor = containerColor,
            contentColor = contentColor,
            indicator = indicator,
            divider = divider,
            tabs = tabsContent
        )
    } else {
        TabRow(
            selectedTabIndex = selectedIndex,
            modifier = widget.modifiers.toCompose(),
            containerColor = containerColor,
            contentColor = contentColor,
            indicator = indicator,
            divider = divider,
            tabs = tabsContent
        )
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
