use cocoa::appkit::*;
use cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory;
use cocoa::base::{id, nil, selector};
use cocoa::foundation::*;
use objc::declare::ClassDecl;
use objc::runtime::{Object, Class, Sel};
use objc::*;

static mut POPOVER: id = nil;
static mut STATUS_ITEM: id = nil;

extern fn toggle_popover(_this: &Object, _cmd: Sel, _sender: id) {
  unsafe {
    let is_shown: bool = msg_send![POPOVER, isShown];
    if is_shown {
      let _: () = msg_send![POPOVER, close];
    } else {
      let button = STATUS_ITEM.button();
      let _: () = msg_send![POPOVER,
				showRelativeToRect: cocoa::appkit::NSView::frame(button)
        ofView: button
        preferredEdge: 1u64
      ];
    }
  }
}

fn main() {
  unsafe {
    let _pool = NSAutoreleasePool::new(nil);
    let app = NSApp();
    app.setActivationPolicy_(NSApplicationActivationPolicyAccessory);

    // status item
    let bar = NSStatusBar::systemStatusBar(nil);
    STATUS_ITEM = bar.statusItemWithLength_(-1.0);
		let icon_path = NSString::alloc(nil).init_str("assets/favicon-light.png");
		let icon_image: id = msg_send![class!(NSImage), alloc];
		let icon_image: id = msg_send![icon_image, initWithContentsOfFile: icon_path];
		let _: () = msg_send![STATUS_ITEM.button(), setImage: icon_image];

    // click handler
    let superclass = Class::get("NSObject").unwrap();
    let mut decl = ClassDecl::new("Delegate", superclass).unwrap();
    decl.add_method(
      selector("toggle:"),
      toggle_popover as extern fn(&Object, Sel, id)
    );
    let delegate_class = decl.register();
    let delegate: id = msg_send![delegate_class, new];
    cocoa::appkit::NSButton::setTarget_(STATUS_ITEM.button(), delegate);
    STATUS_ITEM.button().setAction_(selector("toggle:"));

    // popover
    POPOVER = msg_send![class!(NSPopover), new];
    let _: () = msg_send![POPOVER, setBehavior: 1i64];

    // empty view inside popover
    let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(300.0, 200.0));
    let view: id = msg_send![class!(NSView), alloc];
    let view: id = msg_send![view, initWithFrame: frame];
    let vc: id = msg_send![class!(NSViewController), new];
    let _: () = msg_send![vc, setView: view];
    let _: () = msg_send![POPOVER, setContentViewController: vc];
    let _: () = msg_send![POPOVER, setContentSize: NSSize::new(300.0, 200.0)];

    app.run();
  }
}