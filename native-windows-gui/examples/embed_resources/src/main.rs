/*!
    A very simple application that show your name in a message box.
    Use a manifest file to load control style and requires admin priviledge to start.

    For more info about resources see https://docs.microsoft.com/en-us/windows/win32/menurc/resource-definition-statements
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct EmbedApp {
    #[nwg_control(size: (300, 145), position: (300, 300), flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [EmbedApp::say_goodbye], OnInit: [EmbedApp::init] )]
    window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_control(size: (280, 25), position: (10, 10))]
    name_edit: nwg::TextInput,

    #[nwg_control(position: (130, 110), size: (35, 35))]
    embed_bitmap: nwg::ImageFrame,

    #[nwg_control(size: (280, 60), position: (10, 40))]
    #[nwg_events( OnButtonClick: [EmbedApp::say_hello] )]
    hello_button: nwg::Button
}

impl EmbedApp {

    fn init(&self) {
        let em = &self.embed;
        self.name_edit.set_text(&em.string(0).unwrap());
        self.hello_button.set_text(&em.string(1).unwrap());

        self.window.set_text(&em.string(2).unwrap());
        self.window.set_icon(em.icon_str("TEST").as_ref());

        self.embed_bitmap.set_bitmap(em.bitmap_str("BALL").as_ref());
    }

    fn say_hello(&self) {
        nwg::simple_message("Hello", &format!("Hello {}", self.name_edit.text()));
    }
    
    fn say_goodbye(&self) {
        nwg::simple_message("Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = EmbedApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
