use iced::{
    Element,
    widget::{canvas, container},
};
use noise::NoiseFn;
use vizkit::{
    draw::{ArrowAttrs, vector_iter},
    scale::ScaleContinuous,
};

enum Message {}

struct Arrow {
    values: Vec<[f64; 2]>,
}

fn noise(x: f64, y: f64) -> f64 {
    noise::Perlin::default().get([x, y])
}

fn build_arrow(points: [[f32; 2]; 5], radius: f32) -> canvas::Path {
    let [p1, pi, p2, p3, p4] = points;
    canvas::Path::new(|builder: &mut canvas::path::Builder| {
        builder.move_to(p1.into());

        if radius < 1e5 {
            builder.arc_to(pi.into(), p2.into(), radius);
        } else {
            builder.line_to(p2.into());
        }

        builder.move_to(p3.into());
        builder.line_to(p2.into());
        builder.line_to(p4.into());
    })
}

impl canvas::Program<Message> for Arrow {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<iced::Renderer>> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        let width = bounds.width;
        let height = bounds.height;
        let x_scale = ScaleContinuous::linear()
            .domain([0., 2.])
            .range([0., width]);
        let y_scale = ScaleContinuous::linear()
            .domain([0., 2.])
            .range([height, 0.]);

        vector_iter(
            &self.values,
            |&[x, _]| x_scale.apply(x as f32),
            |&[_, y]| y_scale.apply(y as f32),
            |&[x, y]| ((noise(x + 2., y) + 0.5) * 24.) as f32,
            |&[x, y]| (noise(x, y) * 360.).to_radians() as f32,
            |_| ArrowAttrs::default(),
        )
        .for_each(|arrow_props| {
            let arrow = build_arrow(arrow_props.points, arrow_props.radius);
            let stroke_color: [f32; 3] = arrow_props.stroke_color.into();
            frame.stroke(
                &arrow,
                canvas::Stroke::default()
                    .with_color(iced::Color::from(stroke_color))
                    .with_width(arrow_props.stroke_opacity),
            );
        });

        vec![frame.into_geometry()]
    }
}

#[derive(Default)]
struct App;

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<'_, Message> {
        let rmin = (((2. * 2.) / (4000. * 1.5)) as f64).sqrt();
        let values = poisson_diskus::bridson(&[2., 2.], rmin, 30, false).unwrap();
        container(
            canvas(Arrow { values })
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
        )
        .center(iced::Length::Fill)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .antialiasing(true)
        .run()
}
