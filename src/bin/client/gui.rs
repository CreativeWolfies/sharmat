// Rendering main logic

use super::settings::*;
use super::style::SharmatStyleSheet;

use sharmat::{
    board::{Board, BoardResult},
    game::*,
    movement::Action,
    player::{Player, PlayerColor},
};

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
    pub game: Game,
    pub stylesheet: SharmatStyleSheet,
    pub settings: SharmatSettings,
    pub piece_assets: Rc<HashMap<String, Handle>>,

    // Current interraction state:
    last_touch: Option<(usize, usize)>,
    current_piece: Option<(usize, PlayerColor)>,
    actions: Vec<Action>,
}

/// Message enum for user interaction
#[derive(Debug)]
pub enum SharmatMessage {
    TileTouched(usize, usize), // (x, y)
}

/// Graphical board (visible representation of the board)
#[derive(Debug)]
pub struct GBoard<'a> {
    pub sharmat: &'a Sharmat,
    // pub cache: Cache<Self>,
    pub fill_dark: Color,
    pub fill_light: Color,
    pub fill_dark_hl: Color,
    pub fill_light_hl: Color,
    pub highlight_border_ratio: f32,
    pub piece_assets: Rc<HashMap<String, Handle>>,
    pub flip_board: bool,
    pub last_touch: Option<(usize, usize)>,
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
                game: flags.1,
                stylesheet: SharmatStyleSheet::default(),
                piece_assets: Rc::new(flags.0),
                settings: SharmatSettings::new(flags.2),
                last_touch: None,
                current_piece: None,
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
            .board()
            .do_actions(
                self.game.current_player().expect("No current player?"),
                self.actions.iter(),
            )
            .unwrap();
        Container::new(
            Row::new().push(
                Container::new::<iced_native::Element<_, _>>(
                    GBoard::new(
                        self,
                        self.piece_assets.clone(),
                        true, // TODO :)
                        self.last_touch,
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
                if let Some((sx, sy)) = self.last_touch {
                    if self.is_action_legal(Action::Movement(sx, sy, x, y)) {
                        self.push_action(Action::Movement(sx, sy, x, y));
                    } else {
                        self.last_touch = None;
                        self.actions = vec![];
                    }
                } else {
                    if let Ok(Some((piece, color))) = self.game.board().get(x, y) {
                        if color
                            == self
                                .game
                                .current_player()
                                .expect("No current player?")
                                .color
                        {
                            self.last_touch = Some((x, y));
                            self.current_piece = Some((piece, color));
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
        let actions_res = self.game.board().do_actions(
            self.game.current_player().expect("No current player?"),
            self.actions.iter(),
        );
        if let Ok((new_board, new_player)) = actions_res {
            self.last_touch = None;

            self.game.set_board(new_board);
            self.game.set_current_player(new_player);
            self.game.next_player();
            self.actions = Vec::new();
            self.last_touch = None;
            self.current_piece = None;
        }
    }

    pub fn get_temporary_state(&self) -> BoardResult<(Board, Player)> {
        self.game.board().do_actions(
            self.game.current_player().expect("No current player?"),
            self.actions.iter(),
        )
    }

    pub fn is_action_legal(&self, action: Action) -> bool {
        let (board, current_player) = self.get_temporary_state().unwrap();
        match action {
            Action::Movement(sx, sy, x, y) => {
                if let Ok(Some((piece_raw, color))) = board.get(sx, sy) {
                    if color == current_player.color {
                        if let Some(piece) = self.game.pieces().get(piece_raw) {
                            return piece
                                .movement_type()
                                .get(self.actions.len())
                                .map(|mt| {
                                    mt.flatten(&board, &current_player, sx, sy)
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
        let new_touch = match &action {
            Action::Movement(_x, _y, x2, y2) => Some((*x2, *y2)),
            _ => None,
        };
        self.actions.push(action);

        if let Some((piece_raw, color)) = self.current_piece {
            let (_new_board, new_current_player) = self
                .get_temporary_state()
                .unwrap();
            if color == new_current_player.color {
                if let Some(piece) = self.game.pieces().get(piece_raw) {
                    if self.actions.len() >= piece.movement_type().len() {
                        submit_actions = true;
                    }
                    if new_touch.is_some() {
                        self.last_touch = new_touch;
                    }
                }
            }
        }

        if submit_actions {
            self.submit_actions();
        }
    }
}

impl<'a> GBoard<'a> {
    pub fn new(
        sharmat: &'a Sharmat,
        piece_assets: Rc<HashMap<String, Handle>>,
        flip_board: bool,
        last_touch: Option<(usize, usize)>,
        board: Board,
    ) -> GBoard<'a> {
        GBoard {
            sharmat,
            fill_dark: Color::from_rgb8(226, 149, 120),
            fill_light: Color::from_rgb8(255, 221, 210),
            fill_dark_hl: Color::from_rgb8(113, 129, 120),
            fill_light_hl: Color::from_rgb8(128, 165, 165),
            piece_assets,
            highlight_border_ratio: 0.15,
            flip_board,
            last_touch,
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

    #[inline]
    pub fn game(&self) -> &Game {
        &self.sharmat.game
    }

    fn get_self_color(&self) -> PlayerColor {
        self.game()
            .current_player()
            .expect("No current player?")
            .color
    }

    pub fn get_hints(&self, mut x: usize, mut y: usize) -> Vec<(usize, usize)> {
        if !self.render_hints() {
            return vec![];
        }

        if self.last_touch.is_some() {
            x = self.last_touch.unwrap().0;
            y = self.last_touch.unwrap().1;
            self.get_hints_at(x, y)
        } else if x == std::usize::MAX {
            vec![]
        } else {
            self.get_hints_at(x, y)
        }
    }

    fn get_hints_at(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match self.get(x, y) {
            Some((piece_raw, player_color))
                if player_color == self.get_self_color() || self.render_hints_opponent() =>
            {
                let piece = self
                    .game()
                    .pieces()
                    .get(piece_raw)
                    .expect(&format!("Couldn't find piece {}", piece_raw));
                let player = self
                    .game()
                    .player(player_color)
                    .expect(&format!("Couldn't find player {:?}", player_color));
                piece.movement_type()[0]
                    .flatten(&self.board, player, x, y)
                    .unwrap()
                    .into_iter()
                    .map(|(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
                    .collect()
            }
            _ => vec![],
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
        self.sharmat.settings.get_bool("render_hints").unwrap_or(true)
    }

    pub fn render_hints_opponent(&self) -> bool {
        self.sharmat
            .settings
            .get_bool("render_hints_opponent")
            .unwrap_or(false)
    }
}

impl<'a> widget::Widget<SharmatMessage, Renderer> for GBoard<'a> {
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
                    if let Some(piece) = self.game().pieces().get(piece_index) {
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

                if self.last_touch == Some((x, y)) {
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
                    if state == ButtonState::Pressed {
                        messages.push(SharmatMessage::TileTouched(m_x, m_y));
                    }
                }
            }
        }
    }
}

impl<'a> Into<iced_native::Element<'a, SharmatMessage, Renderer>> for GBoard<'a> {
    fn into(self) -> iced_native::Element<'a, SharmatMessage, Renderer> {
        iced_native::Element::new(self)
    }
}
