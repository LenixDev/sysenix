use cocoa::appkit::*;
use cocoa::appkit::{
  NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory, NSButton, NSView,
};
use cocoa::base::{id, nil, selector};
use cocoa::foundation::*;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::*;

static mut POPOVER: id = nil;
static mut STATUS_ITEM: id = nil;
static WIDTH: f64 = 300.0;
static HEIGHT: f64 = 200.0;

extern "C" fn toggle_popover(_this: &Object, _cmd: Sel, _sender: id) {
  unsafe {
    let is_shown: bool = msg_send![POPOVER, isShown];
    if is_shown {
      let _: () = msg_send![POPOVER, close];
    } else {
      let button = STATUS_ITEM.button();
      let _: () = msg_send![POPOVER,
        showRelativeToRect: NSView::frame(button)
        ofView: button
        preferredEdge: 1u64
      ];
    }
  }
}

fn sbi_set_app_icon() {
  unsafe {
    let icon_path = NSString::alloc(nil).init_str("assets/favicon-light.png");
    let icon_image: id = msg_send![class!(NSImage), alloc]; /* just an allocated object */
    let icon_image: id = msg_send![icon_image, initWithContentsOfFile: icon_path];
    let _: () = msg_send![STATUS_ITEM.button(), setImage: icon_image];
  }
}

extern "C" fn quit(_this: &Object, _cmd: Sel, _sender: id) {
  std::process::exit(0);
}

fn sbi_handle_click(delegate: *mut Object) {
  unsafe {
    // click handler
    NSButton::setTarget_(STATUS_ITEM.button(), delegate);
    STATUS_ITEM.button().setAction_(selector("toggle:"));

    // popover
    POPOVER = msg_send![class!(NSPopover), new];
    let _: () = msg_send![POPOVER, setBehavior: 1i64];
  }
}

fn main() {
  unsafe {
    let _pool = NSAutoreleasePool::new(nil);
    let app = NSApp();

    // tells macos that the app is an accessory(no dock icon, no CMD + TAB window)
    app.setActivationPolicy_(NSApplicationActivationPolicyAccessory);

    // status item
    let bar = NSStatusBar::systemStatusBar(nil);
    STATUS_ITEM = bar.statusItemWithLength_(-1.0);

    sbi_set_app_icon();

    let superclass = Class::get("NSObject").unwrap();
    let mut decl = ClassDecl::new("Delegate", superclass).unwrap();

		// Object-C methods
    decl.add_method(
      selector("toggle:"),
      toggle_popover as extern "C" fn(&Object, Sel, id),
    );
		decl.add_method(
			selector("quit:"),
			quit as extern "C" fn(&Object, Sel, id)
		);

    let delegate_class = decl.register();
    let delegate: id = msg_send![delegate_class, new];

    sbi_handle_click(delegate);
    
		// sbi_popover
    let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(WIDTH, HEIGHT));
    let view: id = msg_send![class!(NSView), alloc];
    let view: id = msg_send![view, initWithFrame: frame];
    let vc: id = msg_send![class!(NSViewController), new];
    let _: () = msg_send![vc, setView: view];
    let _: () = msg_send![POPOVER, setContentViewController: vc];
    let _: () = msg_send![POPOVER, setContentSize: NSSize::new(WIDTH, HEIGHT)];

    app.run();
  }
}
