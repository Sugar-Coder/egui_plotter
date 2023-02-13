# egui plotter
use egui version 0.20.0

## dependencies
- eframe - wasm
- rfd - upload file dialog
- chrono - handle date data
- calamine - read excel file
- futures - for test locally of the rfd.AsyncFileDialog

## 问题
1. Done rfd只支持async的OpenDialog
参考[egui issue](https://github.com/emilk/egui/issues/270), [calamine issue](https://github.com/tafia/calamine/issues/291), [rfd example](https://github.com/PolyMeilex/rfd/blob/master/examples/async.rs)解决

2. 需要点击两次Open才能生成
原因：try_recv是非阻塞的，并且``if ui.button() ``只有被点击后才会执行之后的代码。
```rust
if ui.button("Open File").clicked() {
    // ...
    match self.data_channel.1.try_recv() {
        // ...
    }
}
```
解决：
将``match``语句移出``if``语句block。
在closure中使用ctx.request_repaint, 唤起UI线程

3. eframe的 web cache 问题
参考[issue](https://github.com/emilk/eframe_template/issues/84)