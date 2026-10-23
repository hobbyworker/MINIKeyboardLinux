//! The pages of actions for the selected key: keys, media keys, the mouse, the lights and waits.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use presentation::{
    ActionCaption, BoardCap, BoardFace, DelayPage, EditorModel, EditorTab, KeyArrangement, KeyGroup, KeyUsage,
    LightingPage, MediaChoice, ModifierKey, MouseChoice, Rgb,
};

use crate::i18n::{t, tf};
use crate::names::{
    control_title, key_group_title, led_color_css, led_color_title, lighting_mode_detail, lighting_mode_icon,
    lighting_mode_title, media_icon, media_title, modifier_title, mouse_icon, mouse_title, tab_title,
};
use crate::settings::Settings;
use crate::widgets::{caption, clear, key_button, keycap, section, set_active, tall_keycap, wrap_box};

/// The tabs and the page under them.
pub struct ActionPanel {
    root: gtk::Box,
    editor: Rc<EditorModel>,
    tabs: adw::ToggleGroup,
    shown_tabs: RefCell<Vec<EditorTab>>,
    updating: Rc<Cell<bool>>,
    stack: gtk::Stack,
    keys: KeysPage,
    media: ChoicesPage<MediaChoice>,
    mouse: ChoicesPage<MouseChoice>,
    lighting: LightingView,
    delay: DelayView,
}

fn tab_name(tab: EditorTab) -> &'static str {
    match tab {
        EditorTab::Keys => "keys",
        EditorTab::Media => "media",
        EditorTab::Mouse => "mouse",
        EditorTab::Lighting => "lighting",
        EditorTab::Delay => "delay",
    }
}

impl ActionPanel {
    pub fn new(editor: Rc<EditorModel>, settings: Rc<Settings>) -> Self {
        let root = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .build();
        let tabs = adw::ToggleGroup::builder().halign(gtk::Align::Start).build();
        let stack = gtk::Stack::builder().vhomogeneous(false).hhomogeneous(false).build();
        let scroller = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vexpand(true)
            .child(&stack)
            .build();
        root.append(&tabs);
        root.append(&scroller);

        let keys = KeysPage::new(editor.clone(), settings);
        let media = ChoicesPage::new(
            editor.clone(),
            t("Multimedia"),
            t("A media key replaces what the key did."),
            132,
            |choice| (media_icon(choice), media_title(choice)),
            |editor, choice| editor.assign_media(choice),
            |caption, choice| *caption == ActionCaption::Media(choice),
        );
        let mouse = ChoicesPage::new(
            editor.clone(),
            t("Mouse"),
            t("A mouse action replaces what the key did."),
            160,
            |choice| (mouse_icon(choice), mouse_title(choice)),
            |editor, choice| editor.assign_mouse(choice),
            |caption, choice| *caption == ActionCaption::Mouse(choice),
        );
        let lighting = LightingView::new(editor.clone());
        let delay = DelayView::new(editor.clone());
        for (tab, page) in [
            (EditorTab::Keys, keys.root.clone().upcast::<gtk::Widget>()),
            (EditorTab::Media, media.root.clone().upcast()),
            (EditorTab::Mouse, mouse.root.clone().upcast()),
            (EditorTab::Lighting, lighting.root.clone().upcast()),
            (EditorTab::Delay, delay.root.clone().upcast()),
        ] {
            page.set_margin_bottom(8);
            stack.add_named(&page, Some(tab_name(tab)));
        }

        let updating = Rc::new(Cell::new(false));
        let panel = Self {
            root,
            editor: editor.clone(),
            tabs,
            shown_tabs: RefCell::default(),
            updating: updating.clone(),
            stack,
            keys,
            media,
            mouse,
            lighting,
            delay,
        };
        panel.tabs.connect_active_name_notify(move |tabs| {
            if updating.get() {
                return;
            }
            let Some(name) = tabs.active_name() else { return };
            if let Some(tab) = EditorTab::ALL.into_iter().find(|tab| tab_name(*tab) == name) {
                editor.set_tab(tab);
            }
        });
        panel
    }

    pub fn widget(&self) -> &gtk::Box {
        &self.root
    }

    pub fn refresh(&self) {
        let editor = &self.editor;
        let available = editor.available_tabs();
        self.updating.set(true);
        if *self.shown_tabs.borrow() != available {
            self.tabs.remove_all();
            for tab in &available {
                self.tabs.add(adw::Toggle::builder().name(tab_name(*tab)).label(tab_title(*tab)).build());
            }
            *self.shown_tabs.borrow_mut() = available;
        }
        let tab = editor.tab();
        self.tabs.set_active_name(Some(tab_name(tab)));
        self.updating.set(false);
        self.stack.set_visible_child_name(tab_name(tab));
        self.root.set_sensitive(editor.is_ready());
        match tab {
            EditorTab::Keys => self.keys.refresh(),
            EditorTab::Media => self.media.refresh(editor.media_choices()),
            EditorTab::Mouse => self.mouse.refresh(MouseChoice::ALL.to_vec()),
            EditorTab::Lighting => self.lighting.refresh(),
            EditorTab::Delay => self.delay.refresh(),
        }
    }
}

/// Modifiers to hold, then a key: each key pressed is added to the selected key's presses. The keys are laid
/// out as on a full-size keyboard or in groups, as the user picks — until then, as the keyboard's vendor tool
/// does — and only the keys the keyboard is given can be pressed.
struct KeysPage {
    root: gtk::Box,
    editor: Rc<EditorModel>,
    settings: Rc<Settings>,
    arrangement: adw::ToggleGroup,
    updating: Rc<Cell<bool>>,
    modifiers_only: gtk::Button,
    note: gtk::Label,
    stack: gtk::Stack,
    board: gtk::Grid,
    more: gtk::Box,
    grouped: gtk::Box,
    shown: RefCell<Shown>,
}

#[derive(Default)]
struct Shown {
    board: Vec<BoardCap>,
    more: Vec<KeyUsage>,
    groups: Option<(Vec<KeyGroup>, Vec<KeyUsage>)>,
    modifiers: Vec<(ModifierKey, gtk::Button)>,
}

impl KeysPage {
    fn new(editor: Rc<EditorModel>, settings: Rc<Settings>) -> Self {
        let root = gtk::Box::new(gtk::Orientation::Vertical, 16);
        let top = gtk::Box::new(gtk::Orientation::Vertical, 8);
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        let arrangement = adw::ToggleGroup::new();
        arrangement.add(adw::Toggle::builder().name("qwerty").label("QWERTY").build());
        arrangement.add(adw::Toggle::builder().name("abc").label("ABC").build());
        arrangement.set_tooltip_text(Some(&t("Key Layout")));
        let modifiers_only = keycap(&t("Modifiers Only"));
        modifiers_only.set_tooltip_text(Some(&t("Makes the key hold these modifiers, as a modifier key does.")));
        modifiers_only.set_hexpand(true);
        modifiers_only.set_halign(gtk::Align::End);
        row.append(&arrangement);
        row.append(&modifiers_only);
        let note = caption("");
        top.append(&row);
        top.append(&note);
        root.append(&top);

        let board = gtk::Grid::builder().column_homogeneous(true).row_spacing(3).build();
        let more = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let qwerty = gtk::Box::new(gtk::Orientation::Vertical, 16);
        qwerty.append(&board);
        qwerty.append(&more);
        let grouped = gtk::Box::new(gtk::Orientation::Vertical, 16);
        let stack = gtk::Stack::builder().vhomogeneous(false).build();
        stack.add_named(&qwerty, Some("qwerty"));
        stack.add_named(&grouped, Some("abc"));
        root.append(&stack);

        let updating = Rc::new(Cell::new(false));
        {
            let (settings, updating, editor) = (settings.clone(), updating.clone(), editor.clone());
            arrangement.connect_active_name_notify(move |group| {
                if updating.get() {
                    return;
                }
                let name = group.active_name().map(|name| name.to_string());
                settings.update(|choices| choices.key_arrangement = name);
                // The choice is the interface's, not the model's; showing the page again draws it.
                editor.set_tab(EditorTab::Keys);
            });
        }
        {
            let editor = editor.clone();
            modifiers_only.connect_clicked(move |_| editor.assign_held_modifiers());
        }
        Self {
            root,
            editor,
            settings,
            arrangement,
            updating,
            modifiers_only,
            note,
            stack,
            board,
            more,
            grouped,
            shown: RefCell::default(),
        }
    }

    fn arrangement(&self) -> KeyArrangement {
        match self.settings.choices().key_arrangement.as_deref() {
            Some("qwerty") => KeyArrangement::Qwerty,
            Some("abc") => KeyArrangement::Alphabetical,
            _ => self.editor.vendor_key_arrangement(),
        }
    }

    fn refresh(&self) {
        let editor = &self.editor;
        let arrangement = self.arrangement();
        let name = if arrangement == KeyArrangement::Qwerty { "qwerty" } else { "abc" };
        self.updating.set(true);
        self.arrangement.set_active_name(Some(name));
        self.updating.set(false);
        self.stack.set_visible_child_name(name);

        let limit = editor.press_limit();
        self.note.set_label(&if limit == 0 {
            t("Modifiers apply to the next key.")
        } else if editor.modifiers_count_as_presses() {
            tf(
                "Modifiers apply to the next key. A key holds up to %lld presses, each modifier counting as one.",
                &[&limit],
            )
        } else {
            tf("Modifiers apply to the next key. A key holds up to %lld presses.", &[&limit])
        });
        let held = editor.held_modifiers();
        self.modifiers_only.set_sensitive(!held.is_empty());

        let mut shown = self.shown.borrow_mut();
        match arrangement {
            KeyArrangement::Qwerty => {
                let board = editor.key_board();
                let more = editor.more_keys();
                if shown.board != board || shown.more != more || shown.groups.is_some() || shown.modifiers.is_empty() {
                    shown.modifiers = self.build_board(&board, &more);
                    shown.board = board;
                    shown.more = more;
                    shown.groups = None;
                }
            }
            KeyArrangement::Alphabetical => {
                let groups = (editor.key_groups(), editor.shift_symbols());
                if shown.groups.as_ref() != Some(&groups) {
                    shown.modifiers = self.build_groups(&groups.0, &groups.1);
                    shown.groups = Some(groups);
                    shown.board.clear();
                }
            }
        }
        for (modifier, button) in &shown.modifiers {
            set_active(button, held.contains(modifier));
        }
    }

    fn modifier_button(&self, modifier: ModifierKey, compact: bool) -> gtk::Button {
        let button = keycap(modifier.label());
        if compact {
            button.add_css_class("compact");
        }
        let title = modifier_title(modifier);
        button.set_tooltip_text(Some(&title));
        button.update_property(&[gtk::accessible::Property::Label(&title)]);
        let editor = self.editor.clone();
        button.connect_clicked(move |_| editor.toggle(modifier));
        button
    }

    fn press_button(&self, key: &KeyUsage, compact: bool) -> gtk::Button {
        let button = key_button(key, compact);
        let (editor, key) = (self.editor.clone(), *key);
        button.connect_clicked(move |_| editor.press(&key));
        button
    }

    /// The keys as on a full-size keyboard, its modifier keys held for the next key. A key the keyboard isn't
    /// given stays in its place, off.
    fn build_board(&self, board: &[BoardCap], more: &[KeyUsage]) -> Vec<(ModifierKey, gtk::Button)> {
        while let Some(child) = self.board.first_child() {
            self.board.remove(&child);
        }
        let mut modifiers = vec![];
        for cap in board {
            let button = match cap.face {
                BoardFace::Key(key) => {
                    let button = self.press_button(&key, true);
                    if !cap.is_enabled {
                        button.set_sensitive(false);
                        button.set_tooltip_text(Some(&t("This keyboard doesn’t take this key.")));
                    }
                    button
                }
                BoardFace::Modifier(modifier) => {
                    let button = self.modifier_button(modifier, true);
                    modifiers.push((modifier, button.clone()));
                    button
                }
            };
            button.set_margin_start(1);
            button.set_margin_end(1);
            let quarters = |value: f64| (value * 4.0).round() as i32;
            self.board.attach(&button, quarters(cap.column), cap.row as i32, quarters(cap.width), 1);
        }
        clear(&self.more);
        if !more.is_empty() {
            let keys = wrap_box();
            for key in more {
                keys.append(&self.press_button(key, false));
            }
            self.more.append(&section(&t("More Keys"), None, &keys));
        }
        modifiers
    }

    /// The keys in groups, the letters in alphabetical order, under a row of modifiers to hold.
    fn build_groups(&self, groups: &[KeyGroup], shift_symbols: &[KeyUsage]) -> Vec<(ModifierKey, gtk::Button)> {
        clear(&self.grouped);
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        let mut modifiers = vec![];
        for modifier in ModifierKey::ALL {
            let button = self.modifier_button(modifier, false);
            button.set_size_request(58, -1);
            row.append(&button);
            modifiers.push((modifier, button));
        }
        self.grouped.append(&section(&t("Modifiers"), None, &row));
        for group in groups {
            let keys = wrap_box();
            for key in &group.keys {
                keys.append(&self.press_button(key, false));
            }
            self.grouped.append(&section(&key_group_title(group.kind), None, &keys));
        }
        let symbols = wrap_box();
        for key in shift_symbols {
            let Some(shifted) = key.shifted else { continue };
            let button = keycap(shifted);
            button.set_tooltip_text(Some(&format!("Shift+{}", key.label)));
            let (editor, key) = (self.editor.clone(), *key);
            button.connect_clicked(move |_| editor.press_shifted(&key));
            symbols.append(&button);
        }
        self.grouped.append(&section(&t("Shift Symbols"), None, &symbols));
        modifiers
    }
}

/// Media keys or mouse actions: a cap for each, the one the key does marked.
struct ChoicesPage<Choice> {
    root: gtk::Box,
    editor: Rc<EditorModel>,
    grid: gtk::FlowBox,
    width: i32,
    face: fn(Choice) -> (&'static str, String),
    assign: fn(&EditorModel, Choice),
    is_active: fn(&ActionCaption, Choice) -> bool,
    shown: RefCell<Option<(Vec<Choice>, Option<ActionCaption>)>>,
}

impl<Choice: Copy + PartialEq + 'static> ChoicesPage<Choice> {
    #[allow(clippy::too_many_arguments)]
    fn new(
        editor: Rc<EditorModel>,
        title: String,
        note: String,
        width: i32,
        face: fn(Choice) -> (&'static str, String),
        assign: fn(&EditorModel, Choice),
        is_active: fn(&ActionCaption, Choice) -> bool,
    ) -> Self {
        let grid = gtk::FlowBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .homogeneous(true)
            .min_children_per_line(2)
            .max_children_per_line(8)
            .row_spacing(8)
            .column_spacing(8)
            .build();
        let root = section(&title, Some(&note), &grid);
        Self { root, editor, grid, width, face, assign, is_active, shown: RefCell::default() }
    }

    fn refresh(&self, choices: Vec<Choice>) {
        let caption = self.editor.selected_caption();
        let state = (choices, caption);
        if self.shown.borrow().as_ref() == Some(&state) {
            return;
        }
        self.grid.remove_all();
        for choice in &state.0 {
            let (icon, title) = (self.face)(*choice);
            let button = tall_keycap(icon, &title);
            button.set_size_request(self.width, -1);
            set_active(&button, state.1.as_ref().is_some_and(|caption| (self.is_active)(caption, *choice)));
            let (editor, assign, choice) = (self.editor.clone(), self.assign, *choice);
            button.connect_clicked(move |_| assign(&editor, choice));
            self.grid.append(&button);
        }
        *self.shown.borrow_mut() = Some(state);
    }
}

/// The shown layer's lights, in what the keyboard takes.
struct LightingView {
    root: gtk::Box,
    editor: Rc<EditorModel>,
    shown: RefCell<Option<LightingPage>>,
}

impl LightingView {
    fn new(editor: Rc<EditorModel>) -> Self {
        Self { root: gtk::Box::new(gtk::Orientation::Vertical, 18), editor, shown: RefCell::default() }
    }

    fn refresh(&self) {
        let page = self.editor.lighting_page();
        if *self.shown.borrow() == page {
            return;
        }
        clear(&self.root);
        match &page {
            Some(LightingPage::Modes { modes, active }) => self.modes(modes, *active),
            Some(LightingPage::ColorAndMode { colors, modes, active_color, active_mode }) => {
                let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
                for color in colors {
                    let content = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                    let swatch = gtk::DrawingArea::builder()
                        .content_width(12)
                        .content_height(12)
                        .valign(gtk::Align::Center)
                        .build();
                    let css = led_color_css(*color);
                    swatch.set_draw_func(move |_, context, width, height| {
                        let rgba = gtk::gdk::RGBA::parse(css).unwrap_or(gtk::gdk::RGBA::BLACK);
                        context.set_source_rgba(
                            f64::from(rgba.red()),
                            f64::from(rgba.green()),
                            f64::from(rgba.blue()),
                            1.0,
                        );
                        let radius = f64::from(width.min(height)) / 2.0;
                        context.arc(
                            f64::from(width) / 2.0,
                            f64::from(height) / 2.0,
                            radius,
                            0.0,
                            std::f64::consts::TAU,
                        );
                        let _ = context.fill();
                    });
                    content.append(&swatch);
                    content.append(&gtk::Label::new(Some(&led_color_title(*color))));
                    let button = gtk::Button::builder().child(&content).build();
                    button.add_css_class("keycap");
                    set_active(&button, *active_color == Some(*color));
                    let (editor, color) = (self.editor.clone(), *color);
                    button.connect_clicked(move |_| editor.set_light_color(color));
                    row.append(&button);
                }
                self.root.append(&section(&t("Color"), None, &row));
                self.root.append(&self.effects(
                    modes,
                    *active_mode,
                    Some(&t("Lighting is set per layer; no key selection is needed.")),
                ));
            }
            Some(LightingPage::PerKeyColor { modes, active_mode, keys, selected }) => {
                self.root.append(&self.effects(modes, *active_mode, None));
                let row = gtk::Box::new(gtk::Orientation::Horizontal, 18);
                let palette = gtk::Grid::builder().row_spacing(5).column_spacing(5).build();
                for (index, color) in Rgb::PALETTE.iter().enumerate() {
                    let swatch = color_button(*color);
                    swatch.set_tooltip_text(Some(&color.hex()));
                    let (editor, color) = (self.editor.clone(), *color);
                    swatch.connect_clicked(move |_| editor.set_key_color(color));
                    palette.attach(&swatch, (index % 14) as i32, (index / 14) as i32, 1, 1);
                }
                row.append(&palette);
                if let Some(selected) = selected {
                    let preview = gtk::Box::new(gtk::Orientation::Vertical, 6);
                    let swatch = color_button(selected.color);
                    swatch.set_sensitive(false);
                    swatch.remove_css_class("swatch");
                    swatch.add_css_class("color-preview");
                    preview.append(&swatch);
                    let title = gtk::Label::new(Some(&control_title(selected.title)));
                    title.add_css_class("caption-heading");
                    preview.append(&title);
                    let hex = gtk::Label::new(Some(&selected.color.hex()));
                    hex.add_css_class("monospace");
                    hex.add_css_class("dim-label");
                    preview.append(&hex);
                    row.append(&preview);
                }
                let note = tf("Select one of keys 1–%lld on the keyboard, then pick its color.", &[keys]);
                self.root.append(&section(&t("Color"), Some(&note), &row));
            }
            None => {}
        }
        *self.shown.borrow_mut() = page;
    }

    /// The 1189:8890's modes. What a mode is set to isn't known until the user sets it — the keyboard can't be
    /// asked.
    fn modes(&self, modes: &[u8], active: Option<u8>) {
        let row = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(10).homogeneous(true).build();
        for mode in modes {
            let content = gtk::Box::new(gtk::Orientation::Vertical, 6);
            content.append(&gtk::Image::builder().icon_name(lighting_mode_icon(*mode)).pixel_size(20).build());
            let title = gtk::Label::new(Some(&lighting_mode_title(*mode, true)));
            title.add_css_class("heading");
            content.append(&title);
            if let Some(detail) = lighting_mode_detail(*mode) {
                let detail = caption(&detail);
                detail.set_justify(gtk::Justification::Center);
                detail.set_xalign(0.5);
                content.append(&detail);
            }
            content.set_margin_top(8);
            content.set_margin_bottom(8);
            let button = gtk::Button::builder().child(&content).build();
            button.add_css_class("keycap");
            set_active(&button, active == Some(*mode));
            let (editor, mode) = (self.editor.clone(), *mode);
            button.connect_clicked(move |_| editor.set_lighting_mode(mode));
            row.append(&button);
        }
        self.root.append(&section(&t("LED"), Some(&t("Lighting is set per layer; no key selection is needed.")), &row));
    }

    fn effects(&self, modes: &[u8], active: Option<u8>, note: Option<&str>) -> gtk::Box {
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        for mode in modes {
            let button = keycap(&lighting_mode_title(*mode, false));
            button.set_size_request(90, -1);
            set_active(&button, active == Some(*mode));
            let (editor, mode) = (self.editor.clone(), *mode);
            button.connect_clicked(move |_| editor.set_lighting_mode(mode));
            row.append(&button);
        }
        section(&t("Effect"), note, &row)
    }
}

/// A square of one colour, as a button.
fn color_button(color: Rgb) -> gtk::Button {
    let area = gtk::DrawingArea::builder().content_width(24).content_height(24).build();
    area.set_draw_func(move |_, context, width, height| {
        context.set_source_rgb(
            f64::from(color.red) / 255.0,
            f64::from(color.green) / 255.0,
            f64::from(color.blue) / 255.0,
        );
        context.rectangle(0.0, 0.0, f64::from(width), f64::from(height));
        let _ = context.fill();
    });
    let button = gtk::Button::builder().child(&area).overflow(gtk::Overflow::Hidden).build();
    button.add_css_class("swatch");
    button
}

/// Waits: before each press on the standard format, for the whole key on the legacy one.
struct DelayView {
    root: gtk::Box,
    editor: Rc<EditorModel>,
    /// The rows as last built — the presses' text — and their fields, so a changed number doesn't rebuild them
    /// under the user's typing.
    shown: RefCell<Option<(DelayShape, Vec<gtk::SpinButton>)>>,
    updating: Rc<Cell<bool>>,
}

#[derive(Clone, PartialEq)]
enum DelayShape {
    Unavailable,
    PerPress(Vec<String>),
    PerKey(bool),
}

impl DelayView {
    fn new(editor: Rc<EditorModel>) -> Self {
        Self {
            root: gtk::Box::new(gtk::Orientation::Vertical, 8),
            editor,
            shown: RefCell::default(),
            updating: Rc::default(),
        }
    }

    fn refresh(&self) {
        let page = self.editor.delay_page();
        let shape = match &page {
            DelayPage::Unavailable => DelayShape::Unavailable,
            DelayPage::PerPress { presses, .. } => {
                DelayShape::PerPress(presses.iter().map(|press| press.text.clone()).collect())
            }
            DelayPage::PerKey { value, .. } => DelayShape::PerKey(value.is_some()),
        };
        let values: Vec<u32> = match &page {
            DelayPage::PerPress { presses, .. } => presses.iter().map(|press| press.delay).collect(),
            DelayPage::PerKey { value, .. } => value.iter().copied().collect(),
            DelayPage::Unavailable => vec![],
        };
        if self.shown.borrow().as_ref().map(|(shown, _)| shown) != Some(&shape) {
            let fields = self.build(&page);
            *self.shown.borrow_mut() = Some((shape, fields));
        }
        self.updating.set(true);
        if let Some((_, fields)) = self.shown.borrow().as_ref() {
            for (field, value) in fields.iter().zip(values) {
                if field.value() as u32 != value {
                    field.set_value(f64::from(value));
                }
            }
        }
        self.updating.set(false);
    }

    fn build(&self, page: &DelayPage) -> Vec<gtk::SpinButton> {
        clear(&self.root);
        let mut fields = vec![];
        match page {
            DelayPage::PerPress { range, presses } => {
                let content: gtk::Widget = if presses.is_empty() {
                    dim(&t("The selected key has no presses.")).upcast()
                } else {
                    let grid = gtk::Grid::builder().column_spacing(14).row_spacing(6).build();
                    for (row, press) in presses.iter().enumerate() {
                        grid.attach(&dim(&tf("Press %lld", &[&(press.index + 1)])), 0, row as i32, 1, 1);
                        let text = gtk::Label::builder().label(&press.text).xalign(0.0).width_request(90).build();
                        text.add_css_class("heading");
                        grid.attach(&text, 1, row as i32, 1, 1);
                        let index = press.index;
                        let (field, controls) =
                            self.milliseconds(range.clone(), move |editor, value| editor.set_delay(index, value));
                        grid.attach(&controls, 2, row as i32, 1, 1);
                        fields.push(field);
                    }
                    grid.upcast()
                };
                let note = t("Each press can wait before it is sent. Give the key its presses on the Keys page first.");
                self.root.append(&section(&t("Delay"), Some(&note), &content));
            }
            DelayPage::PerKey { range, value } => {
                let content: gtk::Widget = if value.is_some() {
                    let (field, controls) =
                        self.milliseconds(range.clone(), |editor, value| editor.set_key_delay(value));
                    fields.push(field);
                    controls.upcast()
                } else {
                    dim(&t("Select a key or knob.")).upcast()
                };
                self.root.append(&section(
                    &t("Delay"),
                    Some(&t("One wait for the whole key. Its presses stay.")),
                    &content,
                ));
            }
            DelayPage::Unavailable => {}
        }
        fields
    }

    /// A number of milliseconds, kept in `range`.
    fn milliseconds(
        &self,
        range: std::ops::RangeInclusive<u32>,
        commit: impl Fn(&EditorModel, u32) + 'static,
    ) -> (gtk::SpinButton, gtk::Box) {
        let adjustment =
            gtk::Adjustment::new(0.0, f64::from(*range.start()), f64::from(*range.end()), 10.0, 100.0, 0.0);
        let field = gtk::SpinButton::builder().adjustment(&adjustment).numeric(true).width_chars(6).build();
        let (editor, updating) = (self.editor.clone(), self.updating.clone());
        field.connect_value_changed(move |field| {
            if !updating.get() {
                commit(&editor, field.value() as u32);
            }
        });
        let controls = gtk::Box::new(gtk::Orientation::Horizontal, 4);
        controls.append(&field);
        controls.append(&dim("ms"));
        (field, controls)
    }
}

fn dim(text: &str) -> gtk::Label {
    let label = gtk::Label::builder().label(text).xalign(0.0).build();
    label.add_css_class("dim-label");
    label
}
