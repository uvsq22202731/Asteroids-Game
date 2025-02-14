//! Module pour gérer le vaisseau spatial.
//! Le vaisseau peut se déplacer, tourner, utiliser un bouclier et devenir temporairement invincible.
use macroquad::prelude::*;
use std::f32::consts::PI;

/// Structure représentant le vaisseau spatial du joueur.
/// # Champs
/// - `position`: la position du spaceship
/// - `velocity`: la vitesse du spaceship
/// - `rotation`: l'angle de rotation du spaceship
/// - `shield`: booleen permettant de savoir si le shield est actif ou non
/// - `invincible`: booleen permettant de savoir si le vaisseau est invincible ou non
/// - `ìnvincibily_timer`: compteur la durée de l'invincibilité du vaisseau
/// - `hit`: booleen pour savoir si on a été touché
/// - `active`: permet de savoir si le vaisseau est acitf ou non
/// - `radius`: le rayon du vaisseau
/// - `texture`: la texture du spaceship
/// - `texture_shield_on`: la texture du bouclier actif
/// - `texture_shield_off`: la texture du bouclier quand le spaceship est invincible
/// - `texture_shield_dead`: la texture du bouclier detruit
pub struct Spaceship {
    position: Vec2,
    pub velocity: Vec2,
    pub rotation: f32,
    pub shield: bool,
    pub invincible: bool,
    pub invincibility_timer: f32,
    pub hit: bool,
    pub active: bool,
    radius: f32,
    texture: Texture2D,
    texture_shield_on: Texture2D,
    texture_shield_off: Texture2D,
    texture_shield_dead: Texture2D,
}

impl Spaceship {
    /// Crée un nouveau vaisseau positionné au centre de l'écran.
    /// # Returns
    /// - `Self`: Un objet spaceship positionné au milieu de l'écran, avec un bouclier
    pub async fn new() -> Self {
        let texture = load_texture("assets/spaceship.png").await.unwrap();
        let texture_shield_on = load_texture("assets/shield_on.png").await.unwrap();
        let texture_shield_off = load_texture("assets/shield_off.png").await.unwrap();
        let texture_shield_dead = load_texture("assets/shield_dead.png").await.unwrap();
        Self {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            velocity: vec2(0.0, 0.0),
            rotation: 0.0,
            shield: true, // Bouclier activé au départ
            invincible: false,
            invincibility_timer: 0.0,
            hit: false,
            active: true,
            radius: 25.0,
            texture,
            texture_shield_on,
            texture_shield_off,
            texture_shield_dead,
        }
    }

    /// Dessine le vaisseau et ses effets visuels (bouclier, invincibilité).
    /// # Arguments
    /// - `&self`: instance de vaisseau
    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.position.x - self.radius,
            self.position.y - self.radius,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.radius * 2.0, self.radius * 2.0)),
                rotation: self.rotation + PI / 2.0,
                ..Default::default()
            },
        );

        // Afficher le bouclier si actif
        if self.shield {
            draw_texture_ex(
                &self.texture_shield_on,
                self.position.x - self.radius * 1.5,
                self.position.y - self.radius * 1.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.radius * 3.0, self.radius * 3.0)),
                    rotation: self.rotation + PI / 2.0,
                    ..Default::default()
                },
            );
        } else if !self.shield && !self.invincible {
            draw_texture_ex(
                &self.texture_shield_dead,
                self.position.x - self.radius * 1.5,
                self.position.y - self.radius * 1.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.radius * 3.0, self.radius * 3.0)),
                    rotation: self.rotation + PI / 2.0,
                    ..Default::default()
                },
            );
        }
        // Afficher l'invincibilité si active
        if self.invincible {
            draw_texture_ex(
                &self.texture_shield_off,
                self.position.x - self.radius * 1.5,
                self.position.y - self.radius * 1.5,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.radius * 3.0, self.radius * 3.0)),
                    rotation: self.rotation + PI / 2.0,
                    ..Default::default()
                },
            );
        }
    }

    /// Applique une poussée pour déplacer le vaisseau.
    /// # Arguments
    /// - `&mut self`: instance mutable du vaisseau afin de changer sa vitesse
    /// - `amount`: montant correspondant à l'augmentation de la vitesse
    pub fn apply_thrust(&mut self, amount: f32) {
        let thrust = vec2(self.rotation.cos(), self.rotation.sin()) * amount;
        self.velocity += thrust;
    }

    /// Tourne le vaisseau d'un angle donné.
    /// # Arguments
    /// - `&mut self`: instance mutable du vaisseau afin de changer son angle de rotation
    /// - `angle`: montant correspondant à l'augmentation de l'angle
    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }

    /// Gère la transition du vaisseau autour de l'écran.
    /// # Arguments
    /// - `position`: un vecteur correspond à la position du vaisseau en x et y
    /// # Returns
    /// - `Vec2`: un vecteur contenant x et y correspondant à la nouvelle position du vaisseau
    fn wrap_around_screen(position: Vec2) -> Vec2 {
        vec2(
            (position.x + screen_width()) % screen_width(),
            (position.y + screen_height()) % screen_height(),
        )
    }
}

use crate::stellarobject::StellarObject;

impl StellarObject for Spaceship {
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
        self.position = Self::wrap_around_screen(self.position);
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
        if self.shield {
            self.shield = false;
            self.invincible = true;
            self.invincibility_timer = 2.0;
            self.hit = true;
        } else {
            self.active = false;
        }
    }
}
