use hotkey::{HotkeyListener, ListenerHotkey};
use rdev::EventType;

fn send_ctrl_c() {
  send(&EventType::KeyPress(rdev::Key::ControlLeft));
  send(&EventType::KeyPress(rdev::Key::KeyC));
  send(&EventType::KeyRelease(rdev::Key::ControlLeft));
  send(&EventType::KeyRelease(rdev::Key::KeyC));
}

fn release_all_keys() {
  send(&EventType::KeyRelease(rdev::Key::ControlLeft));
  send(&EventType::KeyRelease(rdev::Key::ShiftLeft));
  send(&EventType::KeyRelease(rdev::Key::KeyA));
}

fn send(event_type: &EventType) {
  let delay = std::time::Duration::from_millis(20);
  match rdev::simulate(event_type) {
    Ok(()) => (),
    Err(_) => {
      println!("We could not send {:?}", event_type);
    }
  }
  std::thread::sleep(delay);
}

fn main() {
  let mut hk = hotkey::Listener::new();
  let mut cb = arboard::Clipboard::new().unwrap();
  hk.register_hotkey(
    ListenerHotkey::new(
      hotkey::modifiers::CONTROL | hotkey::modifiers::SHIFT,
      'A' as u32,
    ),
    move || {
      println!("Keybind triggered!");
      let temp = cb.get_text().unwrap();
      release_all_keys();
      send_ctrl_c();
      let sel = cb.get_text().unwrap();
      println!("Selected text: {sel}");
      cb.set_text(temp).unwrap();
    },
  )
  .unwrap();
  std::thread::sleep(std::time::Duration::from_secs(600));
}
