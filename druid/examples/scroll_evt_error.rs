// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use druid::widget::{List, Scroll, Switch, WidgetExt};
use druid::{AppLauncher, Widget, WindowDesc};
use im::{vector, Vector};

fn build_widget() -> impl Widget<Vector<bool>> {
    Scroll::new(List::new(|| Switch::new()))
        .vertical()
        .center()
        .debug_widget_id()
}

pub fn main() {
    let window = WindowDesc::new(build_widget).title("Scroll evt error");
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(vector![false, false])
        .expect("launch failed");
}
