use egui::{Color32, Rect, ScrollArea, Vec2};
use vizkit::chromatic::{
    Cividis, ColorMap, Diverging, DivergingSpace, Rainbow, Sequential, SequentialSpace, Sinebow,
    Turbo, Viridis, ViridisSpace, WarmCold, WarmColdSpace,
};
use vizkit::scale::ScaleColor;

fn show_color_map<C: ColorMap + Clone>(ui: &mut egui::Ui, title: &str, color_map: C) {
    ui.label(title);

    let (rect, _response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), egui::Sense::hover());

    let painter = ui.painter();
    let width = rect.width() as usize;
    let height = rect.height();
    let interpolator = ScaleColor::linear(color_map).domain([0., width as f32]);
    let rect_width = 2;

    for t in (0..width).step_by(rect_width) {
        let [r, g, b] = interpolator.apply::<[f32; 3]>(t as f32);
        let color = Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8);

        painter.rect_filled(
            Rect::from_min_size(
                rect.min + Vec2::new(t as f32, 0.0),
                Vec2::new(rect_width as f32, height),
            ),
            0.0,
            color,
        );
    }
}

macro_rules! show_color_map {
    ($ui:expr, $color_map:expr) => {
        show_color_map($ui, stringify!($color_map), $color_map)
    };
}

struct App;

impl Default for App {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Body);

            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        show_color_map!(ui, Viridis::new(&ViridisSpace::Viridis));
                        show_color_map!(ui, Viridis::new(&ViridisSpace::Inferno));
                        show_color_map!(ui, Viridis::new(&ViridisSpace::Magma));
                        show_color_map!(ui, Viridis::new(&ViridisSpace::Plasma));
                        show_color_map!(ui, Rainbow::default());
                        show_color_map!(ui, Turbo::default());
                        show_color_map!(ui, Cividis::default());
                        show_color_map!(ui, Sinebow::default());
                        show_color_map!(ui, WarmCold::new(WarmColdSpace::Warm));
                        show_color_map!(ui, WarmCold::new(WarmColdSpace::Cold));
                        show_color_map!(ui, Diverging::new(DivergingSpace::BrBg));
                        show_color_map!(ui, Diverging::new(DivergingSpace::PiYg));
                        show_color_map!(ui, Diverging::new(DivergingSpace::PrGn));
                        show_color_map!(ui, Diverging::new(DivergingSpace::PuOr));
                        show_color_map!(ui, Diverging::new(DivergingSpace::RdBu));
                        show_color_map!(ui, Diverging::new(DivergingSpace::RdGy));
                        show_color_map!(ui, Diverging::new(DivergingSpace::RdYlBu));
                        show_color_map!(ui, Diverging::new(DivergingSpace::RdYlGn));
                        show_color_map!(ui, Diverging::new(DivergingSpace::Spectral));
                        show_color_map!(ui, Sequential::new(SequentialSpace::Blues));
                        show_color_map!(ui, Sequential::new(SequentialSpace::Greens));
                        show_color_map!(ui, Sequential::new(SequentialSpace::Greys));
                        show_color_map!(ui, Sequential::new(SequentialSpace::Oranges));
                        show_color_map!(ui, Sequential::new(SequentialSpace::Purples));
                        show_color_map!(ui, Sequential::new(SequentialSpace::Reds));
                        show_color_map!(ui, Sequential::new(SequentialSpace::BuGn));
                        show_color_map!(ui, Sequential::new(SequentialSpace::BuPu));
                        show_color_map!(ui, Sequential::new(SequentialSpace::GnBu));
                        show_color_map!(ui, Sequential::new(SequentialSpace::OrRd));
                        show_color_map!(ui, Sequential::new(SequentialSpace::PuBu));
                        show_color_map!(ui, Sequential::new(SequentialSpace::PuBuGn));
                        show_color_map!(ui, Sequential::new(SequentialSpace::PuRd));
                        show_color_map!(ui, Sequential::new(SequentialSpace::RdPu));
                        show_color_map!(ui, Sequential::new(SequentialSpace::YlGn));
                        show_color_map!(ui, Sequential::new(SequentialSpace::YlGnBu));
                        show_color_map!(ui, Sequential::new(SequentialSpace::YlOrBr));
                        show_color_map!(ui, Sequential::new(SequentialSpace::YlOrRd));
                    });
                });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Color Maps Viewer",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}
