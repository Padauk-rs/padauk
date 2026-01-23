import UIKit
import PadaukFFI

open class PadaukViewController: UIViewController {
    override open func viewDidLoad() {
        super.viewDidLoad()
        
        // 1. The Handshake
        // Swift is static, so we can often just call the function 
        // if we link the User's library correctly.
        padauk_init() 
        
        // 2. Render
        if let root = padauk_render_root() {
            let renderer = PadaukRenderer(root: root)
            renderer.render(in: self.view)
        }
    }
}