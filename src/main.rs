use cocoa::appkit::*;
use cocoa::appkit::{
  NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory, NSButton, NSView,
};
use cocoa::base::{id, nil, selector};
use cocoa::foundation::*;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use objc::*;

mod constants;
use constants::{PADDING, WIDTH, HEIGHT};

static mut POPOVER: id = nil;
static mut STATUS_ITEM: id = nil;

fn objective_c_methods(decl: &mut ClassDecl) {
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
	unsafe {
		decl.add_method(
			selector("toggle:"),
			toggle_popover as extern "C" fn(&Object, Sel, id),
		);
	}

	extern "C" fn quit(_this: &Object, _cmd: Sel, _sender: id) {
		std::process::exit(0);
	}
	unsafe {
		decl.add_method(
			selector("quit:"),
			quit as extern "C" fn(&Object, Sel, id)
		);
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

		objective_c_methods(&mut decl);

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

		// popover header
		let header_frame = NSRect::new(NSPoint::new(PADDING, PADDING * 1.5), NSSize::new(WIDTH, HEIGHT - PADDING * 2.0));
		let header: id = msg_send![class!(NSTextField), alloc];
		let header: id = msg_send![header, initWithFrame: header_frame];
		let _: () = msg_send![header, setStringValue: NSString::alloc(nil).init_str("Sysenix")];
		let _: () = msg_send![header, setBordered: cocoa::base::NO];
		let _: () = msg_send![header, setEditable: cocoa::base::NO];
		let clear: id = msg_send![class!(NSColor), clearColor];
		let _: () = msg_send![header, setBackgroundColor: clear];
		let _: () = msg_send![view, addSubview: header];
		
		// separator
		let separator_frame = NSRect::new(NSPoint::new(PADDING, PADDING * 1.5), NSSize::new(WIDTH - (PADDING * 2.0), HEIGHT - PADDING));
		let separator: id = msg_send![class!(NSBox), alloc];
		let separator: id = msg_send![separator, initWithFrame: separator_frame];
		let _: () = msg_send![separator, setBoxType: 2i64]; // NSBoxSeparator
		let _: () = msg_send![view, addSubview: separator];

		// popover input item
		let input_frame = NSRect::new(NSPoint::new(PADDING, PADDING), NSSize::new(WIDTH - (PADDING * 2.0), 25.0));
		let input: id = msg_send![class!(NSTextField), alloc];
		let input: id = msg_send![input, initWithFrame: input_frame];
		let _: () = msg_send![input, setPlaceholderString: NSString::alloc(nil).init_str("Ask Sysenix...")];
		let _: () = msg_send![view, addSubview: input];

		
		// // sbi_quit_btn
		// let btn_frame = NSRect::new(NSPoint::new(100.0, 80.0), NSSize::new(100.0, 30.0));
		// let quit_btn: id = msg_send![class!(NSButton), alloc];
		// let quit_btn: id = msg_send![quit_btn, initWithFrame: btn_frame];
		// let _: () = msg_send![quit_btn, setTitle: NSString::alloc(nil).init_str("Quit")];
		// let _: () = msg_send![quit_btn, setTarget: delegate];
		// let _: () = msg_send![quit_btn, setAction: selector("quit:")];
		// let _: () = msg_send![view, addSubview: quit_btn];

    app.run();
  }
}
