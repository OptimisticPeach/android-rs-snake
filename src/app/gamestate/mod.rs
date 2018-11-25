pub mod one_player;
pub mod two_player;
pub mod common;
mod pause_screen;

use graphics::math::Matrix2d;
use graphics::*;
use crate::app::*;
use self::one_player::*;
use self::two_player::*;

pub enum Player{
    One(OnePlayer),
    Two(TwoPlayer),
} 

impl Player {
    pub(in self) fn draw<G: Graphics, T: graphics::character::CharacterCache<Texture=G::Texture>>(
        &mut self,
        c: &Context,
        transform: Matrix2d,
        g: &mut G,
        cache: &mut T,
        winfo: &mut window_info::WindowInfoCache,
        tri_cache: &mut common::TriangleCache
    ) {
        match self{
            Player::One(one) => {
                one.draw(c, transform, g, cache, winfo, tri_cache);
            }
            Player::Two(two) => {
                two.draw(c, transform, g, cache, winfo, tri_cache);
            }
        }
    }

    pub(in self) fn update(&mut self, winfo: &mut window_info::WindowInfoCache, cache: &mut impl graphics::character::CharacterCache) {
        match self{
            Player::One(one) => {
                one.update(winfo, cache);
            }
            Player::Two(two) => {
                two.update(winfo, cache);
            }
        }
    } 

    pub(in self) fn handle(&mut self, action: android_glue::Motion, winfo: &mut window_info::WindowInfoCache){
        match self {
            Player::One(one) => {
                one.handle(action, winfo);
            }
            Player::Two(two) => {
                two.handle(action, winfo);
            }
        }
    }

    pub(in self) fn pause(&mut self) {
        match self {
            Player::One(one) => {
                one.pause();
            }
            Player::Two(_) => {
            }
        }
    }
}

pub struct GameState{
    pub player_state: Player,
    pub is_paused: bool,
    tri_cache: common::TriangleCache,
    run_once: bool
}

impl GameState {
    pub fn initial() -> Self {
        Self{
            player_state: Player::One(OnePlayer::new()),
            is_paused: false,
            tri_cache: common::TriangleCache::new(),
            run_once: false
        }
    }

    pub fn draw<G: Graphics>(
        &mut self,
        c: &Context, 
        transform: Matrix2d,
        g: &mut G,
        cache: &mut impl graphics::character::CharacterCache<Texture=G::Texture>,
        winfo: &mut window_info::WindowInfoCache,
    ){
        if self.run_once {
            self.run_once = true;
            self.initialize(winfo, cache);
        }
        self.player_state.draw(c, transform, g, cache, winfo, &mut self.tri_cache);
        if self.is_paused {
            pause_screen::draw_pause(c, g, winfo);
        }
    }

    pub fn handle(&mut self, action: android_glue::Motion, winfo: &mut window_info::WindowInfoCache){
        match action.action {
            android_glue::MotionAction::Up => {
                if self.is_paused {
                    self.is_paused = false;
                    //TODO:
                    //This can cause us to crash if we 
                    //pause while still holding the 
                    //finger down, and let go and then
                    //try to tap again, we start 
                    //without having finished :(
                    return;
                }
            }
            _ => {}
        }
        self.player_state.handle(action, winfo);
    }

    pub fn size_change(&mut self, winfo: &mut window_info::WindowInfoCache, cache: &mut impl graphics::character::CharacterCache){
        self.initialize(winfo, cache);
    }

    pub fn pause(&mut self){
        self.is_paused = !self.is_paused;
        self.player_state.pause();
    }

    pub fn update(&mut self, winfo: &mut window_info::WindowInfoCache, cache: &mut impl graphics::character::CharacterCache) {
        if !self.is_paused{
            self.player_state.update(winfo, cache);
        }
    }

    /// Run when we have a new window size, or when we haven't run at least once.
    fn initialize(&mut self, winfo: &mut window_info::WindowInfoCache, cache: &mut impl graphics::character::CharacterCache) {
        self.tri_cache = common::TriangleCache::new();
        if winfo.window_size.0 < winfo.window_size.1 {
            let mut player = OnePlayer::new();
            player.initialize(winfo, cache);
            self.player_state = Player::One(player);
        } else {
            let mut player = TwoPlayer::new(); 
            player.initialize(winfo, cache);
            self.player_state = Player::Two(player);
        }
    }
}
