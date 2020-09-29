use iced::{
    canvas::layer::Cache,
    canvas::{self, layer, Canvas, Drawable, Fill, Frame, Path},
    executor,
    Application, Container, Element, Length, Point, Size, Color, Row, Command,
};
use sharmat::*;

#[derive(Debug)]
pub struct Sharmat {
    board: GBoard,
}

#[derive(Debug)]
pub enum SharmatMessage {}

type Message = SharmatMessage;

#[derive(Debug)]
pub struct GBoard {
    width: usize,
    height: usize,
    pub board: Board,
    pub cache: Cache<Self>,
    pub fill_dark: Fill,
    pub fill_light: Fill,
}

impl Application for Sharmat {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self { board: GBoard {
            width: 8,
            height: 8,
            board: Board,
            cache: Cache::new(),
            fill_dark: Fill::Color(Color::from_rgb8(226, 149, 120)),
            fill_light: Fill::Color(Color::from_rgb8(255, 221, 210))
        }}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Sharmat")
    }

    fn view(&mut self) -> Element<Self::Message> {
        Container::new(
            Row::new()
                .push(self.board.view())
        )
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}

impl GBoard {
    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn view<'a>(&'a mut self) -> Element<'a, Message> {
        Canvas::new()
            .push(self.cache.with(self))
            .width(Length::Units(400))
            .height(Length::Units(400))
            .into()
    }
}

impl Drawable for GBoard {
    fn draw(&self, frame: &mut Frame) {
        let mut tile_size =
            (frame.width() / self.width as f32).min(frame.height() / self.height as f32);

        let dark_tiles_path = Path::new(|p| {
            for y in 0..self.height {
                for x in 0..self.width {
                    if (x + y) % 2 == 0 {
                        p.rectangle(
                            Point::new(x as f32 * tile_size, y as f32 * tile_size),
                            Size::new(tile_size, tile_size),
                        );
                    }
                }
            }
        });

        frame.fill(&Path::new(|p| p.rectangle(Point::ORIGIN, Size::new(self.width() as f32 * tile_size, self.height() as f32 * tile_size))), self.fill_light);
        frame.fill(&dark_tiles_path, self.fill_dark);
    }
}
