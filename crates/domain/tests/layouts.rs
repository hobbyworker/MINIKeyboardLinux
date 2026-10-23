use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;

use domain::ports::LayoutProfileRepository;
use domain::usecases::{ChooseLayoutUseCase, ResolveLayoutUseCase};
use domain::*;

#[test]
fn every_layout_is_consistent() {
    assert_eq!(KeyboardLayout::all().len(), 26);
    assert_eq!(KeyboardLayout::all().iter().map(|layout| &layout.id).collect::<HashSet<_>>().len(), 26);
    for layout in KeyboardLayout::all() {
        assert_eq!(layout.controls.iter().collect::<HashSet<_>>().len(), layout.controls.len(), "{}", layout.id);
        let keys: Vec<u8> = layout
            .controls
            .iter()
            .filter_map(|control| match control {
                ControlId::Key(number) => Some(*number),
                ControlId::Knob(..) => None,
            })
            .collect();
        assert_eq!(keys, (1..=keys.len() as u8).collect::<Vec<_>>(), "{}", layout.id);
        let knobs: BTreeSet<u8> = layout.controls.iter().filter_map(ControlId::knob_number).collect();
        assert_eq!(knobs, (1..=layout.knob_count() as u8).collect(), "{}", layout.id);
    }
}

/// `Widget::Identify_KeyBoard_style`.
#[test]
fn identified_layouts() {
    let id =
        |keys, knobs, kind, product| KeyboardLayout::identified_id(&KeyboardIdentity::new(keys, knobs, kind), product);
    assert_eq!(id(6, 1, 10, 0x8840), "6add1");
    assert_eq!(id(0, 2, 10, 0x8840), "0add1");
    assert_eq!(id(0, 2, 10, 0x8850), "0add2");
    assert_eq!(id(4, 1, 0, 0x8840), "4add1");
    assert_eq!(id(4, 1, 0, 0x8851), "4add2");
    assert_eq!(id(4, 1, 1, 0x8840), "4add2");
    assert_eq!(id(4, 1, 10, 0x8840), "15add3");
    assert_eq!(id(7, 7, 10, 0x8840), "15add3");
    for layout in KeyboardLayout::all() {
        assert!(KeyboardLayout::named(&layout.id).is_some());
    }
}

/// One knob that also turns while pressed — not two knobs.
#[test]
fn the_dial_has_one_knob() {
    let dial = KeyboardLayout::named("0add1").unwrap();
    assert_eq!(dial.knob_count(), 1);
    assert!(dial.has(ControlId::Knob(1, KnobPart::PressRotateLeft)));
    assert!(dial.has(ControlId::Knob(1, KnobPart::PressRotateRight)));
    assert_eq!(KeyboardLayout::named("0add2").unwrap().knob_count(), 2);
}

#[test]
fn rotation_turns_around() {
    assert_eq!(Rotation::ThreeQuarters.next(), Rotation::Upright);
    assert_eq!(Rotation::Upright.next(), Rotation::Quarter);
    assert_eq!(Rotation::from_quarter_turns(-1), Rotation::ThreeQuarters);
    assert_eq!(Rotation::from_quarter_turns(6), Rotation::Half);
}

#[derive(Default)]
struct Profiles(RefCell<HashMap<UsbIdentity, LayoutProfile>>);

impl LayoutProfileRepository for Profiles {
    fn profile(&self, keyboard: UsbIdentity) -> Option<LayoutProfile> {
        self.0.borrow().get(&keyboard).cloned()
    }

    fn save(&self, profile: LayoutProfile, keyboard: UsbIdentity) {
        self.0.borrow_mut().insert(keyboard, profile);
    }
}

fn keyboard(product: u16, family: KeyboardFamily) -> ConnectedKeyboard {
    ConnectedKeyboard {
        usb: UsbIdentity::new(0x1189, product),
        family,
        interface_number: 0,
        channel: Channel::Hid,
        report_id: 3,
        report_length: 65,
        details: None,
    }
}

#[test]
fn an_identified_keyboard_gets_its_layout() {
    let identification = KeyboardIdentification {
        commands: CommandSet::Records(RecordFormat::Standard),
        identity: Some(KeyboardIdentity::new(9, 2, 10)),
    };
    let resolved = ResolveLayoutUseCase::new(Rc::new(Profiles::default()))
        .execute(&keyboard(0x8840, KeyboardFamily::K884x), &identification);
    assert_eq!(resolved.layout.id, "9add2");
    assert_eq!(resolved.source, Source::Identified);
    assert!(resolved.choices.is_empty());
    assert!(!resolved.needs_choice());
}

/// The cross-platform app doesn't know the 8890 and its own tool draws no shape, so the user arranges it.
/// Until then it is drawn as the tool's twelve keys, four to a row, and two knobs.
#[test]
fn the_8890_is_arranged() {
    let profiles = Rc::new(Profiles::default());
    let keyboard = keyboard(0x8890, KeyboardFamily::K8890);
    let identification = KeyboardIdentification { commands: CommandSet::Packets { report_id: 3 }, identity: None };
    let resolve = ResolveLayoutUseCase::new(profiles.clone());
    let choose = ChooseLayoutUseCase::new(profiles.clone());
    let resolved = resolve.execute(&keyboard, &identification);
    assert_eq!(resolved.layout.id, "grid-12-4-2");
    assert!(resolved.needs_choice());
    assert!(resolved.choices.is_empty());
    assert_eq!(resolved.grid, Some(GridLimits::K8890));

    let six_and_one = GridShape::new(6, 3, 1).unwrap().layout();
    let chosen = choose.execute(&resolved, &six_and_one, Rotation::Half, keyboard.usb);
    assert_eq!(chosen.layout, six_and_one);
    assert_eq!(chosen.rotation, Rotation::Half);
    assert_eq!(chosen.source, Source::Chosen);

    let again = resolve.execute(&keyboard, &identification);
    assert_eq!(again.layout, six_and_one);
    assert_eq!(again.rotation, Rotation::Half);
    assert!(!again.needs_choice());

    // Neither an arrangement it can't number nor one of the cross-platform app's layouts.
    let too_big = GridShape::new(13, 4, 0).unwrap().layout();
    assert_eq!(choose.execute(&again, &too_big, Rotation::Upright, keyboard.usb).layout, six_and_one);
    let app_layout = KeyboardLayout::named("6add1").unwrap();
    assert_eq!(choose.execute(&again, app_layout, Rotation::Upright, keyboard.usb).layout, six_and_one);
}

#[test]
fn an_unidentified_keyboard_may_have_any_layout() {
    let identification =
        KeyboardIdentification { commands: CommandSet::Records(RecordFormat::Unidentified), identity: None };
    let resolved = ResolveLayoutUseCase::new(Rc::new(Profiles::default()))
        .execute(&keyboard(0x8840, KeyboardFamily::K884x), &identification);
    assert_eq!(resolved.choices.len(), 26);
    assert_eq!(resolved.layout.id, "15add3");
    assert!(resolved.needs_choice());
}

/// The layout of a keyboard that said its shape stays; only the rotation is kept.
#[test]
fn an_identified_layout_only_turns() {
    let profiles = Rc::new(Profiles::default());
    let keyboard = keyboard(0x8840, KeyboardFamily::K884x);
    let identification = KeyboardIdentification {
        commands: CommandSet::Records(RecordFormat::Legacy),
        identity: Some(KeyboardIdentity::new(6, 1, 0)),
    };
    let resolve = ResolveLayoutUseCase::new(profiles.clone());
    let resolved = resolve.execute(&keyboard, &identification);
    let other = KeyboardLayout::named("12add0").unwrap();
    let chosen = ChooseLayoutUseCase::new(profiles).execute(&resolved, other, Rotation::Quarter, keyboard.usb);
    assert_eq!(chosen.layout.id, "6add1");
    assert_eq!(chosen.rotation, Rotation::Quarter);
    assert_eq!(resolve.execute(&keyboard, &identification).rotation, Rotation::Quarter);
}

#[test]
fn grid_ids() {
    let shape = GridShape::new(6, 3, 1).unwrap();
    assert_eq!(shape.id(), "grid-6-3-1");
    assert_eq!(GridShape::from_id("grid-6-3-1"), Some(shape));
    assert_eq!(GridShape::from_id("6add1"), None);
    assert_eq!(GridShape::from_id("grid-6-3-1-0"), None);
    assert_eq!(GridShape::new(0, 1, 0), None);
    assert_eq!(GridShape::new(6, 7, 0), None);
    assert!(GridShape::new(0, 1, 2).is_some());
    assert_eq!(GridShape::new(7, 3, 0).unwrap().rows(), 3);
}

/// Keys first, numbered from 1, then each knob's three actions.
#[test]
fn grid_controls() {
    let layout = GridShape::new(7, 3, 2).unwrap().layout();
    assert_eq!(layout.controls[..7], (1..=7).map(ControlId::Key).collect::<Vec<_>>());
    assert_eq!(
        layout.controls[layout.controls.len() - 3..],
        [
            ControlId::Knob(2, KnobPart::RotateLeft),
            ControlId::Knob(2, KnobPart::Press),
            ControlId::Knob(2, KnobPart::RotateRight)
        ]
    );
    assert!(layout.key_count() == 7 && layout.knob_count() == 2);
    let k8890 = KeyboardCapabilities::new(CommandSet::Packets { report_id: 3 });
    assert!(k8890.addresses(&layout));
    assert!(!k8890.addresses(&GridShape::new(13, 4, 0).unwrap().layout()));
}

#[test]
fn every_simulated_model_has_a_keyboard() {
    let all = SimulatedModel::all();
    assert!(all.len() > 40);
    assert_eq!(all.iter().map(SimulatedModel::id).collect::<HashSet<_>>().len(), all.len());
    assert!(all.iter().all(|model| KeyboardFamily::of(model.usb).is_some()));
    assert_eq!(SimulatedModel::identified_as(KeyboardLayout::named("4add1").unwrap(), RecordFormat::Standard), None);
}
