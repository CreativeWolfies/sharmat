// Rendering main logic

use super::settings::*;
use super::style::SharmatStyleSheet;

use sharmat::{board::Board, game::*, movement::Action, player::PlayerColor};

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use iced::{Application, Background, Color, Command, Container, Element, Length, Point, Row, Size};
use iced_native::{
    input::{mouse, ButtonState},
    layout,
    widget::{self, svg::Handle},
    Event, MouseCursor, Rectangle,
};
use iced_wgpu::{Defaults, Primitive, Renderer};

/// Main window model
#[derive(Debug)]
pub struct Sharmat {
    pub game: Rc<RefCell<Game>>,
    pub stylesheet: SharmatStyleSheet,
    pub settings: SharmatSettings,
    pub piece_assets: Rc<HashMap<String, Handle>>,

    // Current interraction state:
    piece_touched: Option<(usize, usize)>,
    actions: Vec<Action>,
}

/// Message enum for user interaction
#[derive(Debug)]
pub enum SharmatMessage {
    TileTouched(usize, usize), // (x, y)
}

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
    pub settings: SharmatSettings,
    pub piece_assets: Rc<HashMap<String, Handle>>,
    pub flip_board: bool,
    pub piece_touched: Option<(usize, usize)>,
    pub board: Board,
}

impl Application for Sharmat {
    type Executor = iced::executor::Null;
    type Message = SharmatMessage;
    /// SVG handles, Game structure, whether or not to show hints, whether or not to also show hints for the opponent's pieces
    type Flags = (
        HashMap<String, Handle>,
        Game,
        HashMap<String, SharmatSettingType>,
    );

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                game: Rc::new(RefCell::new(flags.1)),
                stylesheet: SharmatStyleSheet::default(),
                piece_assets: Rc::new(flags.0),
                settings: SharmatSettings::new(flags.2),
                piece_touched: None,
                actions: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Sharmat")
    }

    fn view(&mut self) -> Element<Self::Message> {
        let (tmp_board, _tmp_player) = self
            .game
            .borrow()
            .board()
            .do_actions(
                self.game
                    .borrow()
                    .current_player()
                    .expect("No current player?"),
                self.actions.iter(),
            )
            .unwrap();
        Container::new(
            Row::new().push(
                Container::new::<iced_native::Element<_, _>>(
                    GBoard::new(
                        self.game.clone(),
                        self.piece_assets.clone(),
                        self.settings.clone(),
                        true, // TODO :)
                        self.piece_touched,
                        tmp_board,
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

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            SharmatMessage::TileTouched(x, y) => {
                if let Some((sx, sy)) = self.piece_touched {
                    if self.is_action_legal(Action::Movement(sx, sy, x, y)) {
                        self.push_action(Action::Movement(sx, sy, x, y));
                    } else {
                        self.piece_touched = None;
                    }
                } else {
                    if let Ok(Some((_piece, color))) = self.game.borrow().board().get(x, y) {
                        if color
                            == self
                                .game
                                .borrow()
                                .current_player()
                                .expect("No current player?")
                                .color
                        {
                            self.piece_touched = Some((x, y));
                        }
                    }
                }
            }
        }
        Command::none()
    }
}

impl Sharmat {
    pub fn submit_actions(&mut self) {
        let actions_res = self.game.borrow().board().do_actions(
            self.game
                .borrow()
                .current_player()
                .expect("No current player?"),
            self.actions.iter(),
        );
        if let Ok((new_board, new_player)) = actions_res {
            self.piece_touched = None;

            self.game.borrow_mut().set_board(new_board);
            self.game.borrow_mut().set_current_player(new_player);
            self.game.borrow_mut().next_player();
            self.actions = Vec::new();
            self.piece_touched = None;
        }
    }

    pub fn is_action_legal(&self, action: Action) -> bool {
        let game = self.game.borrow();
        let board = game.board();
        let current_player = game.current_player().expect("No current player?");
        match action {
            Action::Movement(sx, sy, x, y) => {
                if let Ok(Some((piece_raw, color))) = board.get(sx, sy) {
                    if color == current_player.color {
                        if let Some(piece) = game.pieces().get(piece_raw) {
                            return piece
                                .movement_type()
                                .get(self.actions.len())
                                .map(|mt| {
                                    mt.flatten(board, current_player, sx, sy)
                                        .unwrap()
                                        .iter()
                                        .find(|(x2, y2)| {
                                            (sx as isize + *x2) as usize == x
                                                && (sy as isize + *y2) as usize == y
                                        })
                                        .is_some()
                                })
                                .unwrap_or(false);
                        }
                    }
                }
            }
            _ => {}
        }
        false
    }

    pub fn push_action(&mut self, action: Action) {
        let mut submit_actions = false;
        self.actions.push(action);

        if let Some((x, y)) = self.piece_touched {
            let game = self.game.borrow();
            let board = game.board();
            let (new_board, new_current_player) = game
                .board()
                .do_actions(
                    game.current_player().expect("No current player?"),
                    self.actions.iter(),
                )
                .unwrap();
            if let Ok(Some((piece_raw, color))) = board.get(x, y) {
                if color == new_current_player.color {
                    if let Some(piece) = game.pieces().get(piece_raw) {
                        if self.actions.len() >= piece.movement_type().len() {
                            submit_actions = true;
                        }
                    }
                }
            }
        }

        if submit_actions {
            self.submit_actions();
        }
    }
}

impl GBoard {
    pub fn new(
        game: Rc<RefCell<Game>>,
        piece_assets: Rc<HashMap<String, Handle>>,
        settings: SharmatSettings,
        flip_board: bool,
        piece_touched: Option<(usize, usize)>,
        board: Board,
    ) -> GBoard {
        GBoard {
            game,
            fill_dark: Color::from_rgb8(226, 149, 120),
            fill_light: Color::from_rgb8(255, 221, 210),
            fill_dark_hl: Color::from_rgb8(113, 129, 120),
            fill_light_hl: Color::from_rgb8(128, 165, 165),
            piece_assets,
            settings: settings.clone(),
            highlight_border_ratio: 0.15,
            flip_board,
            piece_touched,
            board,
        }
    }

    #[inline]
    pub fn get_board_width(&self) -> usize {
        self.board.width.get()
    }

    #[inline]
    pub fn get_board_height(&self) -> usize {
        self.board.height.get()
    }

    #[inline]
    pub fn tile_size(&self, width: f32, height: f32) -> f32 {
        (width / self.get_board_width() as f32).min(height / self.get_board_height() as f32)
    }

    #[inline]
    pub fn get_raw(&self, x: usize, y: usize) -> Option<(usize, PlayerColor)> {
        self.board.get(x, y).ok().flatten()
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<(usize, PlayerColor)> {
        self.board.get(x, y).ok().flatten()
    }

    fn get_hints(&self, mut x: usize, mut y: usize) -> Vec<(usize, usize)> {
        let hovered_piece_raw = if self.piece_touched.is_some() {
            x = self.piece_touched.unwrap().0;
            y = self.piece_touched.unwrap().1;
            self.get(x, y)
        } else if x == std::usize::MAX {
            None
        } else {
            self.get(x, y)
        };

        // Maybe clean up that logic? the self.piece_touched.is_some() just looks dirty
        if hovered_piece_raw.is_some()
            && (hovered_piece_raw.unwrap().1
                == self
                    .game
                    .borrow()
                    .current_player()
                    .expect("No player?")
                    .color
                || self.render_hints_opponent())
            && self.render_hints()
            && self.piece_touched.is_some()
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
                .flatten(&self.board, hovered_player, x, y)
                .unwrap()
                .into_iter()
                .map(|(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
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

    pub fn render_hints(&self) -> bool {
        self.settings.get_bool("render_hints").unwrap_or(true)
    }

    pub fn render_hints_opponent(&self) -> bool {
        self.settings
            .get_bool("render_hints_opponent")
            .unwrap_or(false)
    }
}

impl<'a> widget::Widget<SharmatMessage, Renderer> for GBoard {
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
        self.board.hash(hasher);
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

                if self.piece_touched == Some((x, y)) {
                    // whole tile
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
                } else if hints.iter().find(|(x2, y2)| x == *x2 && y == *y2).is_some() {
                    // outer tile
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
                    // inner tile
                    res.push(Primitive::Quad {
                        bounds: sub_bounds.clone(),
                        background: if (x + y) % 2 == 0 {
                            Background::Color(self.fill_light_hl)
                        } else {
                            Background::Color(self.fill_dark_hl)
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
            if m_x != std::usize::MAX
                && (self.get(m_x, m_y).is_some()
                    || hints.iter().find(|(x, y)| m_x == *x && m_y == *y).is_some())
            {
                MouseCursor::Pointer
            } else {
                MouseCursor::Idle
            },
        )
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: layout::Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<SharmatMessage>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn iced_native::Clipboard>,
    ) {
        let tile_size = self.tile_size(layout.bounds().width, layout.bounds().height);
        if let Event::Mouse(mouse::Event::Input { state, button }) = event {
            if button == mouse::Button::Left {
                let (m_x, m_y) = self.get_mouse_pos(layout.bounds(), cursor_position, tile_size);
                if m_x != std::usize::MAX {
                    if state == ButtonState::Pressed
                        || self.piece_touched.is_some()
                            && self.piece_touched.unwrap() != (m_x, m_y)
                            && state == ButtonState::Released
                    {
                        messages.push(SharmatMessage::TileTouched(m_x, m_y));
                    }
                }
            }
        }
    }
}

impl<'a> Into<iced_native::Element<'a, SharmatMessage, Renderer>> for GBoard {
    fn into(self) -> iced_native::Element<'a, SharmatMessage, Renderer> {
        iced_native::Element::new(self)
    }
}
