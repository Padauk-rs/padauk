package rs.padauk.core

import androidx.compose.runtime.Composable
import rs.padauk.core.renderer.*

@Composable
fun PadaukRenderer(widget: AndroidUiNode) {
    when (widget) {
        is AndroidUiNode.Scaffold -> renderScaffold(widget)
        is AndroidUiNode.AppBar -> renderAppBar(widget)
        is AndroidUiNode.NavigationBar -> renderNavigationBar(widget)
        is AndroidUiNode.NavigationDrawer -> renderNavigationDrawer(widget)
        is AndroidUiNode.Column -> renderColumn(widget)
        is AndroidUiNode.Row -> renderRow(widget)
        is AndroidUiNode.Stack -> renderStack(widget)
        is AndroidUiNode.Dialog -> renderDialog(widget)
        is AndroidUiNode.FullscreenDialog -> renderFullscreenDialog(widget)
        is AndroidUiNode.DatePickerDialog -> renderDatePickerDialog(widget)
        is AndroidUiNode.DateRangePickerDialog -> renderDateRangePickerDialog(widget)
        is AndroidUiNode.TimePickerDialog -> renderTimePickerDialog(widget)
        is AndroidUiNode.Scroll -> renderScroll(widget)
        is AndroidUiNode.ListView -> renderList(widget)
        is AndroidUiNode.ListItem -> renderListItem(widget)
        is AndroidUiNode.Divider -> renderDivider(widget)
        is AndroidUiNode.Text -> renderText(widget)
        is AndroidUiNode.TextField -> renderTextField(widget)
        is AndroidUiNode.Menu -> renderMenu(widget)
        is AndroidUiNode.DropdownField -> renderDropdownField(widget)
        is AndroidUiNode.Button -> renderButton(widget)
        is AndroidUiNode.IconButton -> renderIconButton(widget)
        is AndroidUiNode.Card -> renderCard(widget)
        is AndroidUiNode.Checkbox -> renderCheckbox(widget)
        is AndroidUiNode.Chip -> renderChip(widget)
        is AndroidUiNode.Fab -> renderFab(widget)
        is AndroidUiNode.Image -> renderImage(widget)
    }
}
