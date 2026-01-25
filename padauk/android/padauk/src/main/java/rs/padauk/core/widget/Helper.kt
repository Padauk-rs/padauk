package rs.padauk.core.widget

import android.annotation.SuppressLint
import android.graphics.Color.parseColor
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import rs.padauk.core.Modifiers


@SuppressLint("ModifierFactoryExtensionFunction")
fun Modifiers.toCompose(): Modifier {
    var m : Modifier = Modifier
    this.padding.let { m = m.padding(it.dp) }
    this.width?.let { m = m.width(it.dp) }
    this.height?.let { m = m.height(it.dp) }
    this.backgroundColor?.let {
        m = m.then(Modifier.background(it.toComposeColor()))
    }
    // Add clickable, etc here

    return m
}

@SuppressLint("UseKtx")
fun String.toComposeColor(): Color {
    return try {
        Color(parseColor(this))
    } catch (e: Exception) {
        Color.Black
    }
}
