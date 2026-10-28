use domain::*;

#[test]
fn supported_keyboards() {
    assert_eq!(KeyboardFamily::of(UsbIdentity::new(0x1189, 0x8890)), Some(KeyboardFamily::K8890));
    assert_eq!(KeyboardFamily::of(UsbIdentity::new(0x1189, 0x8840)), Some(KeyboardFamily::K884x));
    assert_eq!(KeyboardFamily::of(UsbIdentity::new(0x514C, 0x8851)), Some(KeyboardFamily::K884x));
    // The cross-platform app looks for 8851 under 514C only.
    assert_eq!(KeyboardFamily::of(UsbIdentity::new(0x1189, 0x8851)), None);
    assert_eq!(KeyboardFamily::of(UsbIdentity::new(0x05AC, 0x8890)), None);
    assert_eq!(KeyboardFamily::SUPPORTED.len(), 16);
    let unique: std::collections::HashSet<_> = KeyboardFamily::SUPPORTED.iter().collect();
    assert_eq!(unique.len(), 16);
}

#[test]
fn usb_identity_prints_as_hex() {
    assert_eq!(UsbIdentity::new(0x514C, 0x8850).to_string(), "514C:8850");
}

#[test]
fn identity_picks_the_record_format() {
    assert_eq!(KeyboardIdentity::new(6, 1, 10).record_format(), RecordFormat::Standard);
    assert_eq!(KeyboardIdentity::new(12, 2, 11).record_format(), RecordFormat::Standard);
    assert_eq!(KeyboardIdentity::new(6, 1, 0).record_format(), RecordFormat::Legacy);
    assert_eq!(KeyboardIdentity::new(6, 1, 1).record_format(), RecordFormat::Legacy);
}

fn capabilities(commands: CommandSet) -> KeyboardCapabilities {
    KeyboardCapabilities::new(commands)
}

const K8890: CommandSet = CommandSet::Packets { report_id: 3 };
const STANDARD: CommandSet = CommandSet::Records(RecordFormat::Standard);
const LEGACY: CommandSet = CommandSet::Records(RecordFormat::Legacy);
const UNIDENTIFIED: CommandSet = CommandSet::Records(RecordFormat::Unidentified);

#[test]
fn layers() {
    assert_eq!(capabilities(K8890).layer_count(), 3);
    assert_eq!(capabilities(CommandSet::Packets { report_id: 0 }).layer_count(), 1);
    assert_eq!(capabilities(STANDARD).layer_count(), 3);
}

/// The standard format spends a place on every modifier; the others put a key's modifiers with it.
#[test]
fn presses_count_modifiers_on_the_standard_format() {
    let macro_ = KeyMacro::new(vec![KeyStroke::new(Modifiers::SUPER | Modifiers::SHIFT, 0x1D), KeyStroke::key(0x04)]);
    assert_eq!(capabilities(STANDARD).presses(&macro_), 4);
    assert_eq!(capabilities(LEGACY).presses(&macro_), 2);
    assert_eq!(capabilities(K8890).presses(&macro_), 2);
    let modifiers_only = KeyMacro::new(vec![KeyStroke::new(Modifiers::CONTROL | Modifiers::ALT, 0)]);
    assert_eq!(capabilities(STANDARD).presses(&modifiers_only), 2);
}

#[test]
fn limits() {
    let five = KeyMacro::new(vec![KeyStroke::key(0x04); 5]);
    let six = KeyMacro::new(vec![KeyStroke::key(0x04); 6]);
    assert!(capabilities(K8890).fits(&five));
    assert!(!capabilities(K8890).fits(&six));
    assert!(capabilities(LEGACY).fits(&KeyMacro::new(vec![KeyStroke::key(0x04); 20])));
    assert!(!capabilities(STANDARD).fits(&KeyMacro::new(vec![KeyStroke::new(Modifiers::SUPER, 0x04); 10])));
    assert!(!capabilities(K8890).fits(&KeyMacro::new(vec![KeyStroke::key(0x04), KeyStroke::new(Modifiers::SHIFT, 0)])));
}

#[test]
fn what_each_keyboard_does() {
    assert_eq!(capabilities(K8890).media_keys(), &MediaKey::BASIC);
    assert_eq!(capabilities(STANDARD).media_keys().len(), 21);
    assert_eq!(capabilities(K8890).delays(), DelaySupport::Unavailable);
    assert_eq!(capabilities(STANDARD).delays(), DelaySupport::PerPress(0..=60000));
    assert_eq!(capabilities(LEGACY).delays(), DelaySupport::PerKey(0..=6000));
    assert_eq!(capabilities(UNIDENTIFIED).delays(), DelaySupport::Unavailable);
    assert_eq!(capabilities(K8890).lighting(), LightingSupport::Modes(vec![0, 1, 2]));
    assert_eq!(capabilities(STANDARD).lighting(), LightingSupport::PerKeyColor { modes: 0..=5, keys: 16 });
    assert_eq!(capabilities(LEGACY).lighting(), LightingSupport::ColorAndMode { modes: 0..=5 });
    assert_eq!(capabilities(STANDARD).reading(), ReadSupport::OnConnect);
    assert_eq!(capabilities(LEGACY).reading(), ReadSupport::OnRequest);
    assert_eq!(capabilities(K8890).reading(), ReadSupport::Never);
    assert_eq!(capabilities(UNIDENTIFIED).reading(), ReadSupport::Never);
}

/// The 8890 numbers twelve keys and two knobs.
#[test]
fn the_8890_addresses_small_layouts_only() {
    let named = |id| KeyboardLayout::named(id).unwrap();
    assert!(capabilities(K8890).addresses(named("6add1")));
    assert!(capabilities(K8890).addresses(named("12add2")));
    assert!(!capabilities(K8890).addresses(named("15add3")));
    assert!(!capabilities(K8890).addresses(named("4add3")));
    assert!(!capabilities(K8890).addresses(named("0add1")));
    assert!(capabilities(UNIDENTIFIED).addresses(named("21add1")));
}

/// The keys each vendor tool offers: 95 on the Windows tool, 110 on the cross-platform app.
#[test]
fn keys_each_keyboard_is_given() {
    let k8890 = capabilities(K8890);
    let standard = capabilities(STANDARD);
    assert_eq!((0..=0xFF).filter(|key| k8890.takes(*key)).count(), 95);
    assert_eq!((0..=0xFF).filter(|key| standard.takes(*key)).count(), 110);
    assert!((0..=0xFF).all(|key| capabilities(UNIDENTIFIED).takes(key) == capabilities(LEGACY).takes(key)));
    for key in [0x32, 0x58, 0x64, 0x68, 0x73] {
        assert!(!k8890.takes(key) && standard.takes(key), "{key:#x}");
    }
    assert!(k8890.takes(0x65) && !k8890.takes(0x66) && !standard.takes(0x66) && !standard.takes(0x74));
    assert!(k8890.takes_keys_of(&KeyMacro::new(vec![KeyStroke::new(Modifiers::SHIFT, 0)])));
    assert!(!k8890.takes_keys_of(&KeyMacro::new(vec![KeyStroke::key(0x04), KeyStroke::key(0x68)])));
}

/// The 8890 has twelve keys and two knobs that turn and press.
#[test]
fn what_the_8890_takes() {
    let k8890 = capabilities(K8890);
    assert!(k8890.can_address(ControlId::Key(12)));
    assert!(!k8890.can_address(ControlId::Key(13)));
    assert!(k8890.can_address(ControlId::Knob(2, KnobPart::RotateRight)));
    assert!(!k8890.can_address(ControlId::Knob(3, KnobPart::Press)));
    assert!(!k8890.can_address(ControlId::Knob(1, KnobPart::PressRotateLeft)));
    assert!(capabilities(STANDARD).can_address(ControlId::Knob(1, KnobPart::PressRotateLeft)));
}

#[test]
fn mouse_actions_come_back_from_their_parts() {
    for action in MouseAction::ALL {
        assert_eq!(MouseAction::from_parts(action.buttons(), action.wheel(), action.modifiers()), Some(action));
    }
    assert_eq!(MouseAction::from_parts(0, 0, Modifiers::NONE), None);
}
