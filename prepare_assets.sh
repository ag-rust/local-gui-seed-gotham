#!/bin/sh
if [ ! -d gui/assets ]; then
	mkdir gui/assets
fi

cp gui/pkg/gui.js gui/assets
cp gui/pkg/gui_bg.wasm gui/assets
cp gui/index.html gui/assets
cp gui/wing.min.css gui/assets
