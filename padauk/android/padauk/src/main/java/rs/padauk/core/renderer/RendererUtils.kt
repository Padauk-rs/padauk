package rs.padauk.core.renderer

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Favorite
import androidx.compose.material.icons.filled.Menu
import androidx.compose.material.icons.filled.Person
import androidx.compose.material.icons.filled.Search
import rs.padauk.core.AndroidUiNode
import rs.padauk.core.IconType
import rs.padauk.core.Modifiers
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale

internal fun AndroidUiNode.modifiersOrNull(): Modifiers? {
    return when (this) {
        is AndroidUiNode.Column -> this.modifiers
        is AndroidUiNode.Row -> this.modifiers
        is AndroidUiNode.Stack -> this.modifiers
        is AndroidUiNode.Dialog -> this.modifiers
        is AndroidUiNode.FullscreenDialog -> this.modifiers
        is AndroidUiNode.DatePickerDialog -> this.modifiers
        is AndroidUiNode.DateRangePickerDialog -> this.modifiers
        is AndroidUiNode.TimePickerDialog -> this.modifiers
        is AndroidUiNode.Scroll -> this.modifiers
        is AndroidUiNode.ListView -> this.modifiers
        is AndroidUiNode.ListItem -> this.modifiers
        is AndroidUiNode.Divider -> this.modifiers
        is AndroidUiNode.Scaffold -> this.modifiers
        is AndroidUiNode.AppBar -> this.modifiers
        is AndroidUiNode.Text -> this.modifiers
        is AndroidUiNode.TextField -> this.modifiers
        is AndroidUiNode.Button -> this.modifiers
        is AndroidUiNode.IconButton -> this.modifiers
        is AndroidUiNode.Card -> this.modifiers
        is AndroidUiNode.Checkbox -> this.modifiers
        is AndroidUiNode.Chip -> this.modifiers
        is AndroidUiNode.Fab -> this.modifiers
        is AndroidUiNode.Image -> this.modifiers
    }
}

internal fun iconVector(icon: IconType) = when (icon) {
    IconType.ADD -> Icons.Filled.Add
    IconType.CLOSE -> Icons.Filled.Close
    IconType.MENU -> Icons.Filled.Menu
    IconType.FAVORITE -> Icons.Filled.Favorite
    IconType.SEARCH -> Icons.Filled.Search
    IconType.PERSON -> Icons.Filled.Person
}

internal fun extractBackActionId(node: AndroidUiNode): String? {
    if (node is AndroidUiNode.Button) {
        val first = node.content.firstOrNull()
        if (first is AndroidUiNode.Text && first.text == "<") {
            return node.actionId
        }
    }
    return null
}

internal fun formatDateRangeHeadline(startMillis: Long?, endMillis: Long?): String {
    val start = startMillis?.let(::formatShortDate) ?: "Start"
    val end = endMillis?.let(::formatShortDate) ?: "End"
    return "$start - $end"
}

private fun formatShortDate(millis: Long): String {
    return SimpleDateFormat("MMM d", Locale.getDefault()).format(Date(millis))
}
