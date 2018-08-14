#include<stdio.h>
@import Cocoa;

// Rust draw callback
typedef void (*drawcb_t)(CGContextRef, NSPoint);

// View drawn by Rust
@interface View : NSView {
    drawcb_t drawcb_;
}

@end

@implementation View

- (instancetype)initWithFrame:(NSRect)frameRect drawcb:(drawcb_t)drawcb {
    self = [super initWithFrame: frameRect];
    if (self) {
        drawcb_ = drawcb;
    }
    // puts("aa");
    return self;
}

- (void)drawRect:(NSRect)dirtyRect {
    [super drawRect: dirtyRect];
    // puts("hi");
    CGContextRef ctx = NSGraphicsContext.currentContext.CGContext;
    drawcb_(ctx, [self.window mouseLocationOutsideOfEventStream]);
}

@end

// App delegate
@interface Del : NSObject<NSApplicationDelegate> {
    @private
    BOOL loop;
    NSTimeInterval interval;
}
@property (strong) NSMenu* menubar;
@property (strong) NSMenuItem* appMenuItem;
@property (strong) NSMenuItem* quitMenuItem;
@property (strong) NSMenu* appMenu;
@property (weak) NSWindow* w;
@property (weak) NSTimer* timer;
@property (weak) NSMenuItem* toggleLoopItem;

@end

@implementation Del

- (BOOL)applicationShouldTerminateAfterLastWindowClosed:(NSApplication *)sender {
    return true;
}

- (void)applicationWillFinishLaunching:(NSNotification *)notification {
    loop = false;
    interval = 0.01;
    self.menubar = [[NSMenu alloc] initWithTitle: @"Bai"];
    self.appMenuItem = [[NSMenuItem alloc] initWithTitle: @"RaffaGen" action: nil keyEquivalent: @""];
    [self.menubar addItem: self.appMenuItem];
    // [NSApp setMainMenu: menubar];
    self.appMenu = [[NSMenu alloc] initWithTitle: @"RaffaGen"];
    self.quitMenuItem = [[NSMenuItem alloc] initWithTitle:@"Quit RaffaGen"
            action:@selector(terminate:)
            keyEquivalent:@"q"];
    NSMenuItem* closeItem = [[NSMenuItem alloc] initWithTitle:@"Close Window"
            action:@selector(terminate:)
            keyEquivalent:@"w"];
    NSMenuItem* redrawItem = [[NSMenuItem alloc] initWithTitle:@"Redraw"
            action:@selector(redrawClicked)
            keyEquivalent:@"r"];
    NSMenuItem* toggleLoopItem = [[NSMenuItem alloc] initWithTitle:@"Start Loop"
            action:@selector(toggleLoop)
            keyEquivalent:@"l"];

    
    NSMenuItem* frameRateMenuItem = [[NSMenuItem alloc] initWithTitle:@"Framerate"
            action:nil
            keyEquivalent:@""];
    NSMenu* frameRateMenu = [[NSMenu alloc] initWithTitle: @"Framerate"];
    
    NSMenuItem* slowItem = [[NSMenuItem alloc] initWithTitle:@"Slow"
            action:@selector(slow)
            keyEquivalent:@"1"];
    [frameRateMenu addItem: slowItem];
    NSMenuItem* mediumItem = [[NSMenuItem alloc] initWithTitle:@"Medium"
            action:@selector(medium)
            keyEquivalent:@"2"];
    [frameRateMenu addItem: mediumItem];
    NSMenuItem* fastItem = [[NSMenuItem alloc] initWithTitle:@"Fast"
            action:@selector(fast)
            keyEquivalent:@"3"];
    [frameRateMenu addItem: fastItem];

    [frameRateMenuItem setSubmenu: frameRateMenu];
    
    self.toggleLoopItem = toggleLoopItem;
    [self.appMenu addItem: self.quitMenuItem];
    [self.appMenu addItem: closeItem];
    [self.appMenu addItem: redrawItem];
    [self.appMenu addItem: toggleLoopItem];
    [self.appMenu addItem: frameRateMenuItem];
    [self.appMenuItem setSubmenu: self.appMenu];
    // puts("varmi");
    // NSLog(@"%@", NSApp.mainMenu);
    NSApp.mainMenu = self.menubar;
    // NSLog(@"%@", NSApp.mainMenu);

}

- (void) redrawClicked {
    self.w.contentView.needsDisplay = true;
    NSAlert* al = [[NSAlert alloc] init];
    // al.messageText = @"TOE dael aeeE";
    // [al runModal];
}

- (void) toggleLoop {
    if (loop) {
        [self.timer invalidate];
        self.toggleLoopItem.title = @"Start Loop";
        loop = false;
    } else {
        NSTimer *timer = [NSTimer timerWithTimeInterval: interval repeats: true block: ^void (NSTimer *timer) {
            // self.w.contentView.needsDisplay = true;
            [self.w.contentView display];
        }];
        self.timer = timer;
        [[NSRunLoop mainRunLoop] addTimer: timer forMode: NSRunLoopCommonModes];
        self.toggleLoopItem.title = @"Stop Loop";
        loop = true;
    }
}

- (void) slow {
    [self setInterval: 1];
}

- (void) medium {
    [self setInterval: 0.1];
}

- (void) fast {
    [self setInterval: 0.01];
}

- (void) setInterval:(NSTimeInterval)newInterval {
    interval = newInterval;
    if (loop) {
        [self.timer invalidate];
        NSTimer *timer = [NSTimer timerWithTimeInterval: interval repeats: true block: ^void (NSTimer *timer) {
            // self.w.contentView.needsDisplay = true;
            [self.w.contentView display];
        }];
        self.timer = timer;
        [[NSRunLoop mainRunLoop] addTimer: timer forMode: NSRunLoopCommonModes];
    }
}

@end

void setup(drawcb_t drawcb) {
    @autoreleasepool {
        [NSApplication sharedApplication];
        [NSApp setActivationPolicy: NSApplicationActivationPolicyRegular];
        NSWindow* window = 
            [[NSWindow alloc] initWithContentRect: NSMakeRect(100.0, 100.0, 500.0,500.0)
            styleMask: NSWindowStyleMaskTitled | NSWindowStyleMaskClosable
            backing: NSBackingStoreBuffered
            defer: false];
        [window makeKeyAndOrderFront: NSApp];
        View* view = [[View alloc] initWithFrame: window.contentView.frame drawcb: drawcb];
        window.contentView = view;
        puts("hihiiii");

        Del* del = [[Del alloc] init];
        del.w = window;
        NSApp.delegate = del;

        [NSApp activateIgnoringOtherApps:YES];
        [NSApp run];
    }
}