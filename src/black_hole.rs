//! Module pour gérer les actions de nos trous noirs
use macroquad::prelude::*;

/// Structure permettant de représenter nos trous noirs
/// # Champs
/// - `position`: position x et y du trou noir
/// - `size`: la taille du trou noir
/// - `counter`: compteur qui compte le nombre de collission
/// - `active`: permet de savoir si le trou noir est actif ou non
/// - `texture`: la texture du trou noir
pub struct BlackHole {
    position: Vec2,
    size: f32,
    pub counter: u8,
    pub active: bool,
    texture: Texture2D,
}

impl BlackHole {
    /// Créer un nouveau trou noir
    /// # Arguments
    /// - `position`: la position x et y du trou noir
    /// - `size_ast`: contient la taille de l'asteroid détruit
    /// # Returns
    /// - `Self`: un trou noir
    pub async fn new(position: Vec2, size_ast: f32) -> Self {
        let texture = load_texture("assets/black_hole.png").await.unwrap();
        Self {
            position,
            size: size_ast,
            counter: 0,
            active: true,
            texture,
        }
    }

    /// Fonction qui dessine le trou noir
    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x - self.radius(),
            self.position.y - self.radius(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.radius() * 2.0, self.radius() * 2.0)),
                ..Default::default()
            },
        );
    }
}

use crate::stellarobject::StellarObject; // Utilise le trait StellarObject

impl StellarObject for BlackHole {
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
    fn move_obj(&mut self) {}

    /// Retourne le rayon de l'objet.
    /// # Arguments
    /// - `&self`: une instance de l'objet stellaire
    /// # Returns
    /// - `f32`: le rayon de l'objet stellaire
    fn radius(&self) -> f32 {
        self.size / 2.0
    }

    /// Gere la collision avec un autre objet.
    /// Ici s'il y a cinq collision avec le trou noir dissparait
    /// # Arguments
    /// - `&mut self`: une instance de l'objet stellaire
    fn handle_collision(&mut self) {
        if self.counter == 5 {
            self.active = false
        } else {
            self.counter += 1;
        }
    }
}
