//! Module pour gérer notre objet stellaire
use macroquad::prelude::*;

/// Trait qui definie les comportements des objets spatiaux.
pub trait StellarObject {
    /// Retourne la position de l'objet.
    /// # Arguments
    /// - `&self`: une instance de l'objet stellaire
    /// # Returns
    /// - `Vec2`: un vecteur avec la position x et y de l'objet stellaire
    fn get_pos(&self) -> Vec2;

    /// Met a jour la position de l'objet.
    /// # Arguments
    /// - `&mut self`: une instance de l'objet stellaire
    fn move_obj(&mut self);

    /// Retourne le rayon de l'objet.
    /// # Arguments
    /// - `&self`: une instance de l'objet stellaire
    /// # Returns
    /// - `f32`: le rayon de l'objet stellaire
    fn radius(&self) -> f32;

    /// Gere la collision avec un autre objet.
    /// # Arguments
    /// - `&mut self`: une instance de l'objet stellaire
    fn handle_collision(&mut self);
}
