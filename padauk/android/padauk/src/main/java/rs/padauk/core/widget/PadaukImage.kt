package rs.padauk.core.widget

import android.graphics.BitmapFactory
import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import rs.padauk.core.BoxFit
import rs.padauk.core.ImageSource
import java.io.File
import java.net.URL

@Composable
fun PadaukImage(source: ImageSource, fit: BoxFit, modifier: Modifier) {
    val contentScale = when (fit) {
        BoxFit.CONTAIN -> ContentScale.Fit
        BoxFit.COVER -> ContentScale.Crop
        BoxFit.FILL -> ContentScale.FillBounds
        BoxFit.FIT_WIDTH -> ContentScale.FillWidth
        BoxFit.FIT_HEIGHT -> ContentScale.FillHeight
        BoxFit.NONE -> ContentScale.None
        BoxFit.SCALE_DOWN -> ContentScale.Inside
    }

    when (source) {
        is ImageSource.Asset -> {
            val context = LocalContext.current
            // Clean up the name (remove extension if user added it, Android IDs don't have extensions)
            val cleanName = source.name.substringBeforeLast(".")
            val resId = context.resources.getIdentifier(cleanName, "drawable", context.packageName)

            if (resId != 0) {
                Image(
                    painter = painterResource(id = resId),
                    contentDescription = null,
                    modifier = modifier,
                    contentScale = contentScale
                )
            } else {
                // Fallback / Placeholder
                Box(modifier = modifier.size(50.dp)) // Empty box or error icon
            }
        }
        is ImageSource.Network -> {
            AsyncImageLoader(source.url, modifier, contentScale)
        }
        is ImageSource.File -> {
            AsyncFileLoader(source.path, modifier, contentScale)
        }
        is ImageSource.Memory -> {
            val bytes = source.data
            val bitmap = remember(bytes) {
                BitmapFactory.decodeByteArray(bytes, 0, bytes.size)?.asImageBitmap()
            }
            if (bitmap != null) {
                Image(
                    bitmap = bitmap,
                    contentDescription = null,
                    modifier = modifier,
                    contentScale = contentScale
                )
            }
        }
    }
}

@Composable
fun AsyncImageLoader(url: String, modifier: Modifier, scale: ContentScale) {
    var imageBitmap by remember { mutableStateOf<ImageBitmap?>(null) }

    LaunchedEffect(url) {
        withContext(Dispatchers.IO) {
            try {
                val stream = URL(url).openStream()
                val bmp = BitmapFactory.decodeStream(stream)
                imageBitmap = bmp?.asImageBitmap()
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }

    if (imageBitmap != null) {
        Image(
            bitmap = imageBitmap!!,
            contentDescription = null,
            modifier = modifier,
            contentScale = scale
        )
    } else {
        // Loading state
        Box(modifier = modifier) {
            CircularProgressIndicator(modifier = Modifier.align(androidx.compose.ui.Alignment.Center))
        }
    }
}

@Composable
fun AsyncFileLoader(path: String, modifier: Modifier, scale: ContentScale) {
    var imageBitmap by remember { mutableStateOf<ImageBitmap?>(null) }

    LaunchedEffect(path) {
        withContext(Dispatchers.IO) {
            val file = File(path)
            if (file.exists()) {
                val bmp = BitmapFactory.decodeFile(file.absolutePath)
                imageBitmap = bmp?.asImageBitmap()
            }
        }
    }

    if (imageBitmap != null) {
        Image(
            bitmap = imageBitmap!!,
            contentDescription = null,
            modifier = modifier,
            contentScale = scale
        )
    }
}