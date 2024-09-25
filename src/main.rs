use std::error::Error;
use std::{io, thread};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use crossterm::{event, terminal, ExecutableCommand};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;
use rusty_time::Timer;
use snake::{render, Direction, Tickable, TICK_INTERVAL};
use snake::food::Food;
use snake::frame::{get_random_empty_position, new_frame, Drawable};
use snake::snake::Snake;

fn main() -> Result<(), Box<dyn Error>> {
    //Setup audio
    let mut audio = Audio::new();
    audio.add("new_game", "new_game.wav");
    audio.add("eat", "eat.wav");
    audio.add("you_lose", "you_lose.wav");
    audio.add("you_win", "you_win.wav");

    audio.play("new_game");

    // Setup terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    //Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();

        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    //Game objects
    let mut win = false;
    let mut instant = Instant::now();
    let mut ticker = Timer::new(Duration::from_millis(TICK_INTERVAL));
    let mut snake = Snake::new();
    let mut food = Food::new();

    //Game loop
    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => break 'gameloop,

                    KeyCode::Up => snake.set_direction(Direction::Up),
                    KeyCode::Down => snake.set_direction(Direction::Down),
                    KeyCode::Left => snake.set_direction(Direction::Left),
                    KeyCode::Right => snake.set_direction(Direction::Right),

                    _ => {}
                }
            }
        }

        //Tick
        ticker.tick(delta);

        if ticker.just_finished() {
            ticker.reset();

            snake.tick();
            food.tick();

            //Handle collision
            if *snake.has_collided() {
                break 'gameloop;
            }

            if snake.get_head_position() == food.position {
                snake.length += 1;
                audio.play("eat");

                food.position = match get_random_empty_position(&curr_frame){
                    Some(x) => x,
                    None => {
                        win = true;
                        break 'gameloop;
                    }
                }
            }
        }

        //Draw & render
        food.draw(&mut curr_frame);
        snake.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    //Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;

    if win {
        audio.play("you_win");
        println!("Congratulations! You win!");
    } else {
        audio.play("you_lose");
        println!("You lose! Good luck next time!");
    }

    //Exit
    audio.wait();
    Ok(())
}