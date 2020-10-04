use std::num::NonZeroUsize;
use iced::{
    canvas::layer::Cache,
    canvas::{Canvas, Drawable, Fill, Frame, Path},
    executor,
    Application, Container, Element, Length, Point, Size, Color, Row, Command, Background,
};
use iced_native::{
    widget::{Widget, svg::Svg},
    layout,
    MouseCursor, Rectangle,
};
use iced_wgpu::{Renderer, Primitive, Defaults};
use sharmat::{
    board::*,
};
use super::style::SharmatStyleSheet;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::hash::Hash;

/// Main window model
#[derive(Debug)]
pub struct Sharmat<'a> {
    board: Rc<RefCell<Board<'a>>>,
    stylesheet: SharmatStyleSheet,
    pieces: Rc<HashMap<String, Svg>>,
}

/// Message enum for user interaction
#[derive(Debug)]
pub enum SharmatMessage {}
type Message = SharmatMessage;

/// Graphical board (visible representation of the board)
#[derive(Debug)]
pub struct GBoard<'a> {
    pub board: Rc<RefCell<Board<'a>>>,
    // pub cache: Cache<Self>,
    pub fill_dark: Color,
    pub fill_light: Color,
    pieces: Rc<HashMap<String, Svg>>,
}

impl<'a> Application for Sharmat<'a> {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = (HashMap<String, Svg>);

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {
            board: Rc::new(RefCell::new(Board::new(NonZeroUsize::new(8).unwrap(), NonZeroUsize::new(8).unwrap()))),
            stylesheet: SharmatStyleSheet::default(),
            pieces: Rc::new(flags),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Sharmat")
    }

    fn view(&mut self) -> Element<Self::Message> {
        Container::new(
            Row::new()
                .push(
                    Container::new::<iced_native::Element<_, _>>(GBoard::new(self.board.clone(), self.pieces.clone()).into())
                    .width(Length::Units(600))
                    .height(Length::Units(600))
                    .padding(10)
                )
        )
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(self.stylesheet)
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}

impl<'a> GBoard<'a> {
    pub fn new(board: Rc<RefCell<Board<'a>>>, pieces: Rc<HashMap<String, Svg>>) -> GBoard<'a> {
        GBoard {
            board,
            fill_dark: Color::from_rgb8(226, 149, 120),
            fill_light: Color::from_rgb8(255, 221, 210),
            pieces,
        }
    }

    #[inline]
    pub fn get_board_width(&self) -> usize {
        self.board.borrow().width.get()
    }

    #[inline]
    pub fn get_board_height(&self) -> usize {
        self.board.borrow().height.get()
    }

    #[inline]
    pub fn tile_size(&self, width: f32, height: f32) -> f32 {
        (width / self.get_board_width() as f32).min(height / self.get_board_height() as f32)
    }
}

impl<'a, Message> Widget<Message, Renderer> for GBoard<'a> {
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Fill
    }

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::new(
            self.tile_size(limits.max().width, limits.max().height) * self.get_board_width() as f32,
            self.tile_size(limits.max().width, limits.max().height) * self.get_board_height() as f32,
        ))
    }

    fn hash_layout(&self, hasher: &mut iced_native::Hasher) {
        self.board.borrow().hash(hasher);
    }

    fn draw(&self, renderer: &mut Renderer, _defaults: &Defaults, layout: layout::Layout<'_>, mouse: Point) -> (Primitive, MouseCursor) {
        let mut res: Vec<Primitive> = Vec::new();
        let tile_size = self.tile_size(layout.bounds().width, layout.bounds().height);
        let mut hovers_piece: bool = false;
        for y in 0..self.get_board_height() {
            for x in 0..self.get_board_width() {
                let v_x = layout.bounds().x + tile_size * x as f32;
                let v_y = layout.bounds().y + tile_size * y as f32;
                let bounds = Rectangle {x: v_x, y: v_y, width: tile_size, height: tile_size};

                // Uncomment once the check_pos bug is resolved
                // if bounds.contains(mouse) && self.board.borrow().get(x, y).unwrap().is_some() {
                //     hovers_piece = true;
                // }
                res.push(Primitive::Quad {
                    bounds,
                    background: if (x + y) % 2 == 0 {Background::Color(self.fill_light)} else {Background::Color(self.fill_dark)},
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                });
            }
        }

        (
            Primitive::Group {primitives: res},
            if hovers_piece {MouseCursor::Pointer} else {MouseCursor::Idle}
        )
    }
}

impl<'a, Message> Into<iced_native::Element<'a, Message, Renderer>> for GBoard<'a> {
    fn into(self) -> iced_native::Element<'a, Message, Renderer> {
        iced_native::Element::new(self)
    }
}
