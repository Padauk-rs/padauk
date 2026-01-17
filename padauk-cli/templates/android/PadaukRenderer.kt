@Composable
fun PadaukRenderer(node: UiNode, onAction: (String) -> Unit) {
    when (node) {
        is UiNode.Column -> {
            Column(modifier = node.modifier.toCompose()) {
                node.children.forEach { child -> PadaukRenderer(child, onAction) }
            }
        }
        is UiNode.Text -> {
            Text(
                text = node.text,
                fontSize = node.spSize.sp,
                modifier = node.modifier.toCompose()
            )
        }
        is UiNode.Button -> {
            Button(
                onClick = { onAction(node.onClick) },
                modifier = node.modifier.toCompose()
            ) {
                if (node.content.isNotEmpty()) {
                    PadaukRenderer(node.content.first(), onAction)
                }
            }
        }
    }
}