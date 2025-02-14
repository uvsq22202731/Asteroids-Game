//! Module pour gérer nos asteroides
//! leur taille, leur séparation, leur déplacements

use ::rand::{thread_rng, Rng}; // Utilisation explicite de ::rand pour éviter les conflits
use macroquad::prelude::*;
use std::f32::consts::PI;

#[derive(Clone)]
/// Structure qui représente un asteroide
/// # Champs
/// - `position`: la position de l'asteroide
/// - `speed`: la vitesse de l'asteroide
/// - `size`: la taille de l'asteroide
/// - `texture`: la texture de l'asteroide
/// - `active`: permet de savoir si l'asteroide est actif ou non
pub struct Asteroid {
    position: Vec2,
    speed: Vec2,
    size: Size,
    texture: Texture2D,
    pub active: bool,
}

#[derive(Clone, Copy, PartialEq)]
/// Énumération représentant les différents taille d'asteroides possible
pub enum Size {
    Large,
    Medium,
    Small,
}

impl Size {
    /// Fonction qui donne une size en f32 pour chaque taille d'asteroide posssible
    /// # Arguments
    /// - `self`: représente l'objet asteroid lui même
    /// # Returns
    /// - `f32`: représente la taille de l'asteroides en nombre flottant
    pub fn scale(self) -> f32 {
        match self {
            Size::Large => 100.0,
            Size::Medium => 70.0,
            Size::Small => 40.0,
        }
    }

    /// Fonction qui donne le prochain état de l'asteroide en fonction de sa taille
    /// Large --> Medium --> Small --> None
    /// # Arguments
    /// - `self`: l'object asteroid lui même
    /// # Returns
    /// - `Òption<Size>`: l'état de l'asteroid après son entrée dans la fonction (Large, Medium, Small ou None)
    pub fn next(self) -> Option<Size> {
        match self {
            Size::Large => Some(Size::Medium),
            Size::Medium => Some(Size::Small),
            Size::Small => None,
        }
    }
}

impl Asteroid {
    const SIZES: [Size; 3] = [Size::Large, Size::Medium, Size::Small];

    /// Fonction qui créer un nouvel asteroid
    /// # Returns
    /// - `self`: un objet asteroid, avec sa position, vitesse, taille, et texture.
    pub async fn new() -> Self {
        let mut rng = thread_rng();
        let size = Self::SIZES[rng.gen_range(0..Self::SIZES.len())];
        let texture = load_texture("assets/asteroid.png").await.unwrap();
        Self {
            position: Self::new_random_position(size.scale()),
            speed: Self::new_random_speed(),
            size,
            texture,
            active: true,
        }
    }

    /// Fonction qui créer un nouveau asteroid avec sa nouvelle taille actualisé
    /// # Arguments
    /// - `size`: la taille de l'asteroide
    /// - `position`: la position de l'asteroide
    /// - `speed`: la vitesse de l'asteroide
    /// - `texture`: la texture de l'asteroid
    /// # Returns
    /// - `Self': un nouveau objet Asteroid
    pub fn new_with_size(size: Size, position: Vec2, speed: Vec2, texture: Texture2D) -> Self {
        Self {
            position,
            speed,
            size,
            texture,
            active: true,
        }
    }

    /// Fonction qui renvoie la taille de l'objet
    /// # Arguments
    /// - `&self`: l'objet asteroid lui même
    /// # Returns
    /// - `Size`: la taille de l'objet
    pub fn get_size(&self) -> Size {
        self.size
    }

    /// Fonction qui dessine la texture sur l'asteroide
    /// # Arguments
    /// - `&self`: l'objet asteroid lui même
    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture, // Utilisation d'une référence à la texture
            self.position.x - self.radius(),
            self.position.y - self.radius(),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size.scale(), self.size.scale())),
                ..Default::default()
            },
        );
    }

    /// Fonction qui sépare l'asteroid en fonction de sa taille
    /// # Arguments
    /// - `&self`: l'objet Asteroid lui même
    /// # Returns
    /// - Òption(<Asteroid, Asteroid): renvoie deux nouveau objet si l'asteroide est séparable sinon `None`
    pub fn split(&self) -> Option<(Asteroid, Asteroid)> {
        if let Some(new_size) = self.size.next() {
            let mut rng = thread_rng();
            let speed_variation = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
            Some((
                Asteroid::new_with_size(
                    new_size,
                    self.position,
                    self.speed + speed_variation,
                    self.texture.clone(),
                ), // Clone de la texture
                Asteroid::new_with_size(
                    new_size,
                    self.position,
                    self.speed - speed_variation,
                    self.texture.clone(),
                ), // Clone de la texture
            ))
        } else {
            None
        }
    }

    /// Fonction qui renvoie une position aléatoire pour le nouveau asteroid crée
    /// # Arguments
    /// - `size`: la taille de l'asteroide actuel
    /// # Returns
    /// - `Vec2`: vecteur avec la nouvelle position de l'asteroide proche de l'ancien
    fn new_random_position(size: f32) -> Vec2 {
        let mut rng = thread_rng();
        let nearpos: f32 = rng.gen_range(size / 2.0..=size);
        let xpos: f32 = if rng.gen_bool(0.5) {
            nearpos
        } else {
            screen_width() - nearpos
        };
        let ypos: f32 = if rng.gen_bool(0.5) {
            nearpos
        } else {
            screen_height() - nearpos
        };
        vec2(xpos, ypos)
    }

    /// Fonction qui renvoie une vitesse aléatoire pour le nouveau asteroid crée
    /// # Returns
    /// - `Vec2`: vecteur avec la nouvelle vitesse
    fn new_random_speed() -> Vec2 {
        let mut rng = thread_rng();
        Vec2::from_angle(rng.gen_range(0.0..=2.0 * PI))
    }

    /// Fonction qui permet d'empêcher les asteroids de sortir de l'écran
    /// # Arguments
    /// - `Vec2`: Vecteur avec la position de l'asteroid
    /// # Returns:
    /// - `Vec2`: Renvoie le vecteur avec les nouvelles coordonnées pour l'asteroid
    fn wrap_around_screen(position: Vec2) -> Vec2 {
        vec2(
            (position.x + screen_width()) % screen_width(),
            (position.y + screen_height()) % screen_height(),
        )
    }
}

use crate::stellarobject::StellarObject;

impl StellarObject for Asteroid {
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
        self.position += self.speed;
        self.position = Self::wrap_around_screen(self.position);
    }

    /// Retourne le rayon de l'objet.
    /// # Arguments
    /// - `&self`: une instance de l'objet stellaire
    /// # Returns
    /// - `f32`: le rayon de l'objet stellaire
    fn radius(&self) -> f32 {
        self.get_size().scale() / 2.0
    }

    /// Gere la collision avec un autre objet.
    /// # Arguments
    /// - `&mut self`: une instance de l'objet stellaire
    fn handle_collision(&mut self) {
        self.active = false
    }
}
