use std::num::NonZeroUsize;
use iced::{
    executor,
    Application, Container, Element, Length, Point, Size, Color, Row, Command, Background,
};
use iced_native::{
    widget::{Widget, svg::Handle},
    layout,
    MouseCursor, Rectangle,
};
use iced_wgpu::{Renderer, Primitive, Defaults};
use sharmat::{
    board::*,
    piece::Piece,
    game::*,
};
use super::style::SharmatStyleSheet;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::hash::Hash;

/// Main window model
#[derive(Debug)]
pub struct Sharmat {
    pub game: Rc<RefCell<Game>>,
    pub stylesheet: SharmatStyleSheet,
    pub piece_assets: Rc<HashMap<String, Handle>>,
}

/// Message enum for user interaction
#[derive(Debug)]
pub enum SharmatMessage {}
type Message = SharmatMessage;

/// Graphical board (visible representation of the board)
#[derive(Debug)]
pub struct GBoard {
    pub board: usize,
    pub game: Rc<RefCell<Game>>,
    // pub cache: Cache<Self>,
    pub fill_dark: Color,
    pub fill_light: Color,
    pub piece_assets: Rc<HashMap<String, Handle>>,
}

impl Application for Sharmat {
    type Executor = executor::Null;
    type Message = Message;
    type Flags = (HashMap<String, Handle>, Game);

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {
            game: Rc::new(RefCell::new(flags.1)),
            stylesheet: SharmatStyleSheet::default(),
            piece_assets: Rc::new(flags.0),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Sharmat")
    }

    fn view(&mut self) -> Element<Self::Message> {
        Container::new(
            Row::new()
                .push(
                    Container::new::<iced_native::Element<_, _>>(GBoard::new(self.game.clone(), 0, self.piece_assets.clone()).into())
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

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
}

impl GBoard {
    pub fn new(game: Rc<RefCell<Game>>, board: usize, piece_assets: Rc<HashMap<String, Handle>>) -> GBoard {
        GBoard {
            game,
            board,
            fill_dark: Color::from_rgb8(226, 149, 120),
            fill_light: Color::from_rgb8(255, 221, 210),
            piece_assets,
        }
    }

    #[inline]
    pub fn get_board_width(&self) -> usize {
        self.game.borrow().boards()[self.board].width.get()
    }

    #[inline]
    pub fn get_board_height(&self) -> usize {
        self.game.borrow().boards()[self.board].height.get()
    }

    #[inline]
    pub fn tile_size(&self, width: f32, height: f32) -> f32 {
        (width / self.get_board_width() as f32).min(height / self.get_board_height() as f32)
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<usize> {
        self.game.borrow().boards()[self.board].get(x, y).ok().flatten()
    }
}

impl<'a, Message> Widget<Message, Renderer> for GBoard {
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
        self.game.borrow().boards()[self.board].hash(hasher);
    }

    fn draw(&self, _renderer: &mut Renderer, _defaults: &Defaults, layout: layout::Layout<'_>, mouse: Point) -> (Primitive, MouseCursor) {
        let mut res: Vec<Primitive> = Vec::new();
        let tile_size = self.tile_size(layout.bounds().width, layout.bounds().height);
        let mut hovers_piece: bool = false;
        for y in 0..self.get_board_height() {
            for x in 0..self.get_board_width() {
                let v_x = layout.bounds().x + tile_size * x as f32;
                let v_y = layout.bounds().y + tile_size * y as f32;
                let bounds = Rectangle {x: v_x, y: v_y, width: tile_size, height: tile_size};

                // Display piece at x, y
                if let Some(piece_index) = self.get(x, y) {
                    if bounds.contains(mouse) {
                        hovers_piece = true;
                    }

                    if let Some(piece) = self.game.borrow().pieces().get(piece_index) {
                        res.push(Primitive::Svg {
                            handle: self.piece_assets.get(piece.id()).unwrap().clone(),
                            bounds,
                        });
                    } else {
                        panic!("Piece index {} does not exist!");
                    }
                }

                res.push(Primitive::Quad {
                    bounds: bounds.clone(),
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

impl<'a, Message> Into<iced_native::Element<'a, Message, Renderer>> for GBoard {
    fn into(self) -> iced_native::Element<'a, Message, Renderer> {
        iced_native::Element::new(self)
    }
}
