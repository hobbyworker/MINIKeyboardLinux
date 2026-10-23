//! The reports written to the keyboard and its answers, for checking what a keyboard does. A report shows on
//! one line up to the zero padding at its end; opened, it shows every byte.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::time::UNIX_EPOCH;

use adw::prelude::*;
use presentation::{DeviceLogModel, LogDirection, LogLine};

use crate::i18n::{t, tf};
use crate::names::log_event_message;

pub struct LogWindow {
    pub window: adw::Window,
    model: Rc<DeviceLogModel>,
    count: gtk::Label,
    expand_all: gtk::Button,
    copy: gtk::Button,
    clear: gtk::Button,
    stack: gtk::Stack,
    list: gtk::ListBox,
    scroller: gtk::ScrolledWindow,
    rows: RefCell<Vec<Row>>,
    expanded: RefCell<HashSet<u64>>,
}

struct Row {
    id: u64,
    widget: gtk::ListBoxRow,
    detail: gtk::Label,
    chevron: gtk::Button,
    can_expand: bool,
}

impl LogWindow {
    pub fn new(model: Rc<DeviceLogModel>) -> Rc<Self> {
        let count = gtk::Label::builder().xalign(0.0).hexpand(true).build();
        count.add_css_class("dim-label");
        let expand_all = gtk::Button::with_label(&t("Expand All"));
        let copy = gtk::Button::with_label(&t("Copy"));
        let clear = gtk::Button::with_label(&t("Clear"));
        let bar = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(8)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(10)
            .margin_end(10)
            .build();
        bar.append(&count);
        bar.append(&expand_all);
        bar.append(&copy);
        bar.append(&clear);

        let list = gtk::ListBox::builder().selection_mode(gtk::SelectionMode::None).build();
        list.add_css_class("navigation-sidebar");
        let scroller = gtk::ScrolledWindow::builder().child(&list).vexpand(true).build();
        let empty = adw::StatusPage::builder()
            .title(t("No packets yet."))
            .icon_name("network-transmit-receive-symbolic")
            .build();
        let stack = gtk::Stack::new();
        stack.add_named(&empty, Some("empty"));
        stack.add_named(&scroller, Some("list"));

        let content = gtk::Box::new(gtk::Orientation::Vertical, 0);
        content.append(&bar);
        content.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
        content.append(&stack);
        let toolbar = adw::ToolbarView::new();
        toolbar.add_top_bar(&adw::HeaderBar::new());
        toolbar.set_content(Some(&content));
        let window = adw::Window::builder()
            .title(t("Device Log"))
            .default_width(760)
            .default_height(420)
            .content(&toolbar)
            .hide_on_close(true)
            .build();

        let log = Rc::new(Self {
            window,
            model: model.clone(),
            count,
            expand_all,
            copy,
            clear,
            stack,
            list,
            scroller,
            rows: RefCell::default(),
            expanded: RefCell::default(),
        });
        {
            let log = Rc::downgrade(&log);
            model.subscribe(move || {
                if let Some(log) = log.upgrade() {
                    log.refresh();
                }
            });
        }
        {
            let weak = Rc::downgrade(&log);
            log.expand_all.connect_clicked(move |_| {
                if let Some(log) = weak.upgrade() {
                    log.toggle_all();
                }
            });
        }
        {
            let weak = Rc::downgrade(&log);
            log.copy.connect_clicked(move |button| {
                if let Some(log) = weak.upgrade() {
                    button.clipboard().set_text(&copy_text(&log.model.lines()));
                }
            });
        }
        {
            let weak = Rc::downgrade(&log);
            log.clear.connect_clicked(move |_| {
                if let Some(log) = weak.upgrade() {
                    log.expanded.borrow_mut().clear();
                    log.model.clear();
                }
            });
        }
        log.refresh();
        log
    }

    /// Adds the lines that came and drops the ones the log let go, rather than drawing every line again.
    fn refresh(self: &Rc<Self>) {
        let lines = self.model.lines();
        let mut rows = self.rows.borrow_mut();
        let first = lines.first().map(|line| line.id);
        while rows.first().is_some_and(|row| first.is_none_or(|first| row.id < first)) {
            let row = rows.remove(0);
            self.list.remove(&row.widget);
        }
        let last = rows.last().map(|row| row.id);
        let mut added = false;
        for line in lines.iter().filter(|line| last.is_none_or(|last| line.id > last)) {
            let row = self.make_row(line);
            self.list.append(&row.widget);
            rows.push(row);
            added = true;
        }
        drop(rows);
        self.count.set_label(&tf("Packets: %lld", &[&lines.len()]));
        self.stack.set_visible_child_name(if lines.is_empty() { "empty" } else { "list" });
        self.copy.set_sensitive(!lines.is_empty());
        self.clear.set_sensitive(!lines.is_empty());
        self.update_expand_all();
        if added {
            let scroller = self.scroller.clone();
            gtk::glib::idle_add_local_once(move || {
                let adjustment = scroller.vadjustment();
                adjustment.set_value(adjustment.upper());
            });
        }
    }

    fn make_row(self: &Rc<Self>, line: &LogLine) -> Row {
        let can_expand = line.event.is_none() && (line.is_shortened() || line.bytes.len() > 16);
        let header = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let chevron = gtk::Button::builder()
            .icon_name("pan-end-symbolic")
            .opacity(if can_expand { 1.0 } else { 0.0 })
            .sensitive(can_expand)
            .build();
        chevron.add_css_class("flat");
        chevron.add_css_class("circular");
        chevron.set_tooltip_text(Some(&t("Expand")));
        header.append(&chevron);
        let time = gtk::Label::new(Some(&time_text(line)));
        time.add_css_class("dim-label");
        header.append(&time);
        let icon = gtk::Image::from_icon_name(match line.direction {
            LogDirection::Output => "go-up-symbolic",
            LogDirection::Input => "go-down-symbolic",
            LogDirection::Event => "help-about-symbolic",
        });
        match line.direction {
            LogDirection::Output => icon.add_css_class("log-output"),
            LogDirection::Input => icon.add_css_class("log-input"),
            LogDirection::Event => icon.add_css_class("dim-label"),
        }
        header.append(&icon);
        let body = match &line.event {
            Some(event) => log_event_message(event),
            None => line.summary(),
        };
        header.append(&gtk::Label::builder().label(body).xalign(0.0).selectable(true).build());
        if line.is_shortened() {
            let more = gtk::Label::new(Some(&format!("… {}", tf("%lld bytes", &[&line.bytes.len()]))));
            more.add_css_class("dim-label");
            header.append(&more);
        }
        let detail = gtk::Label::builder()
            .label(line.rows().join("\n"))
            .xalign(0.0)
            .selectable(true)
            .margin_start(34)
            .visible(false)
            .build();
        detail.add_css_class("dim-label");
        let column = gtk::Box::new(gtk::Orientation::Vertical, 4);
        column.append(&header);
        column.append(&detail);
        column.add_css_class("log-line");
        let widget = gtk::ListBoxRow::builder().child(&column).activatable(false).build();
        let id = line.id;
        {
            let weak = Rc::downgrade(self);
            chevron.connect_clicked(move |_| {
                if let Some(log) = weak.upgrade() {
                    let open = !log.expanded.borrow().contains(&id);
                    log.set_expanded(id, open);
                    log.update_expand_all();
                }
            });
        }
        let row = Row { id, widget, detail, chevron, can_expand };
        let open = self.expanded.borrow().contains(&id);
        show_expanded(&row, open);
        row
    }

    fn set_expanded(&self, id: u64, open: bool) {
        if open {
            self.expanded.borrow_mut().insert(id);
        } else {
            self.expanded.borrow_mut().remove(&id);
        }
        if let Some(row) = self.rows.borrow().iter().find(|row| row.id == id) {
            show_expanded(row, open);
        }
    }

    fn all_expanded(&self) -> bool {
        let rows = self.rows.borrow();
        let expandable: Vec<u64> = rows.iter().filter(|row| row.can_expand).map(|row| row.id).collect();
        !expandable.is_empty() && expandable.iter().all(|id| self.expanded.borrow().contains(id))
    }

    fn toggle_all(&self) {
        let open = !self.all_expanded();
        let ids: Vec<u64> = self.rows.borrow().iter().filter(|row| row.can_expand).map(|row| row.id).collect();
        for id in ids {
            self.set_expanded(id, open);
        }
        self.update_expand_all();
    }

    fn update_expand_all(&self) {
        let any = self.rows.borrow().iter().any(|row| row.can_expand);
        self.expand_all.set_label(&if self.all_expanded() { t("Collapse All") } else { t("Expand All") });
        self.expand_all.set_sensitive(any);
    }
}

fn show_expanded(row: &Row, open: bool) {
    let open = open && row.can_expand;
    row.detail.set_visible(open);
    row.chevron.set_icon_name(if open { "pan-down-symbolic" } else { "pan-end-symbolic" });
    row.chevron.set_tooltip_text(Some(&if open { t("Collapse") } else { t("Expand") }));
}

/// "14:03:27.412", in local time.
fn time_text(line: &LogLine) -> String {
    let since = line.time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let seconds = gtk::glib::DateTime::from_unix_local(since.as_secs() as i64)
        .ok()
        .and_then(|time| time.format("%H:%M:%S").ok())
        .map(|text| text.to_string())
        .unwrap_or_default();
    format!("{seconds}.{:03}", since.subsec_millis())
}

/// Every line, every byte of each report.
fn copy_text(lines: &[LogLine]) -> String {
    lines
        .iter()
        .map(|line| {
            let mark = match line.direction {
                LogDirection::Output => "OUT",
                LogDirection::Input => "IN ",
                LogDirection::Event => "-- ",
            };
            let body = line.event.as_ref().map(log_event_message).unwrap_or_else(|| line.hex());
            format!("{} {mark} {body}", time_text(line))
        })
        .collect::<Vec<_>>()
        .join("\n")
}
