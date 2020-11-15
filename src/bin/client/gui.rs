use super::style::SharmatStyleSheet;
use iced::{
    executor, Application, Background, Color, Command, Container, Element, Length, Point, Row, Size,
};
use iced_native::{
    layout,
    widget::{svg::Handle, Widget},
    MouseCursor, Rectangle,
};
use iced_wgpu::{Defaults, Primitive, Renderer};
use sharmat::{game::*, player::PlayerColor};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

/// Main window model
#[derive(Debug)]
pub struct Sharmat {
    pub game: Rc<RefCell<Game>>,
    pub stylesheet: SharmatStyleSheet,
    pub render_hints: bool,
    pub render_hints_opponent: bool,
    pub piece_assets: Rc<HashMap<String, Handle>>,
}

/// Message enum for user interaction
#[derive(Debug)]
pub enum SharmatMessage {}
type Message = SharmatMessage;

/// Graphical board (visible representation of the board)
#[derive(Debug)]
pub struct GBoard {
    pub game: Rc<RefCell<Game>>,
    // pub cache: Cache<Self>,
    pub fill_dark: Color,
    pub fill_light: Color,
    pub fill_dark_hl: Color,
    pub fill_light_hl: Color,
    pub highlight_border_ratio: f32,
    pub render_hints: bool,
    pub render_hints_opponent: bool,
    pub piece_assets: Rc<HashMap<String, Handle>>,
}

impl Application for Sharmat {
    type Executor = executor::Null;
    type Message = Message;
    /// SVG handles, Game structure, whether or not to show hints, whether or not to also show hints for the opponent's pieces
    type Flags = (HashMap<String, Handle>, Game, bool, bool);

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                game: Rc::new(RefCell::new(flags.1)),
                stylesheet: SharmatStyleSheet::default(),
                piece_assets: Rc::new(flags.0),
                render_hints: flags.2,
                render_hints_opponent: flags.3
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Sharmat")
    }

    fn view(&mut self) -> Element<Self::Message> {
        Container::new(
            Row::new().push(
                Container::new::<iced_native::Element<_, _>>(
                    GBoard::new(
                        self.game.clone(),
                        self.piece_assets.clone(),
                        self.render_hints,
                        self.render_hints_opponent,
                    )
                    .into(),
                )
                .width(Length::Units(600))
                .height(Length::Units(600))
                .padding(10),
            ),
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
    pub fn new(
        game: Rc<RefCell<Game>>,
        piece_assets: Rc<HashMap<String, Handle>>,
        render_hints: bool,
        render_hints_opponent: bool,
    ) -> GBoard {
        GBoard {
            game,
            fill_dark: Color::from_rgb8(226, 149, 120),
            fill_light: Color::from_rgb8(255, 221, 210),
            fill_dark_hl: Color::from_rgb8(113, 129, 120),
            fill_light_hl: Color::from_rgb8(128, 165, 165),
            piece_assets,
            render_hints,
            render_hints_opponent,
            highlight_border_ratio: 0.15,
        }
    }

    #[inline]
    pub fn get_board_width(&self) -> usize {
        self.game.borrow().board().width.get()
    }

    #[inline]
    pub fn get_board_height(&self) -> usize {
        self.game.borrow().board().height.get()
    }

    #[inline]
    pub fn tile_size(&self, width: f32, height: f32) -> f32 {
        (width / self.get_board_width() as f32).min(height / self.get_board_height() as f32)
    }

    #[inline]
    pub fn get_raw(&self, x: usize, y: usize) -> Option<(usize, PlayerColor)> {
        self.game.borrow().board().get(x, y).ok().flatten()
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<(usize, PlayerColor)> {
        self.game.borrow().board().get(x, y).ok().flatten()
    }

    fn get_hints(&self, m_x: usize, m_y: usize) -> Vec<(usize, usize)> {
        let hovered_piece_raw = if m_x == std::usize::MAX {
            None
        } else {
            self.get(m_x, m_y)
        };

        if hovered_piece_raw.is_some()
            && (
                hovered_piece_raw.unwrap().1 == self.game.borrow().current_player().expect("No player?").color
                || self.render_hints_opponent
            )
            && self.render_hints
        {
            let raw = &hovered_piece_raw.unwrap();
            let game = self.game.borrow();
            let hovered_piece = game
                .pieces()
                .get(raw.0)
                .expect(&format!("Couldn't find piece {}", raw.0));
            let hovered_player = game
                .player(raw.1)
                .expect(&format!("Couldn't find player {:?}", raw.1));
            hovered_piece.movement_type()[0]
                .flatten(self.game.borrow().board(), hovered_player, m_x, m_y)
                .unwrap()
                .into_iter()
                .map(|(dx, dy)| ((m_x as isize + dx) as usize, (m_y as isize + dy) as usize))
                .collect()
        } else {
            vec![]
        }
    }

    fn get_mouse_pos(&self, bounds: Rectangle, mouse: Point, tile_size: f32) -> (usize, usize) {
        if bounds.contains(mouse) {
            (
                ((mouse.x - bounds.x) / tile_size).floor() as usize,
                ((mouse.y - bounds.y) / tile_size).floor() as usize,
            )
        } else {
            (std::usize::MAX, std::usize::MAX)
        }
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
            self.tile_size(limits.max().width, limits.max().height)
                * self.get_board_height() as f32,
        ))
    }

    fn hash_layout(&self, hasher: &mut iced_native::Hasher) {
        self.game.borrow().board().hash(hasher);
    }

    fn draw(
        &self,
        _renderer: &mut Renderer,
        _defaults: &Defaults,
        layout: layout::Layout<'_>,
        mouse: Point,
    ) -> (Primitive, MouseCursor) {
        let mut res: Vec<Primitive> = Vec::new();
        let tile_size = self.tile_size(layout.bounds().width, layout.bounds().height);
        let hl_width = tile_size as f32 * self.highlight_border_ratio;

        let (m_x, m_y) = self.get_mouse_pos(layout.bounds(), mouse, tile_size);

        let hints = self.get_hints(m_x, m_y);

        for y in 0..self.get_board_height() {
            for x in 0..self.get_board_width() {
                let v_x = layout.bounds().x + tile_size * x as f32;
                let v_y = layout.bounds().y + tile_size * y as f32;
                let bounds = Rectangle {
                    x: v_x,
                    y: v_y,
                    width: tile_size,
                    height: tile_size,
                };
                let sub_bounds = Rectangle {
                    x: v_x + hl_width,
                    y: v_y + hl_width,
                    width: tile_size - 2.0 * hl_width,
                    height: tile_size - 2.0 * hl_width,
                };

                // Display piece at x, y
                if let Some((piece_index, piece_color)) = self.get(x, y) {
                    if let Some(piece) = self.game.borrow().pieces().get(piece_index) {
                        res.push(Primitive::Svg {
                            handle: self
                                .piece_assets
                                .get(if piece_color.white() {
                                    piece.display_white()
                                } else {
                                    piece.display_black()
                                })
                                .unwrap()
                                .clone(),
                            bounds,
                        });
                    } else {
                        panic!("Piece index {} out of bound!", piece_index);
                    }
                }

                if hints.iter().find(|(x2, y2)| x == *x2 && y == *y2).is_some() {
                    // inner tile
                    res.push(Primitive::Quad {
                        bounds: bounds.clone(),
                        background: if (x + y) % 2 == 0 {
                            Background::Color(self.fill_light_hl)
                        } else {
                            Background::Color(self.fill_dark_hl)
                        },
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                    // outer tile
                    res.push(Primitive::Quad {
                        bounds: sub_bounds.clone(),
                        background: if (x + y) % 2 == 0 {
                            Background::Color(self.fill_light)
                        } else {
                            Background::Color(self.fill_dark)
                        },
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                } else {
                    // entire tile
                    res.push(Primitive::Quad {
                        bounds: bounds.clone(),
                        background: if (x + y) % 2 == 0 {
                            Background::Color(self.fill_light)
                        } else {
                            Background::Color(self.fill_dark)
                        },
                        border_radius: 0,
                        border_width: 0,
                        border_color: Color::TRANSPARENT,
                    });
                }
            }
        }

        (
            Primitive::Group { primitives: res },
            if m_x != std::usize::MAX && self.get(m_x, m_y).is_some() {
                MouseCursor::Pointer
            } else {
                MouseCursor::Idle
            },
        )
    }
}

impl<'a, Message> Into<iced_native::Element<'a, Message, Renderer>> for GBoard {
    fn into(self) -> iced_native::Element<'a, Message, Renderer> {
        iced_native::Element::new(self)
    }
}
