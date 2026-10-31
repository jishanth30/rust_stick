use std::io::{self,stdout,Write};
use std::time::{Duration,Instant};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    cursor::{Hide,MoveTo,Show},
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, size},
    ExecutableCommand, QueueableCommand,
};
pub fn render(
    buffer: &mut Vec<Vec<char>>,
    start_x: isize,
    start_y: isize,
    end_x: isize,
    end_y: isize,
    character: char,
){
    let screen_height = buffer.len() as isize;
    if screen_height == 0{
        return;    
    }
    let screen_width = buffer[0].len() as isize;

    let dx = (end_x - start_x).abs();
    let dy = -(end_y - start_y).abs();

    let step_x = if start_x < end_x {1} else {-1};
    let step_y = if start_y < end_y {1} else {-1};

    let mut errorbal = dx+dy;
    let mut curr_x = start_x;
    let mut curr_y = start_y;

    loop{
        if curr_x >= 0 && curr_x <screen_width && curr_y>=0 && curr_y < screen_height{
            buffer[curr_y as usize][curr_x as usize] = character;
        }

        if curr_x == end_x && curr_y == end_y{
            break;
        }

        let double_error = 2*errorbal;
        if double_error >= dy{
            errorbal += dy;
            curr_x += step_x;
        }
        if double_error <= dx{
            errorbal += dx;
            curr_y += step_y;
        }
    }

}
fn main() -> std::io::Result<()> {
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Red),
        Print("Styled text here."),
        ResetColor
    )?;

    
    
    enable_raw_mode()?;
    
    let mut stdout = io::stdout();

    stdout
        .execute(EnterAlternateScreen)?
        .execute(Hide)?
        .execute(Clear(ClearType::All))?;
    let mut start_x = 0.0;
    let mut start_y = 0.0;
    let mut end_x = 5.0;
    let mut end_y = 5.0;
    let mut last_frame = Instant::now();
    let mut speed = 0.0;
    let grav = 10;
    let mut grav_on = false;
    loop {
        let (width, height) = size()?;
        let now = Instant::now();
        let delta = now.duration_since(last_frame);
        last_frame = now;
        let delta_seconds = delta.as_secs_f32();
        // if grav_on{
        //     speed += (grav as f32)*delta_seconds;
        //     start_y += if (start_y < (height as f32) -3.0) {(speed as f32)*delta_seconds} else {0.0};
        //     end_y += if (end_y < (height as f32) -3.0) {(speed as f32)*delta_seconds} else {0.0};
        // }
        // Convert delta to seconds as f32 or f64 for calculations
        
        
        
        
        let mut screen_buffer = vec![vec![' '; width as usize];height as usize];
        
        stdout.queue(MoveTo(0,0))?;
        render(&mut screen_buffer,start_x as isize,start_y as isize,end_x as isize,end_y as isize,'$');

        stdout
        .queue(MoveTo(2, 1))?
        .queue(SetForegroundColor(Color::Yellow))?
        .queue(Print("=== Custom Crossterm-Only TUI ==="))?;
        
        // let cen_x = width as f32 / 2.0;
        // let cen_y = height as f32 /2.0;

        // let cell_size_cor = 2.0;

        // let screen_x = (cen_x + (proj_x*fov/z)*cell_size_cor);
        // let screen_y = (cen_y - (proj_y*fov/z)*cell_size_cor);

        // if (screen_x < width && screen_y < height) {
        //     screen_buffer[screen_y][screen_x] = '#';
        // }
        //Cursor reset
        print!("\x1b[H");
        for row in screen_buffer {
            let line: String = row.iter().collect();
            println!("{}", line);

        }
        io::stdout().flush().unwrap();


        if event::poll(std::time::Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                
                // Match specific keys or combinations
                match key_event.code {
                    KeyCode::Char('q') => {
                        println!("Quitting...\r");
                        break;
                    }
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        println!("Ctrl+C detected. Exiting...\r");
                        break;
                    }
                    KeyCode::Char(c) => {
                        println!("Character pressed: {}\r", c);
                        if c == 'w' {
                            start_y -= 1.0;
                        }
                        if c == 'a' {
                            start_x -= 1.0;
                        }
                        if c == 's'{
                            start_y += 1.0;
                        }
                        if c == 'd'{
                            start_x += 1.0;
                        }
                        if c == 'p' && !grav_on{
                            grav_on = true;
                        }
                        else if c == 'o' && grav_on{
                            grav_on = false;
                            speed = 0.0;
                        }
                    }
                    KeyCode::Enter => {
                        println!("Enter pressed\r");
                    }
                    KeyCode::Esc => {
                        println!("Escape pressed\r");
                    }
                    _ => {
                        println!("Other key event: {:?}\r", key_event.code);
                        if key_event.code == KeyCode::Up{
                            end_y -= 1.0;
                        }
                        if key_event.code == KeyCode::Down{
                            end_y += 1.0;
                        }
                        if key_event.code == KeyCode::Right{
                            end_x += 1.0;
                        }
                        if key_event.code == KeyCode::Left{
                            end_x -= 1.0;
                        }
                        
                    }
                }
                stdout.flush()?;
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
