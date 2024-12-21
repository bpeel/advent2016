#[derive(Clone, Copy, PartialEq, Eq)]
enum Button {
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    A,
    Up,
    Down,
    Left,
    Right,
    Space,
}

impl Button {
    fn as_char(&self) -> char {
        match self {
            Button::D0 => '0',
            Button::D1 => '1',
            Button::D2 => '2',
            Button::D3 => '3',
            Button::D4 => '4',
            Button::D5 => '5',
            Button::D6 => '6',
            Button::D7 => '7',
            Button::D8 => '8',
            Button::D9 => '9',
            Button::A => 'A',
            Button::Up => '^',
            Button::Down => 'v',
            Button::Left => '<',
            Button::Right => '>',
            Button::Space => ' ',
        }
    }
}

const DPAD_WIDTH: usize = 3;
const DPAD_HEIGHT: usize = 2;
const KEYPAD_WIDTH: usize = 3;
const KEYPAD_HEIGHT: usize = 4;

static DPAD_BUTTONS: [Button; DPAD_WIDTH * DPAD_HEIGHT] = [
    Button::Space, Button::Up,    Button::A,
    Button::Left,  Button::Down,  Button::Right,
];

static KEYPAD_BUTTONS: [Button; KEYPAD_WIDTH * KEYPAD_HEIGHT] = [
    Button::D7,    Button::D8,    Button::D9,
    Button::D4,    Button::D5,    Button::D6,
    Button::D1,    Button::D2,    Button::D3,
    Button::Space, Button::D0,    Button::A,
];

struct Game {
    dpad_positions: [(u8, u8); 2],
    keypad_position: (u8, u8),
    should_quit: bool,
    history: String,
    code: String,
}

impl Game {
    fn new() -> Game {
        Game {
            dpad_positions: [(2, 0), (2, 0)],
            keypad_position: (2, 3),
            should_quit: false,
            history: String::new(),
            code: String::new(),
        }
    }

    fn handle_key_code(&mut self, key: i32) {
        match key {
            ncurses::KEY_UP => self.move_cursor(Button::Up),
            ncurses::KEY_DOWN => self.move_cursor(Button::Down),
            ncurses::KEY_LEFT => self.move_cursor(Button::Left),
            ncurses::KEY_RIGHT => self.move_cursor(Button::Right),
            _ => (),
        }
    }

    fn handle_char(&mut self, ch: ncurses::winttype) {
        if let Some(ch) = char::from_u32(ch as u32) {
            match ch {
                '\u{0003}' => self.should_quit = true, // Ctrl+C
                '\n' => self.activate(),
                _ => (),
            }
        }
    }

    fn handle_key(&mut self, key: ncurses::WchResult) {
        match key {
            ncurses::WchResult::KeyCode(code) => self.handle_key_code(code),
            ncurses::WchResult::Char(ch) => self.handle_char(ch),
        }
    }

    fn draw_dpad(&self, start_x: i32, start_y: i32, cursor_pos: (u8, u8)) {
        self.draw_buttons(
            DPAD_WIDTH,
            &DPAD_BUTTONS,
            start_x, start_y,
            cursor_pos,
        );
    }

    fn draw_buttons(
        &self,
        width: usize,
        buttons: &[Button],
        start_x: i32,
        start_y: i32,
        cursor_pos: (u8, u8),
    ) {
        for y in 0..buttons.len() / width {
            ncurses::mv(start_y + y as i32, start_x);

            for x in 0..width {
                if x == cursor_pos.0 as usize && y == cursor_pos.1 as usize {
                    ncurses::addch('(' as u32);
                    addch_utf8(buttons[y * width + x].as_char());
                    ncurses::addch(')' as u32);
                } else {
                    ncurses::addch(' ' as u32);
                    addch_utf8(buttons[y * width + x].as_char());
                    ncurses::addch(' ' as u32);
                }
            }
        }
    }

    fn redraw(&self) {
        ncurses::clear();

        self.draw_dpad(0, 0, self.dpad_positions[0]);
        self.draw_dpad(0, 4, self.dpad_positions[1]);

        self.draw_buttons(
            KEYPAD_WIDTH,
            &KEYPAD_BUTTONS,
            0, 8,
            self.keypad_position,
        );

        let _ = ncurses::mvaddstr(14, 2, "History: ");
        let _ = ncurses::addstr(&self.history);
        let _ = ncurses::mvaddstr(15, 2, "Code:    ");
        let _ = ncurses::addstr(&self.code);

        ncurses::refresh();
    }

    fn move_cursor(&mut self, button: Button) {
        move_position(
            button,
            &mut self.dpad_positions[0],
            DPAD_WIDTH,
            DPAD_HEIGHT,
        );

        self.history.push(button.as_char());

        self.redraw();
    }

    fn activate(&mut self) {
        let pos = self.dpad_positions[0];
        let button = DPAD_BUTTONS[pos.1 as usize * DPAD_WIDTH + pos.0 as usize];

        if button == Button::A {
            self.activate_second_dpad();
        } else {
            move_position(
                button,
                &mut self.dpad_positions[1],
                DPAD_WIDTH, DPAD_HEIGHT,
            );
        }

        self.history.push('A');

        self.redraw();
    }

    fn activate_second_dpad(&mut self) {
        let pos = self.dpad_positions[1];
        let button = DPAD_BUTTONS[pos.1 as usize * DPAD_WIDTH + pos.0 as usize];

        if button == Button::A {
            self.activate_keypad();
        } else {
            move_position(
                button,
                &mut self.keypad_position,
                KEYPAD_WIDTH, KEYPAD_HEIGHT,
            );
        }
    }

    fn activate_keypad(&mut self) {
        let pos = self.keypad_position;
        let button = KEYPAD_BUTTONS[
            pos.1 as usize * KEYPAD_WIDTH +
                pos.0 as usize
        ];

        self.code.push(button.as_char());
    }
}

fn move_position(
    button: Button,
    pos: &mut (u8, u8),
    width: usize,
    height: usize,
) {
    match button {
        Button::Left => pos.0 = pos.0.saturating_sub(1),
        Button::Right => pos.0 = (pos.0 + 1).min(width as u8 - 1),
        Button::Up => pos.1 = pos.1.saturating_sub(1),
        Button::Down => pos.1 = (pos.1 + 1).min(height as u8 - 1),
        _=> (),
    }
}

fn addch_utf8(ch: char) {
    let mut buf = [0u8; 4];

    let _ = ncurses::addstr(ch.encode_utf8(&mut buf));
}

fn main() {
    gettextrs::setlocale(gettextrs::LocaleCategory::LcAll, "");

    ncurses::initscr();
    ncurses::raw();
    ncurses::noecho();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::start_color();
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut game = Game::new();

    game.redraw();

    while !game.should_quit {
        let Some(key) = ncurses::get_wch()
        else {
            break;
        };

        game.handle_key(key);
    }

    ncurses::endwin();
}
