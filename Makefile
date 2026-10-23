# MINIKeyboard for Linux — build, test, check, try, install.
#
# Needs Rust 1.88 or later, and the development files of GTK 4, libadwaita 1.7 or later, libsoup 3 and libusb 1.0,
# with gettext's msgfmt for the translations.

PREFIX  ?= /usr/local
APP_ID  := me.hobbyworker.MINIKeyboard
LOCALES := target/locale
SHARE    = $(DESTDIR)$(PREFIX)/share

.PHONY: build test check locale run install install-locale help
.DEFAULT_GOAL := help

help:  ## the commands
	@awk 'BEGIN{FS=":.*## "} /^[a-z][a-z-]*:.*## /{printf "  %-16s %s\n",$$1,$$2}' $(MAKEFILE_LIST)

build:  ## a release build for PREFIX — target/release/minikeyboard, looking for its translations under PREFIX
	MINIKEYBOARD_LOCALE_DIR=$(PREFIX)/share/locale cargo build --release --locked

test:  ## every crate's tests — the app doesn't open a window or a keyboard
	cargo test --workspace

check:  ## formatting and lints; warnings are errors
	cargo fmt --all --check
	cargo clippy --workspace --all-targets -- -D warnings

locale:  ## the translations, compiled into target/locale
	@for po in po/*.po; do \
		name=$$(basename $$po .po); mkdir -p $(LOCALES)/$$name/LC_MESSAGES; \
		msgfmt -o $(LOCALES)/$$name/LC_MESSAGES/minikeyboard.mo $$po; \
	done

run: locale  ## a debug build with its translations — it has the Simulator menu
	MINIKEYBOARD_LOCALE_DIR=$(LOCALES) cargo run

install: install-locale  ## after `make build`: the program, translations, desktop file, metainfo, icons and licenses, into DESTDIR/PREFIX
	install -Dm755 target/release/minikeyboard $(DESTDIR)$(PREFIX)/bin/minikeyboard
	install -Dm644 data/$(APP_ID).desktop -t $(SHARE)/applications
	install -Dm644 data/$(APP_ID).metainfo.xml -t $(SHARE)/metainfo
	install -Dm644 data/icons/hicolor/scalable/apps/$(APP_ID).svg -t $(SHARE)/icons/hicolor/scalable/apps
	install -Dm644 data/icons/hicolor/symbolic/apps/$(APP_ID)-symbolic.svg -t $(SHARE)/icons/hicolor/symbolic/apps
	install -Dm644 LICENSE LICENSES/CC-BY-SA-4.0.txt -t $(SHARE)/licenses/$(APP_ID)

install-locale: locale  ## the translations, into DESTDIR/PREFIX/share/locale
	@for directory in $(LOCALES)/*; do \
		install -Dm644 $$directory/LC_MESSAGES/minikeyboard.mo $(SHARE)/locale/$$(basename $$directory)/LC_MESSAGES/minikeyboard.mo; \
	done
