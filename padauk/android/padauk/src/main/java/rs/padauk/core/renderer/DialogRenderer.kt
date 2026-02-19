package rs.padauk.core.renderer

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.DatePicker
import androidx.compose.material3.DatePickerDialog
import androidx.compose.material3.DateRangePicker
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.material3.TimeInput
import androidx.compose.material3.TimePicker
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.rememberDatePickerState
import androidx.compose.material3.rememberDateRangePickerState
import androidx.compose.material3.rememberTimePickerState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import rs.padauk.core.AndroidUiNode
import rs.padauk.core.PadaukRenderer
import rs.padauk.core.padaukDispatchAction
import rs.padauk.core.padaukDispatchActionWithString
import rs.padauk.core.widget.toCompose
import java.util.Calendar

@Composable
internal fun renderDialog(widget: AndroidUiNode.Dialog) {
    AlertDialog(
        onDismissRequest = {
            if (widget.dismissible) {
                widget.dismissActionId?.let { padaukDispatchAction(it) }
            }
        },
        title = widget.title?.let { { Text(text = it) } },
        text = { Text(text = widget.text) },
        confirmButton = {
            TextButton(onClick = { padaukDispatchAction(widget.confirmActionId) }) {
                Text(widget.confirmLabel)
            }
        },
        dismissButton = if (widget.dismissLabel != null && widget.dismissActionId != null) {
            {
                TextButton(onClick = { padaukDispatchAction(widget.dismissActionId) }) {
                    Text(widget.dismissLabel)
                }
            }
        } else {
            null
        },
        modifier = widget.modifiers.toCompose(),
    )
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun renderFullscreenDialog(widget: AndroidUiNode.FullscreenDialog) {
    Dialog(
        onDismissRequest = {
            if (widget.dismissible) {
                padaukDispatchAction(widget.dismissActionId)
            }
        },
        properties = DialogProperties(usePlatformDefaultWidth = false),
    ) {
        Surface(modifier = Modifier.fillMaxSize()) {
            Scaffold(
                topBar = {
                    TopAppBar(
                        title = { Text(widget.title) },
                        navigationIcon = {
                            IconButton(onClick = { padaukDispatchAction(widget.dismissActionId) }) {
                                Icon(
                                    imageVector = Icons.Filled.Close,
                                    contentDescription = widget.dismissLabel,
                                )
                            }
                        },
                        actions = {
                            if (widget.confirmLabel != null && widget.confirmActionId != null) {
                                TextButton(onClick = { padaukDispatchAction(widget.confirmActionId) }) {
                                    Text(widget.confirmLabel)
                                }
                            }
                        },
                    )
                },
                modifier = widget.modifiers.toCompose(),
            ) { innerPadding ->
                Box(modifier = Modifier.padding(innerPadding)) {
                    widget.content.firstOrNull()?.let { PadaukRenderer(it) }
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun renderDatePickerDialog(widget: AndroidUiNode.DatePickerDialog) {
    val state = rememberDatePickerState(
        initialSelectedDateMillis = widget.initialSelectedMillis
    )
    DatePickerDialog(
        onDismissRequest = {
            if (widget.dismissible) {
                widget.dismissActionId?.let { padaukDispatchAction(it) }
            }
        },
        confirmButton = {
            TextButton(onClick = {
                val millis = state.selectedDateMillis
                if (millis != null) {
                    padaukDispatchActionWithString(widget.confirmActionId, millis.toString())
                }
            }) {
                Text(widget.confirmLabel)
            }
        },
        dismissButton = if (widget.dismissLabel != null && widget.dismissActionId != null) {
            {
                TextButton(onClick = { padaukDispatchAction(widget.dismissActionId) }) {
                    Text(widget.dismissLabel)
                }
            }
        } else {
            null
        },
    ) {
        DatePicker(
            state = state,
            showModeToggle = widget.showModeToggle,
        )
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun renderDateRangePickerDialog(widget: AndroidUiNode.DateRangePickerDialog) {
    val state = rememberDateRangePickerState(
        initialSelectedStartDateMillis = widget.initialStartMillis,
        initialSelectedEndDateMillis = widget.initialEndMillis,
    )
    Dialog(
        onDismissRequest = {
            if (widget.dismissible) {
                widget.dismissActionId?.let { padaukDispatchAction(it) }
            }
        },
        properties = DialogProperties(usePlatformDefaultWidth = false),
    ) {
        Surface(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp),
            shape = MaterialTheme.shapes.extraLarge,
            tonalElevation = 6.dp,
        ) {
            Column(modifier = Modifier.fillMaxWidth().padding(16.dp)) {
                DateRangePicker(
                    state = state,
                    title = widget.title?.let {
                        {
                            Text(
                                text = it,
                                maxLines = 1,
                                overflow = TextOverflow.Ellipsis,
                                style = MaterialTheme.typography.labelLarge,
                            )
                        }
                    },
                    headline = {
                        Text(
                            text = formatDateRangeHeadline(
                                state.selectedStartDateMillis,
                                state.selectedEndDateMillis,
                            ),
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis,
                            fontSize = 16.sp,
                        )
                    },
                    showModeToggle = widget.showModeToggle,
                    modifier = Modifier.fillMaxWidth().heightIn(max = 420.dp),
                )
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.End,
                ) {
                    if (widget.dismissLabel != null && widget.dismissActionId != null) {
                        TextButton(onClick = { padaukDispatchAction(widget.dismissActionId) }) {
                            Text(widget.dismissLabel)
                        }
                    }
                    TextButton(onClick = {
                        val start = state.selectedStartDateMillis
                        val end = state.selectedEndDateMillis
                        val payload = "${start ?: ""}|${end ?: ""}"
                        padaukDispatchActionWithString(widget.confirmActionId, payload)
                    }) {
                        Text(widget.confirmLabel)
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
internal fun renderTimePickerDialog(widget: AndroidUiNode.TimePickerDialog) {
    val calendar = remember {
        Calendar.getInstance()
    }
    val initialHour = widget.initialHour ?: calendar.get(Calendar.HOUR_OF_DAY)
    val initialMinute = widget.initialMinute ?: calendar.get(Calendar.MINUTE)
    val state = rememberTimePickerState(
        initialHour = initialHour,
        initialMinute = initialMinute,
        is24Hour = widget.is24Hour,
    )
    var inputMode by remember { mutableStateOf(false) }

    Dialog(
        onDismissRequest = {
            if (widget.dismissible) {
                widget.dismissActionId?.let { padaukDispatchAction(it) }
            }
        },
        properties = DialogProperties(usePlatformDefaultWidth = false),
    ) {
        Surface(
            modifier = Modifier.fillMaxWidth().padding(horizontal = 16.dp),
            shape = MaterialTheme.shapes.extraLarge,
            tonalElevation = 6.dp,
        ) {
            Column(modifier = Modifier.fillMaxWidth().padding(16.dp)) {
                Text(
                    text = widget.title ?: "Select time",
                    style = MaterialTheme.typography.titleMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    modifier = Modifier.padding(bottom = 8.dp),
                )
                if (inputMode) {
                    TimeInput(state = state, modifier = Modifier.fillMaxWidth())
                } else {
                    TimePicker(state = state, modifier = Modifier.fillMaxWidth())
                }
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.End,
                ) {
                    if (widget.showModeToggle) {
                        TextButton(onClick = { inputMode = !inputMode }) {
                            Text(if (inputMode) "Clock" else "Keyboard")
                        }
                    }
                    if (widget.dismissLabel != null && widget.dismissActionId != null) {
                        TextButton(onClick = { padaukDispatchAction(widget.dismissActionId) }) {
                            Text(widget.dismissLabel)
                        }
                    }
                    TextButton(onClick = {
                        val payload = "${state.hour}|${state.minute}"
                        padaukDispatchActionWithString(widget.confirmActionId, payload)
                    }) {
                        Text(widget.confirmLabel)
                    }
                }
            }
        }
    }
}
