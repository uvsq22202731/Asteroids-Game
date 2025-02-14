//! Module pour gérer les missiles dans le jeu.
//! Un missile est tiré par le vaisseau et se déplace dans une direction
//! jusqu'à ce qu'il quitte l'écran ou touche un objet.
use macroquad::prelude::*;

/// Structure représentant un missile.
/// # Champs
/// - `position`: la position du vaisseau en x et y
/// - `velocity`: la vitesse du missile
/// - `active`: permet de savoir si le missile est actif ou non
/// - `radius`: le rayon du missile
pub struct Missile {
    pub position: Vec2,
    velocity: Vec2,
    pub active: bool,
    radius: f32,
}

impl Missile {
    /// Crée un nouveau missile à une position donnée avec une direction.
    ///
    /// # Arguments
    /// - `position`: Position initiale du missile.
    /// - `rotation`: Rotation (en radians) pour déterminer la direction du missile.
    pub fn new(position: Vec2, rotation: f32) -> Self {
        let speed = 4.0;
        Self {
            position,
            velocity: vec2(rotation.cos(), rotation.sin()) * speed,
            active: true,
            radius: 2.0,
        }
    }

    /// Dessine le missile sur l'écran.
    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, RED);
    }

    /// Désactive le missile s'il quitte l'écran.
    fn wrap_around_screen(&mut self) {
        if self.position.x < 0.0
            || self.position.x > screen_width()
            || self.position.y < 0.0
            || self.position.y > screen_height()
        {
            self.active = false;
        }
    }
}

use crate::stellarobject::StellarObject; // Utilise le trait StellarObject

impl StellarObject for Missile {
    /// Retourne la position de l'objet.
    /// # Arguments
    /// - `&self`: une instance de l'objet stellaire
    /// # Returns
    /// - `Vec2`: un vecteur avec la position x et y de l'objet stellaire
    fn get_pos(&self) -> Vec2 {
        self.position
    }

    /// Met a jour la position de l'objet.
    /// # Arguments
    /// - `&mut self`: une instance de l'objet stellaire
    fn move_obj(&mut self) {
        self.position += self.velocity;
        self.wrap_around_screen();
    }

    /// Retourne le rayon de l'objet.
    /// # Arguments
    /// - `&self`: une instance de l'objet stellaire
    /// # Returns
    /// - `f32`: le rayon de l'objet stellaire
    fn radius(&self) -> f32 {
        self.radius
    }

    /// Gere la collision avec un autre objet.
    /// # Arguments
    /// - `&mut self`: une instance de l'objet stellaire
    fn handle_collision(&mut self) {
        self.active = false;
    }
}
