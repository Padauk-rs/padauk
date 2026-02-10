package rs.padauk.core.widget

import android.annotation.SuppressLint
import android.graphics.Color.parseColor
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.RectangleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.ui.semantics.disabled
import androidx.compose.ui.semantics.semantics
import androidx.compose.ui.unit.dp
import androidx.compose.ui.zIndex
import rs.padauk.core.Modifiers
import rs.padauk.core.ColorValue


@SuppressLint("ModifierFactoryExtensionFunction")
fun Modifiers.toCompose(): Modifier {
    var m : Modifier = Modifier
    val shape = if (this.cornerRadius != null) {
        RoundedCornerShape(this.cornerRadius!!.dp)
    } else {
        RectangleShape
    }

    if (this.offsetX != null || this.offsetY != null) {
        val x = this.offsetX ?: 0f
        val y = this.offsetY ?: 0f
        m = m.offset(x.dp, y.dp)
    }

    this.zIndex?.let { m = m.zIndex(it) }

    if (this.fillMaxWidth) {
        m = m.fillMaxWidth()
    }
    if (this.fillMaxHeight) {
        m = m.fillMaxHeight()
    }
    this.width?.let { m = m.width(it.dp) }
    this.height?.let { m = m.height(it.dp) }

    if (this.paddingHorizontal != null || this.paddingVertical != null) {
        val h = this.paddingHorizontal ?: 0f
        val v = this.paddingVertical ?: 0f
        m = m.padding(horizontal = h.dp, vertical = v.dp)
    } else {
        this.padding?.let { m = m.padding(it.dp) }
    }

    if (this.clip || this.cornerRadius != null) {
        m = m.clip(shape)
    }

    this.backgroundColor?.let {
        m = m.then(Modifier.background(it.toComposeColor(), shape))
    }

    if (this.borderWidth != null && this.borderColor != null) {
        m = m.border(this.borderWidth!!.dp, this.borderColor!!.toComposeColor(), shape)
    }

    this.alpha?.let { m = m.alpha(it) }

    if (this.enabled == false) {
        if (this.alpha == null) {
            m = m.alpha(0.38f)
        }
        m = m.semantics { disabled() }
    }

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

fun ColorValue.toComposeColor(): Color {
    return when (this) {
        is ColorValue.Rgb -> Color(
            this.r.toInt(),
            this.g.toInt(),
            this.b.toInt(),
            this.a.toInt()
        )
        is ColorValue.Hex -> this.value.toComposeColor()
    }
}
