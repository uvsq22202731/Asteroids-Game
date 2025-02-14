//! Ce module contient les fonctionnalités principales pour gérer notre jeu

use asteroid::Asteroid;
use black_hole::BlackHole;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;
use missile::Missile;
use spaceship::Spaceship;
use stellarobject::StellarObject;

mod asteroid;
mod black_hole;
mod missile;
mod spaceship;
mod stellarobject;

/// Énumération représentant les différents états du jeu.
enum GameState {
    StartScreen,
    Playing,
    GameOver,
}
/// Structure permettant de gérer les textes temporaires affichés à l'écran
/// # Champs
/// - `text`: le texte lui même
/// - `position`: position du texte
/// - `color`: couleur du texte à l'écran
/// - `lifetime`: durée de vie du texte à l'écran
struct TemporaryText {
    text: String,
    position: Vec2,
    color: Color,
    lifetime: f32, // Temps restant avant disparition
}

/// Charge les différents sons
/// # Arguments
/// - `Sound`: Référence à chaque son.
/// # Returns
/// - `asteroid_destroyed`, son pour l'asteroids détruit
/// - `missile_sound`, son quand on se fait touché
/// - `shield_lost`, son quand on perd le shield
/// - `start_game`, son quand on lance la partie
/// - `game_over`, son quand on perd la partie
async fn load_sounds() -> (Sound, Sound, Sound, Sound, Sound, Sound) {
    let asteroid_destroyed = load_sound("assets/audio/asteroid_destroyed.wav")
        .await
        .unwrap();
    let shield_lost = load_sound("assets/audio/shield_lost.wav").await.unwrap();
    let missile_sound = load_sound("assets/audio/missile_sound.wav").await.unwrap();
    let start_game = load_sound("assets/audio/start_game.wav").await.unwrap();
    let game_over = load_sound("assets/audio/game_over.wav").await.unwrap();
    let new_wave = load_sound("assets/audio/new_wave.wav").await.unwrap();

    (
        asteroid_destroyed,
        shield_lost,
        missile_sound,
        start_game,
        game_over,
        new_wave,
    )
}

/// Charge une texture d'arrière-plan du jeu.
/// # Returns
/// - `Texture2D` : Texture d'arrière-plan chargée.
/// # Panics
/// Panique si la texture ne peut pas être chargée.
async fn load_background_texture() -> Texture2D {
    let texture = load_texture("assets/background.png").await;
    match texture {
        Ok(tex) => tex,
        Err(err) => {
            eprintln!("Erreur lors du chargement de la texture : {:?}", err);
            panic!("Échec du chargement de la texture");
        }
    }
}

/// Charge une texture d'arrière-plan pour l'écran de démarage.
/// # Returns
/// - `Texture2D` : Texture d'arrière-plan chargée.
/// # Panics
/// Panique si la texture ne peut pas être chargée.
async fn load_background_texture_start() -> Texture2D {
    let texture = load_texture("assets/background_start.png").await;
    match texture {
        Ok(tex) => tex,
        Err(err) => {
            eprintln!("Erreur lors du chargement de la texture : {:?}", err);
            panic!("Échec du chargement de la texture");
        }
    }
}

/// Charge une texture d'arrière-plan pour l'écran de fin.
/// # Returns
/// - `Texture2D` : Texture d'arrière-plan chargée.
/// # Panics
/// Panique si la texture ne peut pas être chargée.
async fn load_background_texture_dead() -> Texture2D {
    let texture = load_texture("assets/background_dead.png").await;
    match texture {
        Ok(tex) => tex,
        Err(err) => {
            eprintln!("Erreur lors du chargement de la texture : {:?}", err);
            panic!("Échec du chargement de la texture");
        }
    }
}

/// Dessine une texture en tant qu'arrière-plan.
/// # Arguments
/// - `background_texture`: Référence à une texture d'arrière-plan à dessiner.
fn draw_background(background_texture: &Texture2D) {
    // effacer l'arriere plan avec une couleur unie
    clear_background(BLACK);

    // met à l'échelle par rapport à la taille de l'écran
    let scale_x = screen_width() / background_texture.width();
    let scale_y = screen_height() / background_texture.height();

    let scale = scale_x.max(scale_y);

    // calculer la largeur et la hauteur de l'image redimensionnée
    let width = background_texture.width() * scale;
    let height = background_texture.height() * scale;

    // dessiner la texture redimensionnee pour qu'elle couvre l'ecran
    draw_texture_ex(
        background_texture, // Texture d'arrière-plan
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(width, height)), // Taille de destination
            ..Default::default()
        },
    );
}

///Fonction qui dessine les asteroides
/// # Arguments
/// - `asteroids`: contient tous les asteroides du jeu
fn draw_asteroids(asteroids: &[Asteroid]) {
    for asteroid in asteroids {
        asteroid.draw();
    }
}

///Fonction qui gère le dessin des différents objets sur l'écran de jeu
/// # Arguments
/// - `spaceship`: contient l'objet vaisseau
/// - `asteroids`: contient tous les objets Asteroids du jeu
/// - `missiles`: contient tous les objets Missile du jeu
/// - `wave`: correspond au numéro de vague
/// - `score`: contient le score actuel du joueur
/// - `temporary_texts`: contient tous nos textes temporaires
/// - `black_holes`: contient tous nos trous noirs
fn draw(
    spaceship: &Spaceship,
    asteroids: &[Asteroid],
    missiles: &[Missile],
    black_hole: &[BlackHole],
    wave: u32,
    score: i32,
    temporary_texts: &[TemporaryText],
) {
    spaceship.draw();
    draw_asteroids(asteroids);
    for missile in missiles {
        missile.draw();
    }

    for blackhole in black_hole {
        blackhole.draw();
    }

    // Affichage du texte avec le numéro de vague
    draw_text(&format!("Vague: {}", wave), 10.0, 20.0, 30.0, WHITE);

    // Affichage du texte pour le score
    draw_text(&format!("Score: {}", score), 10.0, 50.0, 30.0, WHITE);

    // Affichage du message "Touché!" au centre de l'écran
    if spaceship.hit {
        let text = "Touché!";
        let text_size = 30.0;
        let text_width = measure_text(text, None, text_size as u16, 1.0).width;
        let x = (screen_width() - text_width) / 2.0;
        let y = screen_height() / 2.0;
        draw_text(text, x, y, text_size, RED);
    }

    draw_temporary_texts(temporary_texts);
}

///Fonction qui gère l'entrée des touches par le joueur
/// # Returns
/// - `bool`: Retourne `true` si la touche escape est appuyé sinon `false`.
fn handle_input(
    spaceship: &mut Spaceship,
    missiles: &mut Vec<Missile>,
    missile_sound: &Sound,
) -> bool {
    if is_key_down(KeyCode::Escape) {
        return true;
    }

    if is_key_down(KeyCode::Right) {
        spaceship.rotate(0.05);
    }
    if is_key_down(KeyCode::Left) {
        spaceship.rotate(-0.05);
    }
    if is_key_down(KeyCode::Up) {
        spaceship.apply_thrust(0.01);
    }
    if is_key_down(KeyCode::Down) {
        spaceship.apply_thrust(-0.01);
    }
    if !is_key_down(KeyCode::Up) && !is_key_down(KeyCode::Down) {
        // Ralentir progressivement
        if spaceship.velocity.length() > 0.0 {
            let direction = spaceship.velocity.normalize();
            spaceship.velocity -= direction * 0.005;
        }
    }
    if is_key_pressed(KeyCode::Space) {
        let missile = Missile::new(spaceship.get_pos(), spaceship.rotation);
        play_sound(
            missile_sound,
            PlaySoundParams {
                looped: false,
                volume: 0.5,
            },
        );
        missiles.push(missile);
    }

    false
}

///Fonction qui met à jour le mouvement des différents objets
/// # Arguments
/// - `spaceship`: contient une instance du vaisseau
/// - `asteroids`: contient une instance de tous les asteroids du jeu
/// - `missiles`: contient une instance de tous les missiles du jeu
/// - `black_holes`: contient tous nos trous noirs
fn update_model(
    spaceship: &mut Spaceship,
    asteroids: &mut Vec<Asteroid>,
    missiles: &mut Vec<Missile>,
    black_holes: &mut Vec<BlackHole>,
) {
    for asteroid in asteroids.iter_mut() {
        asteroid.move_obj(); // Utilisation trait
    }
    asteroids.retain(|a| a.active);

    black_holes.retain(|b| b.active);

    spaceship.move_obj(); // Utilisation trait

    for missile in missiles.iter_mut() {
        missile.move_obj(); // Utilisation trait
    }
    missiles.retain(|m| m.active);
}

///Fonction qui gère la collision entre deux objets
/// # Arguments
/// - `obj1`: contient un objet stellaire
/// - `obj2`: contient un autre objet stellaire
/// # Returns
/// - `bool`: Retourne `true` si il y a une collision sinon `false`.
pub fn check_collision_between(obj1: &mut dyn StellarObject, obj2: &mut dyn StellarObject) -> bool {
    let distance = obj1.get_pos().distance(obj2.get_pos());
    let collision_distance = obj1.radius() + obj2.radius();

    if distance < collision_distance {
        obj1.handle_collision();
        obj2.handle_collision();
        return true;
    }
    false
}

/// Fonction qui gère toutes les collissions qui peuvent se produire dans le jeu.
/// # Arguments
/// - `spaceship`: contient notre asteroid avec ses propriétés
/// - `asteroids`: contient tous nos asteroid
/// - `missiles`: contient tous nos missiles
/// - `black_holes`: contient tous les trous noirs
/// - `score`: contient le score actuel du joueur
/// - `shield_lost`: son quand on perd notre bouclier
/// - `asteroid_destroyed`: son quand on détruit un asteroid
/// - `temporary_texts`: contient tous nos textes temporaires pour afficher le score
/// # Returns
/// - `bool`: Retourne `true` si il y a une collision sinon `false`.
async fn check_collision(
    spaceship: &mut Spaceship,
    asteroids: &mut Vec<Asteroid>,
    missiles: &mut [Missile],
    black_holes: &mut Vec<BlackHole>,
    score: &mut i32,
    shield_lost: &Sound,
    asteroid_destroyed: &Sound,
    temporary_texts: &mut Vec<TemporaryText>,
) -> bool {
    if spaceship.invincible {
        return false;
    }

    let mut asteroids_to_split = Vec::new();

    // Collision entre Asteroids et  SpaceShip
    for asteroid in asteroids.iter_mut() {
        if check_collision_between(asteroid, spaceship) {
            if spaceship.active {
                play_sound(
                    shield_lost,
                    PlaySoundParams {
                        looped: false,
                        volume: 1.5,
                    },
                );
                *score -= 5;
                temporary_texts.push(TemporaryText {
                    text: "-5".to_string(),
                    position: spaceship.get_pos() + Vec2::new(20.0, 20.0),
                    color: RED,
                    lifetime: 1.0,
                });

                black_holes
                    .push(BlackHole::new(asteroid.get_pos(), asteroid.get_size().scale()).await);

                return false;
            } else {
                return true;
            }
        }
    }
    // Collision entre Trou Noir et Spaceship
    for black_hole in black_holes.iter_mut() {
        if check_collision_between(black_hole, spaceship) {
            return true;
        }
    }
    // Collision entre Asteroids et Trou Noir
    for asteroid in asteroids.iter_mut() {
        for black_hole in black_holes.iter_mut() {
            if check_collision_between(asteroid, black_hole) {
                play_sound(
                    asteroid_destroyed,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.7,
                    },
                );
            }
        }
    }
    // Collision entre Trou Noir et missiles
    for black_hole in black_holes.iter_mut() {
        for j in (0..missiles.len()).rev() {
            let missile = &mut missiles[j];
            if check_collision_between(missile, black_hole) && !black_hole.active {
                temporary_texts.push(TemporaryText {
                    text: "+10".to_string(),
                    position: black_hole.get_pos() + Vec2::new(20.0, 20.0),
                    color: GREEN,
                    lifetime: 0.4,
                });
                *score += 10;
            }
        }
    }
    // Collision entre Asteroids et missiles
    for asteroid in asteroids.iter_mut() {
        for j in (0..missiles.len()).rev() {
            let missile = &mut missiles[j];
            if check_collision_between(asteroid, missile) {
                play_sound(
                    asteroid_destroyed,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.7,
                    },
                );
                temporary_texts.push(TemporaryText {
                    text: "+1".to_string(),
                    position: asteroid.get_pos() + Vec2::new(20.0, 20.0),
                    color: GREEN,
                    lifetime: 0.4,
                });
                *score += 1;

                if let Some((child1, child2)) = asteroid.split() {
                    asteroids_to_split.push(child1);
                    asteroids_to_split.push(child2);
                }
                break;
            }
        }
    }

    // Ajouter les nouveaux astéroïdes qui sortent d'un split avec missile à la liste asteroids
    asteroids.extend(asteroids_to_split);

    false
}

/// Gère le centrage du texte en fonction de la taille de la fenêtre
/// # Arguments
/// - `text`: le texte.
/// - `y`: la hauteur.
/// - `font_size`: la taille de la police.
/// - `color`: la couleur du texte.
fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
    let text_width = measure_text(text, None, font_size as u16, 1.0).width;
    let x = (screen_width() - text_width) / 2.0;
    draw_text(text, x, y, font_size, color);
}

/// Gère l'affichage de l'écran de démarrage.
/// # Arguments
/// - `background_texture_start`: Texture d'arrière-plan pour l'écran de démarrage.
/// # Returns
/// - `bool`: Retourne `true` si l'utilisateur commence la partie, sinon `false`.
async fn draw_start_screen(background_texture_start: &Texture2D) -> bool {
    draw_background(background_texture_start);

    let button_width = 200.0;
    let button_height = 50.0;
    let center_x = (screen_width() - button_width) / 2.0;
    let center_y = (screen_height() - button_height) / 2.0;

    draw_centered_text("Asteroids Game", center_y - 150.0, 40.0, WHITE);

    let play_button = Rect::new(center_x, center_y - 50.0, button_width, button_height);
    let quit_button = Rect::new(center_x, center_y + 50.0, button_width, button_height);

    draw_rectangle(
        play_button.x,
        play_button.y,
        play_button.w,
        play_button.h,
        GREEN,
    );
    draw_centered_text("Jouer", play_button.y + 35.0, 30.0, WHITE);
    draw_rectangle(
        quit_button.x,
        quit_button.y,
        quit_button.w,
        quit_button.h,
        RED,
    );
    draw_centered_text("Quitter", quit_button.y + 35.0, 30.0, WHITE);

    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position().into();
        if play_button.contains(mouse_pos) {
            return true; // Start the game
        } else if quit_button.contains(mouse_pos) {
            std::process::exit(0); // Quit the game
        }
    }

    false
}

/// Gère l'affichage de l'écran de fin.
/// # Arguments
/// - `background_texture_start`: Texture d'arrière-plan pour l'écran de démarrage.
/// # Returns
/// - `bool`: Retourne `true` si l'utilisateur relance la partie, sinon `false`.
async fn draw_game_over_screen(background_texture_dead: &Texture2D) -> bool {
    draw_background(background_texture_dead);

    let button_width = 200.0;
    let button_height = 50.0;
    let center_x = (screen_width() - button_width) / 2.0;
    let center_y = (screen_height() - button_height) / 2.0;

    draw_centered_text("Game Over", center_y - 150.0, 40.0, WHITE);

    let replay_button = Rect::new(center_x, center_y - 50.0, button_width, button_height);
    let quit_button = Rect::new(center_x, center_y + 50.0, button_width, button_height);

    draw_rectangle(
        replay_button.x,
        replay_button.y,
        replay_button.w,
        replay_button.h,
        GREEN,
    );
    draw_centered_text("Rejouer", replay_button.y + 35.0, 30.0, WHITE);
    draw_rectangle(
        quit_button.x,
        quit_button.y,
        quit_button.w,
        quit_button.h,
        RED,
    );
    draw_centered_text("Quitter", quit_button.y + 35.0, 30.0, WHITE);

    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position().into();
        if replay_button.contains(mouse_pos) {
            return true; // Restart the game
        } else if quit_button.contains(mouse_pos) {
            std::process::exit(0); // Quit the game
        }
    }

    false
}

/// Lance une nouvelle vague d'astéroïdes.
/// # Arguments
/// - `asteroids`: Vecteur mutable contenant les astéroïdes.
/// - `wave`: Numéro de la vague actuelle.
async fn start_new_wave(asteroids: &mut Vec<Asteroid>, wave: u32) {
    let num_asteroids = 5 + (wave - 1);
    for _ in 0..num_asteroids {
        asteroids.push(Asteroid::new().await);
    }
}

/// Fonction qui dessine les textes temporaires
/// Ici le score ajouté
/// # Arguments
/// - `temporary_texts`: contient tous nos textes à affichier
fn draw_temporary_texts(temporary_texts: &[TemporaryText]) {
    for temp_text in temporary_texts {
        draw_text(
            &temp_text.text,
            temp_text.position.x,
            temp_text.position.y,
            20.0, // Taille de la police
            temp_text.color,
        );
    }
}

/// Fonction qui met à jour nos textes temporaires,
/// càd les fait disparaitre au bout d'un temps donné
/// # Arguments
/// - `temporary_texts`: contient tous nos textes temporaires
fn update_temporary_texts(temporary_texts: &mut Vec<TemporaryText>) {
    for text in temporary_texts.iter_mut() {
        text.lifetime -= get_frame_time();
    }
    temporary_texts.retain(|text| text.lifetime > 0.0);
}

#[macroquad::main("Spaceship and Asteroids")]
async fn main() {
    let (asteroid_destroyed, shield_lost, missile_sound, start_game, game_over, new_wave) =
        load_sounds().await;
    let mut start_game_sound: bool = false;
    let mut end_game_sound: bool = false;
    let background_texture = load_background_texture().await;
    let background_texture_start = load_background_texture_start().await;
    let background_texture_dead = load_background_texture_dead().await;
    let mut temporary_texts: Vec<TemporaryText> = Vec::new();
    let mut game_state = GameState::StartScreen;
    let mut spaceship = Spaceship::new().await;
    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut missiles: Vec<Missile> = Vec::new();
    let mut black_holes: Vec<BlackHole> = Vec::new();
    let mut wave = 1;
    let mut score: i32 = 0;

    start_new_wave(&mut asteroids, wave).await;

    loop {
        match game_state {
            GameState::StartScreen => {
                if draw_start_screen(&background_texture_start).await {
                    game_state = GameState::Playing;
                }
            }
            GameState::Playing => {
                if !start_game_sound {
                    play_sound(
                        &start_game,
                        PlaySoundParams {
                            looped: false,
                            volume: 1.0,
                        },
                    );
                    start_game_sound = true; // Le son est joué une seule fois
                }
                draw_background(&background_texture);
                draw(
                    &spaceship,
                    &asteroids,
                    &missiles,
                    &black_holes,
                    wave,
                    score,
                    &temporary_texts,
                );

                if handle_input(&mut spaceship, &mut missiles, &missile_sound) {
                    break;
                }

                if check_collision(
                    &mut spaceship,
                    &mut asteroids,
                    &mut missiles,
                    &mut black_holes,
                    &mut score,
                    &shield_lost,
                    &asteroid_destroyed,
                    &mut temporary_texts,
                )
                .await
                {
                    play_sound(
                        &missile_sound,
                        PlaySoundParams {
                            looped: false,
                            volume: 1.0,
                        },
                    );
                    game_state = GameState::GameOver;
                }

                if asteroids.is_empty() {
                    temporary_texts.push(TemporaryText {
                        text: "+10".to_string(),
                        position: spaceship.get_pos() + Vec2::new(20.0, 20.0),
                        color: GOLD,
                        lifetime: 1.0,
                    });

                    score += 10;
                    wave += 1;
                    spaceship.shield = true;
                    spaceship.invincible = true;
                    spaceship.hit = false;
                    spaceship.invincibility_timer = 1.0;
                    play_sound(
                        &new_wave,
                        PlaySoundParams {
                            looped: false,
                            volume: 1.0,
                        },
                    );
                    start_new_wave(&mut asteroids, wave).await;
                }

                update_model(
                    &mut spaceship,
                    &mut asteroids,
                    &mut missiles,
                    &mut black_holes,
                );

                update_temporary_texts(&mut temporary_texts);

                if spaceship.invincible {
                    spaceship.invincibility_timer -= get_frame_time();
                    if spaceship.invincibility_timer <= 0.0 {
                        spaceship.invincible = false;
                        spaceship.hit = false;
                    }
                }
            }
            GameState::GameOver => {
                if !end_game_sound {
                    play_sound(
                        &game_over,
                        PlaySoundParams {
                            looped: false,
                            volume: 1.0,
                        },
                    );
                    end_game_sound = true;
                }
                if draw_game_over_screen(&background_texture_dead).await {
                    start_game_sound = false;
                    game_state = GameState::Playing;
                    if !start_game_sound {
                        play_sound(
                            &start_game,
                            PlaySoundParams {
                                looped: false,
                                volume: 1.0,
                            },
                        );
                        start_game_sound = true; // Le son est joué une seule fois
                    }
                    spaceship = Spaceship::new().await;
                    asteroids.clear();
                    missiles.clear();
                    black_holes.clear();
                    wave = 1;
                    score = 0;
                    start_new_wave(&mut asteroids, wave).await;
                }
            }
        }

        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Structure représentant un objet avec une position, un rayon, et un état actif/inactif.
    struct Object {
        position: Vec2,
        radius: f32,
        pub active: bool,
    }

    impl Object {
        /// Crée une nouvelle instance d'`Object`.
        ///
        /// # Arguments
        /// - `position`: Position initiale de l'objet.
        /// - `radius`: Rayon de l'objet.
        fn new(position: Vec2, radius: f32) -> Self {
            Self {
                position,
                active: true,
                radius,
            }
        }
    }

    impl StellarObject for Object {
        fn get_pos(&self) -> Vec2 {
            self.position
        }

        fn move_obj(&mut self) {
            self.position.x += 10.0;
            self.position.y += 10.0;
        }

        fn radius(&self) -> f32 {
            self.radius
        }

        fn handle_collision(&mut self) {
            self.active = false;
        }
    }

    /// Vérifie que la fonction `check_collision_between` détecte correctement une collision entre deux objets.
    ///
    /// # Contexte
    /// - `obj1` est initialisé à `(50.0, 100.0)` avec un rayon de `20.0`.
    /// - `obj2` est initialisé à `(20.0, 70.0)` avec un rayon de `30.0`.
    ///
    /// # Comportement attendu
    /// La collision doit être détectée si la distance entre les deux objets est inférieure ou égale à la somme de leurs rayons.
    ///
    /// # Panique
    /// Le test échoue avec le message `"Il n'y a pas de collision !"` si aucune collision n'est détectée.
    #[test]
    fn test_check_collision() {
        let mut obj1 = Object::new(vec2(50.0, 100.0), 20.0);
        let mut obj2 = Object::new(vec2(20.0, 70.0), 30.0);

        let expected_collision = check_collision_between(&mut obj1, &mut obj2);

        assert!(expected_collision, "Il n'y a pas de collision !");
    }

    /// Vérifie que la méthode `get_pos` retourne correctement la position initiale de l'objet.
    ///
    /// # Contexte
    /// - L'objet est initialisé avec la position `(50.0, 100.0)`.
    ///
    /// # Comportement attendu
    /// La méthode `get_pos` doit retourner exactement la position initiale.
    ///
    /// # Panique
    /// Le test échoue avec le message `"La position retournée par get_pos() est incorrecte !"`
    /// si la position retournée diffère de la position attendue.
    #[test]
    fn test_get_position() {
        let obj = Object::new(vec2(50.0, 100.0), 30.0);

        let expected_position = vec2(50.0, 100.0);

        assert_eq!(
            obj.get_pos(),
            expected_position,
            "La position retournée par get_pos() est incorrecte !"
        );
    }

    /// Vérifie que la méthode `move_obj` déplace correctement l'objet.
    ///
    /// # Contexte
    /// - L'objet est initialisé avec la position `(50.0, 100.0)`.
    /// - La méthode `move_obj` doit ajouter `10.0` aux coordonnées **x** et **y**.
    ///
    /// # Comportement attendu
    /// Après l'appel à `move_obj`, la nouvelle position doit être `(60.0, 110.0)`.
    ///
    /// # Panique
    /// Le test échoue avec le message `"L'objet n'a pas été déplacé correctement !"`
    /// si la nouvelle position ne correspond pas à la position attendue.
    #[test]
    fn test_move_object() {
        let mut obj = Object::new(vec2(50.0, 100.0), 30.0);

        let initial_position = obj.get_pos();

        obj.move_obj();

        let new_position = obj.get_pos();

        assert_eq!(
            new_position,
            vec2(initial_position.x + 10.0, initial_position.y + 10.0),
            "L'objet n'a pas été déplacé correctement !"
        );
    }
}
