# egui plotter
use egui version 0.20.0

## dependencies
eframe - wasm
rfd - upload file
chrono - handle date data
calamine - read excel file
futures - for test locally of the rfd.AsyncFileDialog

## 问题
Done rfd只支持async的OpenDialog
参考[issue](https://github.com/emilk/egui/issues/270)解决
需要点击两次Open才能打开: 解决：在closure中使用ctx.request_repaint