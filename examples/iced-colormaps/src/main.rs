use std::sync::Arc;

use iced::Length;
use iced::widget::{container, scrollable};
use iced::{
    Color, Element, Point, Size,
    widget::{canvas, column, text},
};
use vizkit::chromatic::{
    Cividis, ColorMap, Diverging, DivergingSpace, Rainbow, Sequential, SequentialSpace, Sinebow,
    Turbo, Viridis, ViridisSpace, WarmCold, WarmColdSpace,
};
use vizkit::scale::ScaleColor;

#[derive(Default)]
struct App;

enum Message {}

struct ColorMapDraw<C: ColorMap> {
    color_map: C,
}

impl<Message, C: ColorMap + Clone> canvas::Program<Message> for ColorMapDraw<C> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let width = bounds.width;
        let height = bounds.height;
        let interpolator = ScaleColor::linear(self.color_map.clone()).domain([0., width as f32]);
        let rect_width = 2;
        for t in (0..width as usize).step_by(rect_width) {
            frame.fill_rectangle(
                Point::new(t as f32, 0.),
                Size::new(rect_width as f32, height),
                Color::from(interpolator.apply::<[f32; 3]>(t as f32)),
            );
        }
        vec![frame.into_geometry()]
    }
}

fn show_color_map<'a, C: ColorMap + Clone + 'a>(
    title: &'a str,
    color_map: C,
) -> Element<'a, Message> {
    column![
        text(title),
        canvas(ColorMapDraw { color_map })
            .width(Length::Fill)
            .height(30)
    ]
    .into()
}

macro_rules! show_color_map {
    ($space:expr) => {
        show_color_map(stringify!($space), $space)
    };
}

impl App {
    fn update(&mut self, _: Message) {}
    fn view(&self) -> Element<'_, Message> {
        scrollable(
            container(column![
                show_color_map!(Viridis::new(&ViridisSpace::Viridis)),
                show_color_map!(Viridis::new(&ViridisSpace::Inferno)),
                show_color_map!(Viridis::new(&ViridisSpace::Magma)),
                show_color_map!(Viridis::new(&ViridisSpace::Plasma)),
                show_color_map!(Rainbow::default()),
                show_color_map!(Turbo::default()),
                show_color_map!(Cividis::default()),
                show_color_map!(Sinebow::default()),
                show_color_map!(WarmCold::new(WarmColdSpace::Warm)),
                show_color_map!(WarmCold::new(WarmColdSpace::Cold)),
                show_color_map!(Diverging::new(DivergingSpace::BrBg)),
                show_color_map!(Diverging::new(DivergingSpace::PiYg)),
                show_color_map!(Diverging::new(DivergingSpace::PrGn)),
                show_color_map!(Diverging::new(DivergingSpace::PuOr)),
                show_color_map!(Diverging::new(DivergingSpace::RdBu)),
                show_color_map!(Diverging::new(DivergingSpace::RdGy)),
                show_color_map!(Diverging::new(DivergingSpace::RdYlBu)),
                show_color_map!(Diverging::new(DivergingSpace::RdYlGn)),
                show_color_map!(Diverging::new(DivergingSpace::Spectral)),
                show_color_map!(Sequential::new(SequentialSpace::Blues)),
                show_color_map!(Sequential::new(SequentialSpace::Greens)),
                show_color_map!(Sequential::new(SequentialSpace::Greys)),
                show_color_map!(Sequential::new(SequentialSpace::Oranges)),
                show_color_map!(Sequential::new(SequentialSpace::Purples)),
                show_color_map!(Sequential::new(SequentialSpace::Reds)),
                show_color_map!(Sequential::new(SequentialSpace::BuGn)),
                show_color_map!(Sequential::new(SequentialSpace::BuPu)),
                show_color_map!(Sequential::new(SequentialSpace::GnBu)),
                show_color_map!(Sequential::new(SequentialSpace::OrRd)),
                show_color_map!(Sequential::new(SequentialSpace::PuBu)),
                show_color_map!(Sequential::new(SequentialSpace::PuBuGn)),
                show_color_map!(Sequential::new(SequentialSpace::PuRd)),
                show_color_map!(Sequential::new(SequentialSpace::RdPu)),
                show_color_map!(Sequential::new(SequentialSpace::YlGn)),
                show_color_map!(Sequential::new(SequentialSpace::YlGnBu)),
                show_color_map!(Sequential::new(SequentialSpace::YlOrBr)),
                show_color_map!(Sequential::new(SequentialSpace::YlOrRd)),
            ])
            .padding(20),
        )
        .into()
    }
}

fn theme_fn(_app: &App) -> Option<iced::Theme> {
    Some(iced::Theme::Custom(Arc::new(iced::theme::Custom::new(
        "CustomTheme".to_string(),
        iced::theme::Palette {
            background: Color::from_rgb(19. / 255., 20. / 255., 22. / 255.),
            text: Color::WHITE,
            primary: Color::WHITE,
            success: Color::from_rgb(0., 1., 0.),
            warning: Color::from_rgb(1., 0.5, 0.),
            danger: Color::from_rgb(1., 0., 0.),
        },
    ))))
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(theme_fn)
        .run()
}
